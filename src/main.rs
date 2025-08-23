// Main.rs

use std::path::Path;
use walkdir::WalkDir;


fn main() {
    
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    // Check if user provided a directory
    let current_dir = if args.len() > 1 {
        Path::new(&args[1]).to_path_buf()
    } else {
        std::env::current_dir().unwrap()
    };

    println!("\nSearching for PDF files in: {:?}\n", current_dir.display());

    // Create a vector to store pdf files paths
    let mut pdf_files = Vec::new();

    // Iterate over the directory and subdirectories
    for entry in WalkDir::new(current_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "pdf" {
                pdf_files.push(path.to_path_buf());
            }
        }
    }

    for pdf_file in pdf_files {
        println!("{:?}", pdf_file);
    } if pdf_files.is_empty() {
        println!("No PDF files foun")
    }
}
