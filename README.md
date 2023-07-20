# git-todo-collector
collect todo or others from a git  code repo

## Install

```bash
cargo install git-todo-collector
```

## Run

```bash
# find in current, output as table
git-todo-collector
# output table with other rpeo
git-todo-collector -r ./some-git-repo-path
# json
git-todo-collector -r ./some-git-repo-path --format json
# html
git-todo-collector -r ./some-git-repo-path --format html > git-todos.html
```

### work with jq
```bash
git-todo-collector --format json | jq ''
```

the output

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