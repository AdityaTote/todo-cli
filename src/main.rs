use std::process;
use std::env;
use todo_cli::{ TodoStore, cli::Config, run::compute };

fn main() {
    let mut todo = TodoStore::new();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = compute(config.command, &mut todo) {
        println!("Application error: {err}");
        process::exit(1);
    }
}