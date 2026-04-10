use clap::{Parser, Subcommand};

use crate::models::task::Priority;

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
    SetPriority {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        priority: Priority,
    },
    List,
}
