use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ENUMS
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Priority {
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
    is_completed: Completion,
    creation_date: DateTime<Utc>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tasks {
    tasks: Vec<Task>,
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

    // IDK how to make it but it's genius TRUST
    fn with_task_by_name<F>(&mut self, name: String, action: F)
    where
        F: FnOnce(&mut Tasks, usize),
    {
        let target: &str = name.trim();

        if let Some(index) = self.find_task_by_name(target) {
            action(self, index)
        } else {
            println!("Task {target} not found")
        }
    }

    pub fn print_task(task: &Task) {
        let date_str = task.creation_date.format("%Y-%m-%d").to_string();
        let completion = match task.is_completed {
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

    pub fn completed(tasks: &[Task]) -> usize {
        let mut completed: usize = 0;
        for task in tasks.iter() {
            if task.is_completed == Completion::Completed {
                completed += 1
            }
        }
        completed
    }

    pub fn add(&mut self, name: String, description: String) {
        // Task DATA defined HERE
        let priority = Priority::Low;
        let creation_date = Utc::now();

        let task = Task {
            priority,
            name,
            description,
            is_completed: Completion::Pending,
            creation_date,
        };
        self.tasks.push(task);
    }

    pub fn complete(&mut self, name: String) {
        self.with_task_by_name(name, |tasks, index| {
            tasks.tasks[index].is_completed = Completion::Completed;
        })
    }

    pub fn delete(&mut self, name: String) {
        self.with_task_by_name(name, |tasks, index| {
            tasks.tasks.remove(index);
        })
    }

    pub fn list(&self) {
        println!(
            "Completed: {}\nPending: {}\n",
            Self::completed(&self.tasks),
            Self::total(&self.tasks) - Self::completed(&self.tasks)
        );
        for task in self.tasks.iter() {
            Self::print_task(task);
        }
    }
}
