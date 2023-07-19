use chrono::{DateTime, FixedOffset, NaiveDateTime};
use clap::Parser;
use git2::{Repository, TreeWalkMode, TreeWalkResult};
use serde_derive::{Deserialize, Serialize};
use serde_json::to_string;
use std::path::{Path, PathBuf};
use table_to_html::HtmlTable;
use tabled::{Table, Tabled};

static /*🤔*/ TODO_PREFIX: &str = "TODO:";
static COMMIT_HEAD_SIZE: usize = 7;
static LINE_LENGTH: usize = 12;

const FORMAT_TABLE: &str = "table";
const FORMAT_JSON: &str = "json";
const FORMAT_CSV: &str = "csv";
const FORMAT_HTML: &str = "html";

const WITH_LINE_IGNORE: &str = "ignore";
const WITH_LINE_EXCERPT: &str = "excerpt";
const WITH_LINE_FULL: &str = "full";

#[derive(Parser)]
#[command(author = "alingse", version="0.1.0", about, long_about = None)]
struct Cli {
    /// the git repo
    #[arg(short, long)]
    repo: Option<PathBuf>,

    /// output format
    #[arg(short, long, value_parser([FORMAT_TABLE, FORMAT_JSON, FORMAT_CSV, FORMAT_HTML]), default_value=FORMAT_TABLE)]
    format: Option<String>,

    /// with the line content
    #[arg(long, value_parser([WITH_LINE_IGNORE, WITH_LINE_EXCERPT, WITH_LINE_FULL]), default_value=WITH_LINE_IGNORE)]
    with_line: Option<String>,
}

#[derive(Serialize, Deserialize, Tabled)]
struct TODO {
    path: String,
    lineno: usize,
    commit: String,
    author: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    line: String,
    datetime: String,
}

fn main() {
    let cli: Cli = Cli::parse();
    let with_line: String = cli.with_line.unwrap();
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
                let mut todo = TODO {
                    path: path.clone(),
                    lineno: *lineno,
                    line: "".to_string(),
                    commit: commit_id.to_string().as_str()[0..COMMIT_HEAD_SIZE].to_string(),
                    author: commit.author().to_string(),
                    datetime: format!("{}", gittime_to_datetime(commit.time())),
                };
                //
                match with_line.as_str() {
                    WITH_LINE_EXCERPT => {
                        todo.line = excerpt_line(line.trim(), LINE_LENGTH).to_string();
                    }
                    WITH_LINE_FULL => {
                        todo.line = line.trim().to_string();
                    }
                    _ => {}
                }
                todos.push(todo);
            }
        }
        TreeWalkResult::Ok
    })
    .unwrap();

    // do format
    let format = cli.format.unwrap();
    match format.as_str() {
        FORMAT_JSON => {
            for todo in todos {
                let json = to_string(&todo).unwrap();
                println!("{}", json);
            }
        }
        FORMAT_CSV => {}
        FORMAT_TABLE => {
            let table = Table::new(todos);
            println!("{}", table.to_string());
        }
        FORMAT_HTML => {
            let mut html_table = HtmlTable::from(Table::builder(&todos));
            html_table.set_border(1);
            println!("{}", html_table.to_string());
        }
        _ => {}
    }
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

fn excerpt_line(line: &str, length: usize) -> &str {
    let position = line.char_indices().nth(length).unwrap().0;
    return &line[..position];
}
