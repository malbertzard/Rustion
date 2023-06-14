use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Todo {
    tasks: Vec<String>,
}

impl Todo {
    fn new() -> Todo {
        Todo { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn display_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks.");
        } else {
            println!("Tasks:");
            for (index, task) in self.tasks.iter().enumerate() {
                println!("{}. {}", index + 1, task);
            }
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task removed.");
        } else {
            println!("Invalid task index.");
        }
    }

    fn save_tasks(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        fs::write(path, yaml)?;
        Ok(())
    }

    fn load_tasks(path: &str) -> Result<Todo, Box<dyn std::error::Error>> {
        let yaml = fs::read_to_string(path)?;
        let todo: Todo = serde_yaml::from_str(&yaml)?;
        Ok(todo)
    }
}

struct AddCommand;

impl AddCommand {
    fn execute(todo: &mut Todo, parts: &[&str]) {
        if parts.len() < 2 {
            println!("Invalid command. Usage: add <task>");
        } else {
            let task = parts[1].to_string();
            todo.add_task(task);
            println!("Task added.");
        }
    }
}

struct ListCommand;

impl ListCommand {
    fn execute(todo: &Todo) {
        todo.display_tasks();
    }
}

struct RemoveCommand;

impl RemoveCommand {
    fn execute(todo: &mut Todo, parts: &[&str]) {
        if parts.len() < 2 {
            println!("Invalid command. Usage: remove <index>");
        } else {
            if let Ok(index) = parts[1].parse::<usize>() {
                todo.remove_task(index - 1);
            } else {
                println!("Invalid task index.");
            }
        }
    }
}

struct SaveCommand;

impl SaveCommand {
    fn execute(todo: &Todo, path: &str) {
        if let Err(err) = todo.save_tasks(path) {
            eprintln!("Failed to save tasks to '{}': {}", path, err);
        } else {
            println!("Tasks saved to '{}'.", path);
        }
    }
}

struct ExitCommand;

impl ExitCommand {
    fn execute() {
        println!("Goodbye!");
    }
}

fn main() {
    let path = "tasks.yaml";

    let mut todo = if Path::new(path).exists() {
        Todo::load_tasks(path).unwrap_or_else(|_| {
            eprintln!("Failed to load tasks from '{}'. Starting with an empty list.", path);
            Todo::new()
        })
    } else {
        Todo::new()
    };

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        let parts: Vec<&str> = command.splitn(2, ' ').collect();
        let action = parts[0];

        match action {
            "add" => AddCommand::execute(&mut todo, &parts),
            "list" => ListCommand::execute(&todo),
            "remove" => RemoveCommand::execute(&mut todo, &parts),
            "save" => SaveCommand::execute(&todo, path),
            "exit" => {
                ExitCommand::execute();
                if let Err(err) = todo.save_tasks(path) {
                    eprintln!("Failed to save tasks to '{}': {}", path, err);
                }
                break;
            }
            _ => {
                println!("Invalid command.");
            }
        }
    }
}


