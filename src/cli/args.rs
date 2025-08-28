// args.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pdfx")]
#[command(about = "🛠️ A lightning-fast terminal-native PDF toolkit")]
#[command(version = "0.1.0")]
#[command(author = "ions <zara.leonardo@gmail.com>")]
#[command(
    long_about = "pdfx is a lightning-fast terminal-native PDF toolkit. It allows you to index, search, and manage your PDF files with ease."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    // Initialize PDF index for current directory
    Init {
        // Directory to index (defaults to current directory)
        path: Option<PathBuf>,
    },

    // Search indexed PDFs
    Search {
        query: String,
        #[arg(long, short = 'f')]
        filename: bool, // search filename
        #[arg(long, short = 'c')]
        content: bool, // search content
    },

    // Show all indexed PDFs
    List {
        // Show all PDFs (default: false)
        #[arg(short, long)]
        all: bool,
    },

    // Clean up pdfx data and database
    Cleanup,
}
