use std::fs;

use crate::models::task::{Tasks};

pub fn load_from_storage() -> Tasks {
    let raw = fs::read_to_string("tasks.json")
        .unwrap_or_else(|_| "{}".to_string());

    let tasks: Tasks = serde_json::from_str(&raw)
        .expect("Failed to parse tasks.json");

    tasks
}

pub fn save_to_storage(tasks: &Tasks){
    let data = serde_json::to_string_pretty(tasks)
        .expect("Failed to serialize");

    fs::write("tasks.json", data)
        .expect("Failed to write to file");
}