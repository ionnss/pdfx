// commands.rs

use std::path::Path;
use dirs;
use crate::database::database::PdfDatabase;
use crate::indexer::indexer::scan_directory;

pub fn init_command(dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get database path using dirs crate
    let db_path = get_database_path()?;

    // 2. Create PdfDatabase
    let db = PdfDatabase::new(&db_path)?;

    // 3. Call your indexer::scan_directory function
    scan_directory(dir_path, &db)?;

    // 4. Print success message
    println!("\nIndexed {} PDFs in {}\n", db.count_pdfs()?, db_path.display());
    Ok(())
}

pub fn search_command(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Future: Search the database for PDFs matching query
    println!("🔍 Search not implemented yet: '{}'", query);
    Ok(())
}

pub fn recent_command(limit: i32) -> Result<(), Box<dyn std::error::Error>> {
    // Future: Show recent PDFs from database
    println!("📅 Recent not implemented yet (limit: {})", limit);
    Ok(())
}

pub fn list_command(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Future: List PDFs from database
    println!("📋 List not implemented yet (all: {})", all);
    Ok(())
}

pub fn cleanup_command() -> Result<(), Box<dyn std::error::Error>> {
    // Check if pdfx directory exists WITHOUT creating it
    let data_dir = dirs::data_dir().expect("Failed to get data directory");
    let pdfx_dir = data_dir.join("pdfx");
    
    if pdfx_dir.exists() {
        // Remove the entire pdfx directory and database
        std::fs::remove_dir_all(&pdfx_dir)?;
        println!("\n✅ Cleaned up pdfx data directory: {}\n", pdfx_dir.display());
        println!("🗑️  Removed database and all indexed data\n");
    } else { 
        println!("\nℹ️  No pdfx data found to clean up\n");
    }
    
    Ok(())
}

fn get_database_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Helper function to get database path in user's data directory
    let data_dir = dirs::data_dir().expect("Failed to get data directory");
    let pdfx_dir = data_dir.join("pdfx");

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&pdfx_dir)?;

    let db_path = pdfx_dir.join("db.sqlite");
    Ok(db_path)
}