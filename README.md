# Task Tracker in Rust

Small CLI pet project for learning Rust while building a simple task tracker.

## Current Features

- List tasks
- Add tasks
- Complete tasks
- Delete tasks
- Store tasks in JSON

## Development

Use Cargo for the normal development loop:

```bash
cargo check
cargo test
cargo run -- list
cargo run -- add --name "sample" --desc "demo task"
```

If you want to run the compiled debug binary directly:

```bash
cargo build
./target/debug/task-tracker list
./target/debug/task-tracker complete --name "sample"
```

## CLI Commands

```bash
task-tracker list
task-tracker add --name "sample" --desc "demo task"
task-tracker complete --name "sample"
task-tracker delete --name "sample"
```

## Installation

Build a release binary:

```bash
cargo build --release
```

Install the binary locally:

```bash
cargo install --path .
```

## Storage

This version uses a Linux-only JSON storage path:

```text
~/.local/share/task-tracker/tasks.json
```

If the file does not exist yet, the app starts with an empty task list.

## Notes

This is still a work-in-progress pet project. The goal is to keep it small, readable, and useful for learning Rust concepts over time.
