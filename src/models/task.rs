use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

// ENUMS
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Completion {
    Completed,
    Pending,
}

// STRUCTS
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    priority: Priority,
    name: String,
    description: String,
    completion: Completion,
    creation_date: DateTime<Utc>,
    last_edit: DateTime<Utc>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<Task>,
    last_edit: DateTime<Utc>,
}

impl Tasks {
    pub fn find_task_by_name(&self, target: &str) -> Option<usize> {
        for (index, task) in self.tasks.iter().enumerate() {
            if task.name == target {
                return Some(index);
            }
        }

        None
    }

    pub fn print_task(task: &Task) {
        let date_str = task.creation_date.format("%Y-%m-%d").to_string();
        let completion = match task.completion {
            Completion::Completed => "C",
            Completion::Pending => "P",
        };
        let priority = match task.priority {
            Priority::Low => "!",
            Priority::Medium => "!!",
            Priority::High => "!!!",
        };

        print!(
            "{} {}\n {}\n  Priority: {}\n  Created: {}\n",
            completion, task.name, task.description, priority, date_str
        );
    }

    pub fn total(tasks: &[Task]) -> usize {
        tasks.len()
    }

    pub fn list_completed(tasks: &[Task]) -> usize {
        let mut completed: usize = 0;
        for task in tasks.iter() {
            if task.completion == Completion::Completed {
                completed += 1
            }
        }
        completed
    }

    pub fn add(&mut self, name: String, description: String) {
        // Task DATA defined HERE
        let priority = Priority::Low;
        let creation_date = Utc::now();
        let last_edit = Utc::now();

        let task = Task {
            priority,
            name,
            description,
            completion: Completion::Pending,
            creation_date,
            last_edit,
        };
        self.tasks.push(task);

        self.last_edit = Utc::now();
    }

    pub fn complete(&mut self, name: String) {
        let target: &str = name.trim();

        if let Some(index) = self.find_task_by_name(target) {
            self.tasks[index].completion = Completion::Completed;

            self.last_edit = Utc::now();
        } else {
            println!("Task {target} not found")
        }
    }

    pub fn set_priority(&mut self, name: String, priority: Priority) {
        let target: &str = name.trim();

        if let Some(index) = self.find_task_by_name(target) {
            self.tasks[index].priority = priority;

            self.last_edit = Utc::now();
        } else {
            println!("Task {target} not found")
        }
    }

    pub fn delete(&mut self, name: String) {
        let target: &str = name.trim();

        if let Some(index) = self.find_task_by_name(target) {
            self.tasks.remove(index);

            self.last_edit = Utc::now();
        } else {
            println!("Task {target} not found")
        }
    }

    pub fn list(&self) {
        println!(
            "Completed: {}\nPending: {}\n",
            Self::list_completed(&self.tasks),
            Self::total(&self.tasks) - Self::list_completed(&self.tasks)
        );
        for task in self.tasks.iter() {
            Self::print_task(task);
        }
    }
}
