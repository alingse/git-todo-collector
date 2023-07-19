use chrono::{DateTime, FixedOffset, NaiveDateTime};
use clap::Parser;
use git2::{Repository, TreeWalkMode, TreeWalkResult};
use serde_derive::{Deserialize, Serialize};
use serde_json::to_string;
use std::path::{Path, PathBuf};
use tabled::{Table, Tabled};

static /*🤔*/ TODO_PREFIX: &str = "TODO:";
static COMMIT_HEAD_SIZE: usize = 7;
static LINE_LENGTH: usize = 15;
#[derive(Parser)]
#[command(author = "alingse", version="0.1.0", about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "PATH")]
    repo: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Tabled)]
struct TODO {
    path: String,
    lineno: usize,
    commit_id: String,
    author: String,
    line: String,
    datetime: String,
}

fn main() {
    let cli = Cli::parse();
    let mut path: PathBuf = PathBuf::from(".");
    if let Some(p) = cli.repo {
        path = p;
    }

    let repo: Repository = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let head = repo.head().unwrap();
    let target = head.target().unwrap();
    let commit = repo.find_commit(target).unwrap();
    let tree = repo.find_tree(commit.tree_id()).unwrap();

    let mut todos: Vec<TODO> = Vec::new();
    tree.walk(TreeWalkMode::PreOrder, |dir: &str, entry| {
        if entry.kind() == Some(git2::ObjectType::Blob) {
            let obj = entry.to_object(&repo).unwrap();
            let blob = obj.as_blob().unwrap();
            if blob.is_binary() {
                return TreeWalkResult::Ok;
            }
            // TODO
            let content = std::str::from_utf8(blob.content()).unwrap();
            let mut lines: Vec<(usize, &str)> = Vec::new();
            for (lineno, line) in content.lines().into_iter().enumerate() {
                if line.contains(TODO_PREFIX) {
                    lines.push((lineno + 1, line));
                }
            }
            if lines.len() == 0 {
                return TreeWalkResult::Ok;
            }

            let path = dir.to_owned() + entry.name().unwrap();
            let blame = repo.blame_file(Path::new(&path), None).unwrap();
            for (lineno, line) in lines.iter() {
                let blame_line = blame.get_line(*lineno).unwrap();
                let commit_id = blame_line.final_commit_id();
                let commit = repo.find_commit(commit_id).unwrap();
                // build TODO
                let todo = TODO {
                    path: path.clone(),
                    lineno: *lineno,
                    line: line.trim()[..LINE_LENGTH].to_string(),
                    commit_id: commit_id.to_string().as_str()[0..COMMIT_HEAD_SIZE].to_string(),
                    author: commit.author().to_string(),
                    datetime: format!("{}", gittime_to_datetime(commit.time())),
                };
                todos.push(todo);
            }
        }
        TreeWalkResult::Ok
    })
    .unwrap();

    let table = Table::new(todos).to_string();
    println!("{}", table);
    /*
    for todo in todos {
        let json = to_string(&todo).unwrap();
        println!("{}", json);
    }
     */
}

fn gittime_to_datetime(t: git2::Time) -> DateTime<FixedOffset> {
    let secs: i64 = t.seconds();
    let sign: char = t.sign();
    let offset: i32 = t.offset_minutes() * 60;

    let timezone: FixedOffset;
    if sign == '+' {
        timezone = FixedOffset::east_opt(offset).unwrap();
    } else {
        timezone = FixedOffset::west_opt(offset).unwrap();
    };

    let naive = NaiveDateTime::from_timestamp_opt(secs, 0).unwrap();
    let dt: DateTime<FixedOffset> = DateTime::<FixedOffset>::from_utc(naive, timezone);
    return dt;
}
