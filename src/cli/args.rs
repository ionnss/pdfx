// args.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pdfx")]
#[command(about = "🛠️ A lightning-fast terminal-native PDF toolkit")]
#[command(version = "0.1.0")]
#[command(author = "ions <zara.leonardo@gmail.com>")]
#[command(
    long_about = "pdfx is a lightning-fast terminal-native PDF toolkit. It allows you to index, search, and manage your PDF files with ease.

EXAMPLES:
  # Initialize PDF index
  pdfx init                    # Indexes current directory
  pdfx init ~                  # Indexes home directory
  pdfx init ~/Documents        # Indexes specific directory

  # Search indexed PDFs
  pdfx search \"machine learning\"     # Search in both filename and content
  pdfx search \"rust\" --filename      # Search in filenames only
  pdfx search \"concurrency\" --content # Search in content only

  # List PDFs
  pdfx list                   # Show recent PDFs only
  pdfx list -a                # Show all PDFs with details

  # Clean up
  pdfx cleanup                # Removes all indexed data"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize PDF index for current directory
    ///
    /// Examples:
    ///   pdfx init                    # Indexes current directory
    ///   pdfx init ~                  # Indexes home directory  
    ///   pdfx init ~/Documents        # Indexes specific directory
    #[command(name = "init")]
    Init {
        /// Directory to index (defaults to current directory)
        path: Option<PathBuf>,
    },

    /// Search indexed PDFs
    ///
    /// Examples:
    ///   pdfx search "machine learning"     # Search in filenames
    ///   pdfx search "rust programming"     # Search in filenames
    ///
    /// Note: Search is performed on PDF filenames only.
    #[command(name = "search")]
    Search {
        /// Search query
        query: String,
    },

    /// Show all indexed PDFs
    ///
    /// Examples:
    ///   pdfx list                   # Show recent PDFs only
    ///   pdfx list -a                # Show all PDFs with details
    ///   pdfx list --all             # Same as -a flag
    #[command(name = "list")]
    List {
        /// Show all PDFs (default: false)
        #[arg(short, long)]
        all: bool,
    },

    /// Clean up pdfx data and database
    ///
    /// Examples:
    ///   pdfx cleanup                # Removes all indexed data
    #[command(name = "cleanup")]
    Cleanup,
}
