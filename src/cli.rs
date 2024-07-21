use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todo", about = "A simple to-do list application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "[List all tasks in the todo list]")]
    List,
    #[command(
        about = "[Add a new task to the todo list]",
        long_about = "Add a new task to the todo list with a title and an optional description. \
        Example usage:\n\n`todo add \"Buy groceries\" \"Milk, Eggs, Bread\"`"
    )]
    Add {
        title: String,
        #[arg(default_value = "")]
        description: String,
    },
    #[command(
        about = "[Remove a task from the todo list by its ID]",
        long_about = "Remove a task from the todo list by specifying its ID. Example usage:\n\n`todo remove 1`"
    )]
    Remove {
        id: String
    },
    #[command(
        about = "[Mark a task as done by its ID]",
        long_about = "Mark a task as done by specifying its ID. Example usage:\n\n`todo done 1`"
    )]
    Done {
        id: String
    },
}
