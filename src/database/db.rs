// database.rs

// imports
use crate::types::{PdfEntry, PdfSearchResult};
use rusqlite::{params, Connection, Result};

 
// Connection string for the database
pub struct PdfDatabase {
    conn: Connection,
}

impl PdfDatabase {
    pub fn new(path: &std::path::Path) -> Result<Self> {
        // 1. Open connection to the database
        let conn = Connection::open(path)?;

        // 2. Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pdfs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                filename TEXT NOT NULL,
                size INTEGER NOT NULL,
                modified DATETIME NOT NULL,
                indexed_at DATETIME NOT NULL,
                content TEXT
            )",
            [],
        )?;

        // 3. Return the database instance if successful
        Ok(PdfDatabase { conn })
    }

    pub fn insert_pdf(&self, pdf: &PdfEntry) -> Result<()> {
        // Use INSERT OR REPLACE to handle duplicates
        // This will update the entry if path already exists
        self.conn.execute(
            "INSERT OR REPLACE INTO pdfs (path, filename, size, modified, indexed_at, content)
            VALUES (?, ?, ?, ?, ?, ?)",
            params![
                pdf.path,
                pdf.filename,
                pdf.size as i64,
                pdf.modified.to_rfc3339(),
                pdf.indexed_at.unwrap().to_rfc3339(),
                pdf.content.as_deref(),
            ],
        )?;

        Ok(())
    }

    pub fn open(path: &std::path::Path) -> Result<Self> {
        // 1. Open connection to the database
        let conn = Connection::open(path)?;
        Ok(PdfDatabase { conn })
    }

    pub fn count_pdfs(&self) -> Result<i64> {
        // 1. Count the number of PDFs in the database
        let count = self
            .conn
            .query_row("SELECT COUNT(*) FROM pdfs", [], |row| row.get(0))?;

        // 2. Return the count
        Ok(count)
    }

    pub fn extract_pdf_content(path: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // First, check if the file exists and is readable
        if !std::path::Path::new(path).exists() {
            return Ok(None);
        }

        // Method 1: Try pdftotext (process-isolated, safest)
        let pdftotext_result = std::process::Command::new("pdftotext")
            .arg("-q")  // Quiet mode
            .arg("-eol")  // End of line
            .arg("unix")  // Unix line endings
            .arg("-layout")  // Maintain layout
            .arg(path)
            .arg("-")  // Output to stdout
            .output();

        match pdftotext_result {
            Ok(output) => {
                if output.status.success() {
                    let content = String::from_utf8_lossy(&output.stdout);
                    if content.trim().is_empty() {
                        // Try fallback to pdf-extract
                        return Self::extract_with_pdf_extract(path);
                    } else {
                        return Ok(Some(content.to_string()));
                    }
                } else {
                    // pdftotext failed, try fallback
                    return Self::extract_with_pdf_extract(path);
                }
            }
            Err(_) => {
                // pdftotext not available, try fallback
                return Self::extract_with_pdf_extract(path);
            }
        }
    }

    fn extract_with_pdf_extract(path: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // Method 2: Use pdf-extract with comprehensive panic protection
        // Spawn a separate thread to completely isolate any panics
        let path = path.to_string();
        let handle = std::thread::spawn(move || {
            // Use a timeout to prevent hanging
            let timeout_duration = std::time::Duration::from_secs(10); // 10 second timeout
            let start = std::time::Instant::now();
            
            // Try to extract with panic protection
            let result = std::panic::catch_unwind(|| {
                pdf_extract::extract_text(&path)
            });

            // Check if we've exceeded the timeout
            if start.elapsed() > timeout_duration {
                return Err(pdf_extract::OutputError::IoError(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "PDF extraction timed out"
                )));
            }

            match result {
                Ok(extract_result) => extract_result,
                Err(_) => Err(pdf_extract::OutputError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "PDF extraction panicked"
                ))),
            }
        });

        // Wait for the thread to complete with a timeout
        match handle.join() {
            Ok(extract_result) => {
                match extract_result {
                    Ok(content) => {
                        // Only store non-empty content
                        if content.trim().is_empty() {
                            Ok(None)
                        } else { 
                            Ok(Some(content))
                        }
                    }
                    Err(_) => {
                        // PDF extraction failed - return None and continue
                        Ok(None)
                    }
                }
            }
            Err(_) => {
                // Thread panicked or timed out - return None and continue
                Ok(None)
            }
        }
    }


    pub fn simple_search(
        &self,
        query: &str,
        search_filename: bool,
        search_content: bool,
    ) -> Result<Vec<PdfSearchResult>> {
        // Default - if neither flag is specified, search both
        let (search_filename, search_content) = if !search_filename && !search_content {
            (true, true)
        } else {
            (search_filename, search_content)
        };

        let mut results = Vec::new();

        // 1. Search filenames
        if search_filename {
            let sql = "SELECT id, path, filename, size FROM pdfs WHERE filename LIKE ?1";
            let search_pattern = format!("%{}%", query);
            let mut stmt = self.conn.prepare(sql)?;
            let pdf_iter = stmt.query_map([&search_pattern], |row| {
                Ok(PdfSearchResult {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    filename: row.get(2)?,
                    size: row.get(3)?,
                    content: None,
                })
            })?;

            for pdf in pdf_iter {
                results.push(pdf?);
            }
        }

        // 2. Search content (placeholder for now)
        if search_content {
            
        }

        // At this point, you need to return the results from the function.
        // Since your function signature is `-> Result<Vec<PdfEntry>>`, you should return `Ok(results)` at the end.
        Ok(results)
    }
}
