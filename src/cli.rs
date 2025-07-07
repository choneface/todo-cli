use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple TUI-based todo CLI", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo item
    Add {
        /// Description of the task
        description: String,

        /// Optional priority (0–9)
        #[arg(long)]
        priority: Option<u8>,

        /// Optional due date (YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,

        /// Comma-separated list of tags
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// List all todos
    List {
        /// Include completed tasks
        #[arg(long)]
        all: bool,

        /// Filter by priority
        #[arg(long)]
        priority: Option<u8>,

        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,

        /// Filter by due date (YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,
    },

    /// Launch TUI editor to complete/edit todos
    Edit,
}
