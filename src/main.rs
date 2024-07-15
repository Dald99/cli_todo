use clap::{Parser};
use colored::*;
mod cli;
use todo::task::Task;
use todo::tasks::Tasks;
use todo::cli::{Cli, Commands};


fn main() {
    let cli = Cli::parse();

    let task = Task::new("Piano".to_string(), "Learn Piano".to_string());
    let task2 = Task::new("Rust".to_string(), "Learn Rust".to_string());
    let task3 = Task::new("Homework".to_string(), "".to_string());
    let task4 = Task::new("Big Mix".to_string(), "Buy Big Mix".to_string());
    let mut tasks = Tasks::new();
    tasks.add(task);
    tasks.add(task2);
    tasks.add(task3);
    tasks.add(task4);

    match cli.command {
        Commands::List => {
            tasks.list();
        }
        Commands::Add { title, description } => {
            tasks.list();
            let task = Task::new(title, description);
            tasks.add(task);
            println!("{}", "Task added".green());
            tasks.list();
        }
        Commands::Remove { id } => {
            match id.parse::<usize>() {
                Ok(index) => {
                    match tasks.remove(index) {
                        Ok(()) => {
                            println!("{}", "Task removed".green());
                            tasks.list();
                        }
                        Err(e) => {
                            println!("{}", e);
                            tasks.list();
                        }
                    }
                }
                Err(_) => {
                    println!("{}", "Please enter a valid number for the task ID.".red());
                }
            }
        }
        Commands::Done { id } => {
            match id.parse::<usize>() {
                Ok(index) => {
                    match tasks.done(index) {
                        Ok(()) => {
                            println!("{}", "Task changed".green());
                            tasks.list();
                        }
                        Err(e) => {
                            println!("{}", e);
                            tasks.list();
                        }
                    }
                }
                Err(_) => {
                    println!("{}", "Please enter a valid number for the task ID.".red());
                }
            }
        }
    }
}

