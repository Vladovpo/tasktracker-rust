use crate::cli::{Cli, Commands};
use crate::models::task::Tasks;
use crate::storage::json::{load_from_storage, save_to_storage};

pub struct App {
    tasks: Tasks,
}

impl App {
    pub fn new() -> Self {
        let tasks = load_from_storage();

        Self { tasks }
    }

    pub fn run(&mut self, cli: Cli) {
        match cli.command {
            Commands::Add { name, desc } => {
                self.tasks.add(name, desc);
                save_to_storage(&self.tasks);
            }
            Commands::Delete { name } => {
                self.tasks.delete(name);
                save_to_storage(&self.tasks);
            }
            Commands::List => {
                self.tasks.list();
            }
            Commands::Complete { name } => {
                self.tasks.complete(name);
                save_to_storage(&self.tasks);
            }
            Commands::SetPriority { name, priority } => {
                self.tasks.set_priority(name, priority);
                save_to_storage(&self.tasks);
            }
        };
    }
}
