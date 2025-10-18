use std::env;
use todo_cli:: { help, Todo, TodoStore, Priority, TodoStatus };


fn main() {
    let mut todo = TodoStore::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let command = args[1].as_str();

    match command {
        "help" => help(),
        "add" => {
            if args.len() < 4 {
                println!("Usage: todo-cli add <name> <priority> [description]");
            } else {
                let todo_data = Todo::new(&args[2], &args[3], &args[4]);
                match todo.create_todo(todo_data) {
                    Ok(_) => println!("Todo created successfully"),
                    Err(e) => println!("Error creating todo: {}", e),
                }
            }
        },
        "delete" => {
            if args.len() < 3 {
                println!("Usage: todo-cli delete <name>");
            } else {
                let todo_name = args[2..].join(" ");
               match todo.delete_todo(todo_name.trim().to_string()) {
                   Err(e) => println!("Error deleting todo: {}", e),
                   Ok(_) => println!("Todo deleted successfully"),
               }
            }
        },
        "list" => {
            todo.print_todos();
        },
        "list-priority" => {
            if args.len() < 3 {
                println!("Usage: todo-cli list-priority <high|medium|low>");
            } else  {
                let priority = match args[2].to_lowercase().as_str() {
                    "high" => Priority::High,
                    "medium" => Priority::Medium,
                    "low" => Priority::Low,
                    _ => {
                        println!("Invalid priority. Use high, medium, or low.");
                        return;
                    }
                };
                match todo.get_todos_by_priority(priority) {
                    Ok(filtered_todos) => {
                        todo.print_filtered_todos(&filtered_todos, &args[2]);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        "list-status" => {
            if args.len() < 3 {
                println!("Usage: todo-cli list-status <pending|completed>");
            } else  {
                let status = match args[2].to_lowercase().as_str() {
                    "pending" => TodoStatus::Pending,
                    "completed" => TodoStatus::Completed,
                    _ => {
                        println!("Invalid status. Use pending or completed.");
                        return;
                    }
                };
                match todo.get_todos_by_status(status) {
                    Ok(filtered_todos) => {
                        todo.print_filtered_todos(&filtered_todos, &args[2]);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        "sort" => {
            if args.len() < 3 {
                println!("Usage: todo-cli sort <high-to-low|low-to-high>");
            } else  {
                let sort_ord = match args[2].to_lowercase().as_str() {
                    "high-to-low" => todo_cli::SortOrder::HighToLow,
                    "low-to-high" => todo_cli::SortOrder::LowToHigh,
                    _ => {
                        println!("Invalid sort order. Use high-to-low or low-to-high.");
                        return;
                    }
                };
                todo.get_todos_by_sorted_priority(sort_ord);
                todo.print_todos();
            }
        },
        "done" => {
            if args.len() < 3 {
                println!("Usage: todo-cli done <name>");
            } else {
                let todo_name = args[2..].join(" ");
                match todo.mark_as_done(&todo_name) {
                    Ok(_) => {
                        println!("✓ Todo '{}' marked as completed!", todo_name);
                        todo.print_todos();
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        _ => {
            println!("Unknown command: {}", command);
            help();
        }
    }
}
