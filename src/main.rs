use std::io;
use std::io::Write;

// CONSTANTS
const INSTRUCTIONS: &str = "task-tracker CLI tool\n\
-| help - displays help manual page\n\
-| quit - finishes the program";
const HELP_MAN: &str = "\
-| help - displays help manual page\n\
-| quit - finishes the program\n\
-| list - lists existing tasks\n\
-| add - prompts for a new task\n\
-| delete - prompts for an existing task for deletion";

// STRUCTS
struct Task {
    name: String,
    description: String,
    is_completed: bool
}

struct Tasks {
    tasks: Vec<Task>
}

// FUNCTIONS
fn main() {
    // to refactor when I'll have learned structs
    let mut tasks = Tasks {
        tasks: Vec::new()
    };

    menu(&mut tasks);
}

fn menu(tasks: &mut Tasks) {
    println!("{}", INSTRUCTIONS);
    loop {
        let choice: String = input("[task-tracker]$ ");

        match choice.trim() {
            "list" => list(tasks),
            "add" => add(tasks),
            "delete" => delete(tasks),
            "help" => help(),
            "quit" => break,
            _ => println!("Invalid option! Try again")
        }
    }
}

// should print out tasks in a viewable manner
fn list(tasks: &mut Tasks) {
    for (index, task) in tasks.tasks.iter().enumerate() {
        print!("{} - {}\n\
            - {}\n\
            Completed: {}\n", index+1, task.name, task.description, task.is_completed)
    }
}

fn help() {
    println!("{}", HELP_MAN);
}

fn add(tasks: &mut Tasks) {
    let task_name = input("Type in a task to add: ");
    let task_description = input("Type in this task's description: ");

    let task = Task {
        name: task_name,
        description: task_description,
        is_completed: false
    };

    tasks.tasks.push(task)
}

// deleting is completing for less inefficient brainfuck
fn delete(tasks: &mut Tasks) {
    let temp_target: String = input("Type in the literal name of the task to delete: ");
    let target: &str = temp_target.trim();

    let found_index: Option<usize> = find_task_by_name(tasks, target);

    match found_index {
        Some(index) => {
            tasks.tasks.remove(index);
            println!("Task {target} successfully removed");
        },
        None => println!("Task {target} not found"),
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn find_task_by_name(tasks: &Tasks, target: &str) -> Option<usize> {
    let mut target_index: Option<usize> = None;

    for (index, task) in tasks.tasks.iter().enumerate() {
        if task.name.len() > 0 && task.name == target {
            target_index = Some(index);
            return target_index;
        }
    }
    target_index
}