use crate::{TodoStore, cli::Command, help, Todo};

pub fn compute(command: Command, todo: &mut TodoStore )-> Result<(), Box<dyn std::error::Error>> {
    match command {
        Command::Help => {
            help();
        },
        Command::List => {
            todo.print_todos();
        },
        Command::Add { name, priority, description } => {
            let new_todo = Todo::new(&name, &priority, &description);
            match todo.create_todo(new_todo) {
                Ok(_) => println!("✅ Todo '{}' added successfully!", name),
                Err(e) => eprintln!("❌ Error: {}", e),
            }
        },
        Command::Delete { name } => {
            match todo.delete_todo(name.clone()) {
                Ok(_) => println!("✅ Todo '{}' deleted successfully!", name),
                Err(e) => eprintln!("❌ Error: {}", e),
            }
        },
        Command::ListPriority { priority } => {
            let priority_str = format!("{:?}", priority);
            match todo.get_todos_by_priority(priority) {
                Ok(todos) => {
                    todo.print_filtered_todos(&todos, &priority_str);
                },
                Err(e) => eprintln!("❌ {}", e),
            }
        },
        Command::ListStatus { status } => {
            let status_str = format!("{:?}", status);
            match todo.get_todos_by_status(status) {
                Ok(todos) => {
                    todo.print_filtered_todos(&todos, &status_str);
                },
                Err(e) => eprintln!("❌ {}", e),
            }
        },
        Command::Sort { order } => {
            todo.get_todos_by_sorted_priority(order);
            todo.print_todos();
        },
        Command::Done { name } => {
            match todo.mark_as_done(&name) {
                Ok(_) => println!("✅ Todo '{}' marked as done!", name),
                Err(e) => eprintln!("❌ Error: {}", e),
            }
        },
    }
    Ok(())

}