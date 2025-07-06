mod cli;
mod commands;

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
        } => commands::add::run(description, priority, due, tags),
        Commands::List => commands::list::run(),
        Commands::Done => commands::done::run(),
        Commands::Remove => commands::remove::run(),
        Commands::Edit => commands::edit::run(),
    }
}
