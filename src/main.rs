use clap::Parser;
use colored::*;

use todo::cli::{Cli, Commands};
use todo::json::{load_file, save_file};
use todo::task::Task;
use todo::tasks::Tasks;

mod cli;
fn main() {
    let cli = Cli::parse();
    let filename = "tasks.json";

    let mut tasks = match load_file(filename) {
        Ok(tasks) => {
            tasks
        }
        Err(_) => Tasks::new()
    };

    if let Err(e) = handle_command(cli.command, &mut tasks, filename) {
        eprintln!("{}", e);
    }
}

fn handle_command(command: Commands, tasks: &mut Tasks, filename: &str) -> Result<(), String> {
    match command {
        Commands::List => {
            tasks.list();
            Ok(())
        }
        Commands::Add { title, description } => {
            let task = Task::new(title, description);
            tasks.add(task);
            println!("{}", "Task added".green());
            save_file(tasks, filename).map_err(|e| e.to_string())
        }
        Commands::Remove { id } => {
            match id.parse::<usize>() {
                Ok(index) => {
                    match tasks.remove(index) {
                        Ok(()) => {
                            println!("{}", "Task removed".green());
                            save_file(tasks, filename).map_err(|e| e.to_string())
                        }
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(_) => Err("Please enter a valid number for the task ID.".to_string().red().to_string()),
            }
        }
        Commands::Done { id } => {
            match id.parse::<usize>() {
                Ok(index) => {
                    match tasks.done(index) {
                        Ok(()) => {
                            println!("{}", "Task marked as done".green());
                            save_file(tasks, filename).map_err(|e| e.to_string())
                        }
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(_) => Err("Please enter a valid number for the task ID.".to_string().red().to_string()),
            }
        }
    }
}

