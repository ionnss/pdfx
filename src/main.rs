// main.rs

use clap::Parser;
use pdfx::cli::args::{Cli, Commands};
use pdfx::cli::commands::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => {
            let target_dir = path.unwrap_or_else(|| std::env::current_dir().unwrap());
            init_command(&target_dir)?;
        }
        Some(Commands::Search { query }) => {
            search_command(&query)?;
        }
        Some(Commands::Recent { limit }) => {
            recent_command(limit)?;
        }
        Some(Commands::List { all }) => {
            list_command(all)?;
        }
        Some(Commands::Cleanup) => {
            cleanup_command()?;
        }
        None => {
            println!("📊 pdfx - Use --help for available commands");
        }
    }
    Ok(())
}