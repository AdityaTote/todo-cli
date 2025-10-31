use serde::{Serialize, Deserialize};
use std::{ fmt, fs::{self, read_to_string}, path::PathBuf };

pub mod cli;

#[derive(Debug)]
pub enum SortOrder {
    HighToLow,
    LowToHigh
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum TodoStatus {
    Completed,
    Pending
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoStatus::Completed => write!(f, "✓ Completed"),
            TodoStatus::Pending => write!(f, "◯ Pending"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::High => write!(f, "🔴 HIGH"),
            Priority::Medium => write!(f, "🟡 MEDIUM"),
            Priority::Low => write!(f, "🟢 LOW"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    name: String,
    status: TodoStatus,
    priority: Priority,
    description: Option<String>
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = self.description.as_ref().map(|s| s.as_str()).unwrap_or("No description");
        write!(
            f,
            "│ {} │ {} │ {} │ {} │",
            self.format_column(&self.name, 25),
            self.format_column(&self.status.to_string(), 12),
            self.format_column(&self.priority.to_string(), 12),
            self.format_column(desc, 40)
        )
    }
}

impl Todo {
    fn format_column(&self, text: &str, width: usize) -> String {
        if text.chars().count() > width {
            let truncated: String = text.chars().take(width - 3).collect();
            format!("{:width$}", format!("{}...", truncated), width = width)
        } else {
            format!("{:width$}", text, width = width)
        }
    }

    pub fn new(name: &str, priority: &str, description: &str) -> Self {
        let priority_enum = match priority.to_lowercase().as_str() {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            "low" => Priority::Low,
            _ => Priority::Low
        };

        Self {
            name: name.to_string(),
            status: TodoStatus::Pending,
            priority: priority_enum,
            description: Some(description.to_string())
        }
    }
}

pub struct TodoStore {
    todos: Vec<Todo>,
    file_path: PathBuf
}

impl TodoStore {
    pub fn new() -> Self {
        let file_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(".todo_store.json");
        let todos = Self::load_file(&file_path).unwrap_or_else(|_| Vec::new());
        Self { todos, file_path }
    }

    fn load_file(file_path: &PathBuf) -> Result<Vec<Todo>, String> {
        if !file_path.exists() {
            return Ok(Vec::new());
        };

        let data = read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

        serde_json::from_str(&data).map_err(|e| format!("Failed to parse JSON: {}", e))

    }

    fn save_to_file(&self) -> Result<(), String> {
        let data = serde_json::to_string(&self.todos).map_err(|e| format!("Failed to serialize todos: {}", e))?;
        fs::write(&self.file_path, data).map_err(|e| format!("Failed to write file: {}", e))
    }

    pub fn list_todos(&self) -> Result<&Vec<Todo>, String> {
        if self.todos.is_empty() {
            Err("No todos found".to_string())
        } else {
            Ok(&self.todos)
        }
    }

    pub fn get_todo_by_name(&self, todo_name: &str) -> Result<&Todo, String> {
        let matching_todos: Vec<&Todo> = self.todos.iter().filter(|t| todo_name == t.name).collect();
        if matching_todos.is_empty() {
            Err("Todo not found".to_string())
        } else {
            Ok(matching_todos[0])
        }
    }

    pub fn get_todos_by_status(&self, status: TodoStatus) -> Result<Vec<&Todo>, String> {
        let temp_store: Vec<&Todo> = self.todos.iter().filter(|t| status == t.status).collect();
        if temp_store.is_empty() {
            Err("Todo not found".to_string())
        } else {
            Ok(temp_store)
        }
    }

    pub fn get_todos_by_priority(&self, priority: Priority) -> Result<Vec<&Todo>, String> {
        let temp_store: Vec<&Todo> = self.todos.iter().filter(|t| priority == t.priority).collect();
        if temp_store.is_empty() {
            Err("Todo not found!".to_string())
        } else {
            Ok(temp_store)
        }
    }

   pub fn get_todos_by_sorted_priority(&mut self, sort_ord: SortOrder) {
        match sort_ord {
            SortOrder::HighToLow => {
                self.todos.sort_by(|a, b| a.priority.cmp(&b.priority));
            }
            SortOrder::LowToHigh => {
                self.todos.sort_by(|a, b| b.priority.cmp(&a.priority));
            }
        }
    }

    pub fn create_todo(&mut self, todo: Todo) -> Result<bool, String> {
        let existing_todo = self.get_todo_by_name(&todo.name);
        match existing_todo {
            Ok(_) => return Err("Todo already exists".to_string()),
            Err(_) => {
                self.todos.push(todo);
                self.save_to_file()?;
                Ok(true)
            }
        }
    }

    pub fn delete_todo(&mut self, todo_name: String) -> Result<bool, String> {
        if let Some(index) = self.todos.iter().position(|t| t.name == todo_name) {
            self.todos.remove(index);
            self.save_to_file()?;
            Ok(true)
        } else {
            Err("Todo not found".to_string())
        }
    }

    pub fn mark_as_done(&mut self, todo_name: &str) -> Result<bool, String> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.name == todo_name) {
            todo.status = TodoStatus::Completed;
            self.save_to_file()?;
            Ok(true)
        } else {
            Err("Todo not found".to_string())
        }
    }

