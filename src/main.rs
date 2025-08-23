mod dash;

use std::path::Path;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use dash::DashData;

fn main() {
    
    // Get the arguments passed to the program
    let args: Vec<String> = std::env::args().collect();

    // If an argument is provided, use it as the directory to search, otherwise use the current directory
    let current_dir = if args.len() > 1 {
        Path::new(&args[1]).to_path_buf()
    } else {
        std::env::current_dir().unwrap()
    };

    // Print the directory being searched
    println!("\nSearching for PDF files in: {:?}\n", current_dir.display());

    // First pass: Count total files
    println!("Counting files...\n");
    let total_files = WalkDir::new(current_dir.clone())
        .into_iter()
        .filter_map(|e| e.ok())
        .count() as u64;

    // Docker-style progress bar with Braille dots
    let pb = ProgressBar::new(total_files);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {pos}/{len} files ({percent}%) {msg}")
            .unwrap()
            .progress_chars("⣿⣷⣯⣟⡿⢿⠿⠟⠛⠋ ")  // Docker-style Braille dots
    );

    let mut pdf_files = Vec::new();
    let mut processed = 0;

    // Second pass: Process with progress bar
    for entry in WalkDir::new(current_dir.clone()) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        processed += 1;
        pb.set_position(processed);
        pb.set_message(format!("Found {} PDFs", pdf_files.len()));

        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "pdf" {
                pdf_files.push(path.to_path_buf());
            }
        }
    }

    pb.finish_with_message("Search complete!\n");

    // Show results
    if pdf_files.is_empty() {
        println!("\nNo PDF files found in {:?}", current_dir.display());
    } else {
        println!("\nFound {} PDF files:\n", pdf_files.len());
        for pdf_file in pdf_files {
            println!("{:?}", pdf_file);
        }
    }

    
}