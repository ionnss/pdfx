// commands.rs

use crate::database::db::PdfDatabase;
use crate::helpers::help::{
    calculate_search_duration, human_readable_size, hyperlink, truncate,
}; //shorten_path
use crate::indexer::scanner::scan_directory;
use dirs;
use std::path::Path;
use std::time::Instant;

pub fn init_command(dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get database path using dirs crate
    let db_path = get_database_path()?;

    // 2. Create PdfDatabase
    let db = PdfDatabase::new(&db_path)?;

    // 3. Call your indexer::scan_directory function
    scan_directory(dir_path, &db)?;

    // 4. Print success message
    println!(
        "\x1b[1;92m✅ Successfully indexed\x1b[0m \x1b[1;92m{}\x1b[0m \x1b[34mPDFs in\x1b[0m \x1b[1;92m{}\x1b[0m\n",
        db.count_pdfs()?,
        db_path.display()
    );
    Ok(())
}

pub fn search_command(
    query: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if database exists
    let db_path = get_database_path()?;
    if !db_path.exists() {
        return Err("Database with indexed PDFs not found".into());
    } else {
        let start_time = Instant::now();

        // Connect to database
        let db = PdfDatabase::open(&db_path)?;

        // In search_command
        let results = db.simple_search(query)?;

        // Print results
        if results.is_empty() {
            println!("❌ No results found for '{}'.", query);
        } else {
            // Header
            println!(
                "\n\n\x1b[34m🔎 Search results for:\x1b[0m \x1b[1;92m{}\x1b[0m\n",
                query
            );
            println!(
                "\x1b[34m⏱️  Search time:\x1b[0m \x1b[1;92m{}ms\x1b[0m\n",
                calculate_search_duration(start_time)
            );
            println!(
                "\x1b[1;92m Σ\x1b[0m\x1b[34m Total results:\x1b[0m \x1b[1;92m{}\x1b[0m",
                results.len()
            );
            println!("\n");

            // Results
            for (i, r) in results.iter().enumerate() {
                println!("📄 {}. {}", i + 1, truncate(&r.filename, 40));
                println!("    Size: {}", human_readable_size(r.size));
                println!(
                    "    Path: {}",
                    hyperlink(&r.path, &r.path)
                    //hyperlink(&shorten_path(&r.path, 40), &r.path)
                );
                

                
                println!("\x1b[34m───────────────────────────────────────────────\x1b[0m"); // separator line
                println!(); // empty line between documents
            }
        }
    }

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
        println!(
            "\n✅ Cleaned up pdfx data directory: {}\n",
            pdfx_dir.display()
        );
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