    pub fn print_todos(&self) {
        if self.todos.is_empty() {
            println!("\n📋 No todos found! Add one with: todo add <name> <priority> <description>\n");
            return;
        }

        println!("\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                                    📋 YOUR TODO LIST                                              ║");
        println!("╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣");
        println!("│ {:25} │ {:12} │ {:12} │ {:40} │", "NAME", "STATUS", "PRIORITY", "DESCRIPTION");
        println!("├───────────────────────────────────────────────────────────────────────────────────────────────────┤");
        
        for todo in &self.todos {
            println!("{}", todo);
        }
        
        println!("╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝");
        
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.status == TodoStatus::Completed).count();
        let pending = total - completed;
        
        println!("\n📊 Summary: {} total │ ✓ {} completed │ ◯ {} pending\n", total, completed, pending);
    }

    pub fn print_filtered_todos(&self, todos: &[&Todo], filter_type: &str) {
        if todos.is_empty() {
            println!("\n📋 No todos found matching the {} filter!\n", filter_type);
            return;
        }

        println!("\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                               📋 FILTERED TODO LIST ({})                               ║", filter_type.to_uppercase());
        println!("╠═══════════════════════════════════════════════════════════════════════════════════════════════════╣");
        println!("│ {:25} │ {:12} │ {:12} │ {:40} │", "NAME", "STATUS", "PRIORITY", "DESCRIPTION");
        println!("├───────────────────────────────────────────────────────────────────────────────────────────────────┤");
        
        for todo in todos {
            println!("{}", todo);
        }
        
        println!("╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝");
        println!("\n📊 Showing {} todo(s)\n", todos.len());
    }

}

const TODO_HELP: &str = r#"Usage: todo [COMMAND] [ARGUMENTS]

A simple and fast Todo manager written in Rust, with support for:
- Priority (High, Medium, Low)
- Status (Pending / Completed)
- Search and Sorting

Available commands:

    add [NAME] [PRIORITY] [DESCRIPTION]
        Adds a new todo
        Priority must be: high | medium | low
        Example: todo add "Buy Carrots" high "from supermarket"

    delete [NAME]
        Deletes an existing todo (matching by name)
        Example: todo delete "Buy Carrots"

    list
        Lists all todos
        Example: todo list

    list-priority [high|medium|low]
        Lists todos filtered by a specific priority
        Example: todo list-priority high

    list-status [pending|completed]
        Lists todos filtered by status
        Example: todo list-status completed

    sort [high-to-low | low-to-high]
        Sorts todos by priority
        Example: todo sort high-to-low

    done [NAME]
        Marks a todo as completed
        Example: todo done "Buy Carrots"

    help
        Shows this help message

"#;

pub fn help() {
    println!("{}", TODO_HELP);
}