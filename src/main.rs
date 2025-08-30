// main.rs

use clap::Parser;
use pdfx::cli::args::{Cli, Commands};
use pdfx::cli::commands::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Display ASCII art
    println!(
        "\x1b[34m
░█▀█░█▀▄░█▀▀░█░█
░█▀▀░█░█░█▀▀░▄▀▄
░▀░░░▀▀░░▀░░░▀░▀                                       
\x1b[0m"
    );

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => {
            let target_dir = path.unwrap_or_else(|| std::env::current_dir().unwrap());
            init_command(&target_dir)?;
        }
        Some(Commands::Search { query }) => {
            search_command(&query)?;
        }
        Some(Commands::List {}) => {
            list_command()?;
        }
        Some(Commands::Export { format }) => {
            export_command(format.as_deref())?;
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
