mod cli;
mod commands;
mod storage;
mod tui;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            description,
            priority,
            due,
            tags,
            notes,
        } => commands::add::run(description, priority, due, tags, notes),
        Commands::List {
            all,
            priority,
            tag,
            due,
        } => commands::list::run(all, priority, tag, due),
        Commands::Edit => commands::edit::run(),
    }
}
