use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

// CLAP PART
#[derive(Parser)]
#[command(name = "task-tracker")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        desc: String,
    },
    Delete {
        #[arg(short, long)]
        name: String,
    },
    Complete {
        #[arg(short, long)]
        name: String,
    },
    List
}

// STRUCTS
#[derive(Serialize, Deserialize)]
struct Task {
    priority: usize,
    name: String,
    description: String,
    is_completed: bool,
    creation_date: DateTime<Utc>,
}

struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    fn load_from_storage() -> Self {
        let raw = std::fs::read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
        let tasks: Vec<Task> = serde_json::from_str(&raw).expect("Failed to parse tasks.json");

        Tasks { tasks }
    }

    fn save_to_storage(&self){
        let data = serde_json::to_string_pretty(&self.tasks).expect("Failed to serialize");
        std::fs::write("tasks.json", data).expect("Failed to write to file");
    }

    fn find_task_by_name(&self, target: &str) -> Option<usize> {
        let mut target_index: Option<usize> = None;

        for (index, task) in self.tasks.iter().enumerate() {
            if task.name.len() > 0 && task.name == target {
                target_index = Some(index);
                return target_index;
            }
        }
        target_index
    }

    // IDK how to make it but it's genius TRUST
    fn with_task_by_name<F>(&mut self, name: String, action: F)
    where F: FnOnce(&mut Tasks, usize)
    {
        let target: &str = name.trim();

        if let Some(index) = self.find_task_by_name(target) { action(self, index) }
        else { println!("Task {target} not found") }
    }

    fn print_task(task: &Task) {
        let date_str = task.creation_date.format("%Y-%m-%d").to_string();
        let completion = if task.is_completed { "+" } else { "-" };

        print!("{} {}\n {}\n  Priority: {}\n  Created: {}\n", completion, task.name, task.description, task.priority, date_str);
    }

    fn total(tasks: &Vec<Task>) -> usize {
        tasks.len()
    }

    fn completed(tasks: &Vec<Task>) -> usize {
        let mut completed: usize = 0;
        for task in tasks.iter() {
            if task.is_completed == true {
                completed += 1
            }
        }
        completed
    }

    fn add(&mut self, name: String, description: String) {
        // Task DATA defined HERE
        let priority = Self::total(&self.tasks) + 1;
        let creation_date = Utc::now();

        let task = Task {
            priority,
            name,
            description,
            is_completed: false,
            creation_date,
        };
        self.tasks.push(task);
    }

    fn complete(&mut self, name: String) {
        self.with_task_by_name(name, | tasks, index | { tasks.tasks[index].is_completed = true; })
    }

    fn delete(&mut self, name: String) {
        self.with_task_by_name(name, | tasks, index | { tasks.tasks.remove(index); })
    }

    fn list(&self) {
        println!("Completed: {}\nPending: {}\n", Self::completed(&self.tasks), Self::total(&self.tasks) - Self::completed(&self.tasks));
        for task in self.tasks.iter() {
            if task.is_completed == false {
                Self::print_task(task);
            } else {
                Self::print_task(task);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = Tasks::load_from_storage();

    match cli.command {
        Commands::Add { name, desc } => { tasks.add(name, desc) },
        Commands::Delete { name } => { tasks.delete(name) },
        Commands::Complete { name } => { tasks.complete(name) },
        Commands::List => tasks.list(),
    }

    tasks.save_to_storage();
}