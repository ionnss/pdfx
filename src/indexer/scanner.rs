// indexer.rs

// imports
use crate::database::db::PdfDatabase;
use crate::types::PdfEntry;
use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use walkdir::WalkDir;

// function to scan a directory for PDF files
pub fn scan_directory(
    path: &Path,
    database: &PdfDatabase,
) -> Result<(), Box<dyn std::error::Error>> {
    // Phase 1: Count total files for accurate progress bar
    let count_pb = ProgressBar::new_spinner();
    count_pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg} {pos}")
            .unwrap()
            .progress_chars("⣿⣷⣯⣟⡿⢿⠿⠟⠛⠋ "),
    );
    count_pb.set_message("Counting files...");

    let mut total_files = 0;
    for entry in WalkDir::new(path) {
        if entry.is_ok() {
            total_files += 1;
            if total_files % 100 == 0 {
                count_pb.set_position(total_files);
            }
        }
    }
    count_pb.finish_and_clear();

    // Phase 2: Process files with accurate progress bar
    let pb = ProgressBar::new(total_files);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("🔍 {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files | {per_sec} | ETA: {eta}")
            .unwrap()
            .progress_chars("⣿⣷⣯⣟⡿⢿⠿⠟⠛⠋ ")
    );
    pb.set_message("Scanning for PDFs...");

    let mut files_processed = 0;
    let mut pdfs_found = 0;
    let mut dirs_skipped = 0;
    let mut pdfs_processed = 0; // Counter for PDFs being processed

    // Walk the directory tree recursively with walkdir
    for entry in WalkDir::new(path) {
        // Error handling for walkdir - silent skip permission denied
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                dirs_skipped += 1;
                continue;
            }
        };

        files_processed += 1;
        pb.set_position(files_processed);

        // 1.2. Get the file path
        let file_path = entry.path();

        // 1.3. Check if the file is a PDF
        if let Some(extension) = file_path.extension() {
            if extension == "pdf" {
                // 1.3.1. Get the file metadata
                let metadata = file_path.metadata()?;

                // 1.3.2. Extract content safely with panic handling
                pdfs_processed += 1;
                
                // Show sub-progress for content extraction
                let filename = file_path.file_name().unwrap().to_string_lossy();
                let short_name = if filename.len() > 25 { 
                    format!("{}...", &filename[..25]) 
                } else { 
                    filename.to_string() 
                };
                
                pb.set_message(format!("Scanning for PDFs... (PDF #{}/{}: {})", 
                    pdfs_processed, pdfs_found + 1, short_name));
                
                let content = PdfDatabase::extract_pdf_content(&file_path.to_string_lossy())
                    .unwrap_or(None);

                // Reset message after extraction
                pb.set_message("Scanning for PDFs...");

                // 1.3.3. Create PdfEntry
                let pdf_entry = PdfEntry {
                    id: None,
                    path: file_path.to_string_lossy().to_string(),
                    filename: file_path.file_name().unwrap().to_string_lossy().to_string(),
                    size: metadata.len(),
                    modified: DateTime::from(metadata.modified()?),
                    indexed_at: Some(Utc::now()),
                    content: content,
                };

                // 1.3.4. Insert the PDF into the database
                database.insert_pdf(&pdf_entry)?;
                pdfs_found += 1;
            }
        }
    }

    // Finish progress bar with summary
    pb.finish_with_message(format!(
        "✅ Scan complete! {} PDFs found | {} files processed | {} directories skipped\n",
        pdfs_found, files_processed, dirs_skipped
    ));

    Ok(())
}
