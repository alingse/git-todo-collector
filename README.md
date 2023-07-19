# git-todo-collector
collect todo or others from a git  code repo

## Install

```bash
git clone https://github.com/alingse/git-todo-collector
cd git-todo-collector
cargo build
```

## Run

```bash
# table
./target/debug/git-todo-collector
# table with other rpeo
./target/debug/git-todo-collector -r ../../xx
# json
./target/debug/git-todo-collector --format json
```

work with jq
```bash
./target/debug/git-todo-collector --format json | jq ''
```

```json
{
  "path": "src/main.rs",
  "lineno": 10,
  "commit": "36adbd3",
  "author": "alingse <alingse@foxmail.com>",
  "datetime": "2023-07-20 00:50:44 +08:00"
}
{
  "path": "src/main.rs",
  "lineno": 11,
  "commit": "36adbd3",
  "author": "alingse <alingse@foxmail.com>",
  "datetime": "2023-07-20 00:50:44 +08:00"
}
{
  "path": "src/main.rs",
  "lineno": 12,
  "commit": "36adbd3",
  "author": "alingse <alingse@foxmail.com>",
  "datetime": "2023-07-20 00:50:44 +08:00"
}
```