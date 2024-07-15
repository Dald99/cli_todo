use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todo", about = "A simple to-do list application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Add {
        title: String,
        #[arg(default_value = "")]
        description: String,
    },
    Remove {
        id: String
    },
    Done {
        id: String
    },
}
