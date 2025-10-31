
use crate::{Priority, TodoStatus, SortOrder, help};

#[derive(Debug)]
pub enum Command {
    Add {
        name: String,
        priority: String,
        description: String
    },
    Delete { name: String },
    ListPriority { priority: Priority },
    ListStatus { status: TodoStatus},
    Sort { order: SortOrder },
    Done { name: String },
    List,
    Help
}

#[derive(Debug)]
pub struct  Config {
    pub command: Command
}

impl Config  {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            help();
            return Err("not enough arguments");
        }

        let action = args[1].as_str();

        let command = match action {
            "help" => Command::Help,
            "list" => Command::List,
            "add" => {
                if args.len() < 5 {
                    return Err("Usage: todo-cli add <name> <priority> <description>");
                }
                Command::Add { 
                    name: args[2].clone(), 
                    priority: args[3].clone(), 
                    description: args[4].clone() 
                }
            },
            "delete" => {
                if args.len() < 3 {
                    return Err("Usage: todo-cli delete <name>");
                }
                Command::Delete { name: args[2..].join(" ").trim().to_string() }
            },
            "list-priority" => {
                if args.len() < 3 {
                    return Err("Usage: todo-cli list-priority <high|medium|low>");
                } 
                let priority = match args[2].to_lowercase().as_str() {
                    "high" => Priority::High,
                    "medium" => Priority::Medium,
                    "low" => Priority::Low,
                    _ => return Err("Invalid priority. Use high, medium, or low."),
                };
                Command::ListPriority { priority }
            },
            "list-status" => {
                if args.len() < 3 {
                    return Err("Usage: todo-cli list-status <pending|completed>");
                }
                let status = match args[2].to_lowercase().as_str() {
                    "pending" => TodoStatus::Pending,
                    "completed" => TodoStatus::Completed,
                    _ => return Err("Invalid status. Use pending or completed."),
                };
                Command::ListStatus { status }
            },
            "sort" => {
                if args.len() < 3 {
                    return Err("Usage: todo-cli sort <high-to-low|low-to-high>");
                }

                let order = match args[2].to_lowercase().as_str() {
                    "high-to-low" => SortOrder::HighToLow,
                    "low-to-high" => SortOrder::LowToHigh,
                    _ => return Err("Invalid sort order. Use high-to-low or low-to-high."),
                };

                Command::Sort { order }
            },
            "done" => {
                if args.len() < 3 {
                    return Err("Usage: todo-cli done <name>");
                }
                Command::Done { name: args[2..].join(" ").trim().to_string() }
            },
            _ => return Err("Unknown command. Use 'help' to see available commands."),
        };
        Ok(Config{ command })
    }
}