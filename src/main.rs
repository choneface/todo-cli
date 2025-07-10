mod cli;
mod commands;
mod storage;
mod tui;

use crate::storage::FileStorage;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let storage = FileStorage::new("todo.json");

    match cli.command {
        Commands::Add {
            description,
            priority,
            due,
            tags,
            notes,
        } => commands::add::run(storage, description, priority, due, tags, notes),
        Commands::List {
            all,
            priority,
            tag,
            due,
        } => commands::list::run(storage, all, priority, tag, due),
        Commands::Edit => commands::edit::run(storage),
    }
}
