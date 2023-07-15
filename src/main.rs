use std::path::{Path, PathBuf};

use clap::Parser;
use git2::{Blame, Repository, TreeWalkMode, TreeWalkResult};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "PATH")]
    repo: Option<PathBuf>,
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

    tree.walk(TreeWalkMode::PreOrder, |path: &str, entry| {
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
                if line.contains("TODO") {
                    lines.push((lineno, line));
                }
            }
            if lines.len() == 0 {
                return TreeWalkResult::Ok;
            }
            let filepath = path.to_owned() + entry.name().unwrap();
            let blame = repo.blame_file(Path::new(&filepath), None).unwrap();
            for (lineno, line) in lines.iter() {
                let blame_line = blame.get_line(*lineno).unwrap();
                println!("line {} got {}", line, blame_line.final_commit_id());
            }
        }
        TreeWalkResult::Ok
    })
    .unwrap();
}
