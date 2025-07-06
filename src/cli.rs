use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        description: String,

        #[arg(long)]
        priority: Option<u8>,

        #[arg(long)]
        due: Option<String>,

        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    List {
        #[arg(long)]
        all: bool,

        #[arg(long)]
        priority: Option<u8>,

        #[arg(long)]
        tag: Option<String>,

        #[arg(long)]
        due: Option<String>,
    },
    Edit,
}