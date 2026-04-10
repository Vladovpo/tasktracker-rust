use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task-tracker")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        desc: String,
    },
    Delete {
        #[arg(short, long)]
        name: String,
    },
    Complete {
        #[arg(short, long)]
        name: String,
    },
    List,
}
