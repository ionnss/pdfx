// commands.rs

use crate::database::db::PdfDatabase;
use crate::export::exporter::Exporters;
use crate::helpers::help::{
    calculate_search_duration, get_downloads_path, human_readable_size, hyperlink, truncate,
}; //shorten_path
use crate::indexer::scanner::scan_directory;
use dirs;
use std::path::Path;
use std::time::Instant;

pub fn init_command(dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get database path using dirs crate
    let db_path = PdfDatabase::get_database_path()?;

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

pub fn search_command(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check if database exists
    let db_path = PdfDatabase::get_database_path()?;
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
                    hyperlink(&r.path, &r.path) //hyperlink(&shorten_path(&r.path, 40), &r.path)
                );

                println!("\x1b[34m───────────────────────────────────────────────\x1b[0m"); // separator line
                println!(); // empty line between documents
            }
        }
    }

    Ok(())
}

pub fn list_command() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = PdfDatabase::get_database_path()?;
    if !db_path.exists() {
        return Err("Database with indexed PDFs not found. Run 'pdfx init' first.".into());
    }

    let db = PdfDatabase::open(&db_path)?;
    let results = db.get_all_pdfs()?;

    println!("\n📋 All Indexed PDFs");
    println!("📊 Total: {} PDFs\n", results.len());

    for (i, r) in results.iter().enumerate() {
        println!(
            "\x1b[1;94m📄 {}. {}\x1b[0m",
            i + 1,
            truncate(&r.filename, 50)
        ); // Bright blue for filename
        println!("    \x1b[34mSize:\x1b[0m {}", human_readable_size(r.size)); // Standard blue for "Size:"
        println!("    \x1b[36mPath:\x1b[0m {}", hyperlink(&r.path, &r.path)); // Cyan blue for "Path:"
        println!(
            "    \x1b[38;5;75mModified:\x1b[0m {}",
            r.modified.format("%Y-%m-%d %H:%M:%S")
        ); // Light blue for "Modified:"
        println!("\x1b[34m───────────────────────────────────────────────\x1b[0m");
        println!();
    }

    Ok(())
}

pub fn export_command(format: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = PdfDatabase::get_database_path()?;
    if !db_path.exists() {
        return Err("Database with indexed PDFs not found. Run 'pdfx init' first.".into());
    }

    let db = PdfDatabase::open(&db_path)?;
    let export_path = get_downloads_path()?;
    let export_content = db.get_all_pdfs()?;

    if export_content.is_empty() {
        println!("📋 No PDFs found to export.");
        return Ok(());
    }

    let formats_to_export = match format {
        Some(f) => f.split(",").map(|s| s.trim()).collect(),
        None => vec!["json", "csv", "markdown", "yaml", "html"],
    };

    println!(
        "Exporting {} PDFs to {}",
        export_content.len(),
        export_path.display()
    );

    // Generate each format
    for fmt in formats_to_export {
        match fmt {
            "json" => {
                Exporters::export_to_json(&export_content, &export_path)?;
                println!("  ✅ Generated pdfs.json");
            }
            "csv" => {
                Exporters::export_to_csv(&export_content, &export_path)?;
                println!("  ✅ Generated pdfs.csv");
            }
            "markdown" => {
                Exporters::export_to_markdown(&export_content, &export_path)?;
                println!("  ✅ Generated pdfs.md");
            }
            "yaml" => {
                Exporters::export_to_yaml(&export_content, &export_path)?;
                println!("  ✅ Generated pdfs.yaml");
            }
            "html" => {
                Exporters::export_to_html(&export_content, &export_path)?;
                println!("  ✅ Generated pdfs.html");
            }
            _ => {
                println!("  ❌ Unsupported format: {}", fmt);
            }
        }
    }

    println!("🎉 Export complete!");
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
