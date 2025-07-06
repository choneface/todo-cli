mod cli;
mod commands;
mod storage;

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
        Commands::List {
            all,
            priority,
            tag,
            due,
        } => commands::list::run(all, priority, tag, due),
        Commands::Done => commands::done::run(),
        Commands::Remove => commands::remove::run(),
        Commands::Edit => commands::edit::run(),
    }
}
