use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::models::task::Tasks;

const APP_DIR_NAME: &str = "task-tracker";
const STORAGE_FILE_NAME: &str = "tasks.json";

fn storage_file_path() -> PathBuf {
    let home = env::var_os("HOME").expect("HOME is not set");

    PathBuf::from(home)
        .join(".local")
        .join("share")
        .join(APP_DIR_NAME)
        .join(STORAGE_FILE_NAME)
}

fn load_from_path(path: &Path) -> Tasks {
    match fs::read_to_string(path) {
        Ok(raw) => serde_json::from_str(&raw)
            .unwrap_or_else(|error| panic!("Failed to parse {}: {error}", path.display())),
        Err(error) if error.kind() == ErrorKind::NotFound => Tasks::default(),
        Err(error) => panic!("Failed to read {}: {error}", path.display()),
    }
}

pub fn load_from_storage() -> Tasks {
    load_from_path(&storage_file_path())
}

fn save_to_path(path: &Path, tasks: &Tasks) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("Failed to create {}: {error}", parent.display()));
    }

    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize");

    fs::write(path, data)
        .unwrap_or_else(|error| panic!("Failed to write {}: {error}", path.display()));
}

pub fn save_to_storage(tasks: &Tasks) {
    save_to_path(&storage_file_path(), tasks);
}
