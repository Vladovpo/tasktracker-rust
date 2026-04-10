Task Tracker in Rust: Pet Project 🦀

A small CLI tool I’m building while learning Rust — simple, evolving, and intentionally a bit experimental.

> Warning: This is a Linux-only tool and requires having '/home/$USER/.local/share/' parent folders

Main Features ✨
📋 List tasks: See what’s done and what’s not.
➕ Add tasks: Quickly throw new things in.
✅ Complete tasks: Mark stuff as finished.
🗑️ Delete tasks: Remove what you don’t need anymore.
💾 Persistent storage: Everything lives in a JSON file.
How to Use 🚀

Run directly with Cargo while developing:

cargo run -- list
cargo run -- add --name "sample" --desc "demo task"
cargo run -- complete --name "sample"
cargo run -- delete --name "sample"

Or build and run the binary:

cargo build
./target/debug/task-tracker list
Installation ⚙️

Build a release version:

cargo build --release

Install it globally (so you can call it from anywhere):

cargo install --path .

Don’t forget to have Cargo’s bin directory in your PATH
(.bashrc, .zshrc, or config.fish).

Storage 💾

Tasks are stored locally in:

~/.local/share/task-tracker/tasks.json
If the file doesn’t exist → it gets created automatically
If it’s empty → you just start fresh
Development 🛠️

Typical workflow:

cargo check
cargo test
cargo run -- list

Nothing fancy — just iterating and improving step by step.

Future Plans (unordered chaos) 🧠
Better CLI output (less raw, more readable)
More task metadata (priority, maybe dates)
Cleaner architecture as I learn more
Possibly encryption or other experiments

Note:
This is a learning project, not a finished product.
The goal is to gradually improve structure, design, and Rust fluency over time — not to build the perfect task tracker immediately 🚧