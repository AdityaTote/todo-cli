use std::process;
use std::env;
use todo_cli::{Priority, Todo, TodoStatus, TodoStore, help};
use todo_cli::cli::{Command, Config};

fn main() {
    let mut todo = TodoStore::new();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match config.command {
        Command::Help => help(),
        Command::Add { name, priority, description } => {
            let todo_data = Todo::new(&name, &priority, &description);
            match todo.create_todo(todo_data) {
                Ok(_) => println!("Todo created successfully"),
                Err(e) => println!("Error creating todo: {}", e),
            }
        }
        Command::Delete { name } => {
            match todo.delete_todo(name) {
                Err(e) => println!("Error deleting todo: {}", e),
                Ok(_) => println!("Todo deleted successfully"),
            }
        }
        Command::List => {
            todo.print_todos();
        }
        Command::ListPriority { priority } => {
            let filter_name = match priority {
                Priority::High => "high",
                Priority::Medium => "medium",
                Priority::Low => "low",
            };
            
            match todo.get_todos_by_priority(priority) {
                Ok(filtered_todos) => {
                    todo.print_filtered_todos(&filtered_todos, filter_name);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Command::ListStatus { status } => {
            let filter_name = match status {
                TodoStatus::Pending => "pending",
                TodoStatus::Completed => "completed",
            };
            
            match todo.get_todos_by_status(status) {
                Ok(filtered_todos) => {
                    todo.print_filtered_todos(&filtered_todos, filter_name);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Command::Sort { order } => {
            todo.get_todos_by_sorted_priority(order);
            todo.print_todos();
        }
        Command::Done { name } => {
            match todo.mark_as_done(&name) {
                Ok(_) => {
                    println!("✓ Todo '{}' marked as completed!", name);
                    todo.print_todos();
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}