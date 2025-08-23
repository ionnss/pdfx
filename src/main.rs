// Main.rs

use walkdir::WalkDir;

fn main() {

    // Get the current directory
    let current_dir = std::env::current_dir().unwrap();
    println!("Searching for PDF files in: {:?}", current_dir.display());

    // Create a vector to store pdf files paths
    let mut pdf_files = Vec::new();

    // Iterate over the directory and subdirectories
    for entry in WalkDir::new(current_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "pdf" {
                pdf_files.push(path.to_path_buf());
            }
        }
    }

    for pdf_file in pdf_files {
        println!("{:?}", pdf_file);
    }
}
