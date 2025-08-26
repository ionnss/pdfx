// indexer.rs

// imports
use std::path::Path;
use walkdir::WalkDir;
use chrono::{Utc, DateTime};
use crate::database::database::PdfDatabase;
use crate::types::PdfEntry;

// function to scan a directory for PDF files
pub fn scan_directory(path: &Path, database: &PdfDatabase) -> Result<(),Box<dyn std::error::Error>> {
    // 1. Walk the directory tree recursively with walkdir
    for entry in WalkDir::new(path) {

        // 1.1. Error handling for walkdir 
        let entry = entry?;

        // 1.2. Get the file path
        let file_path = entry.path();

        // 1.3. Check if the file is a PDF
        if let Some(extension) = file_path.extension() {
            if extension == "pdf" {
                // 1.3.1. Get the file metadata
                let metadata = file_path.metadata()?;

                // 1.3.2. Create PdfEntry
                let pdf_entry = PdfEntry {
                    id: None,
                    path: file_path.to_string_lossy().to_string(),
                    filename: file_path.file_name().unwrap().to_string_lossy().to_string(),
                    size: metadata.len(),
                    modified: DateTime::from(metadata.modified()?),
                    indexed_at: Some(Utc::now()),
                };

                // 1.3.4. Insert the PDF into the database
                database.insert_pdf(&pdf_entry)?;
            }
        }

    }   
    Ok(())
}