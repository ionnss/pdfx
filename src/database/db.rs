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
                indexed_at DATETIME NOT NULL
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
            "INSERT OR REPLACE INTO pdfs (path, filename, size, modified, indexed_at)
            VALUES (?, ?, ?, ?, ?)",
            params![
                pdf.path,
                pdf.filename,
                pdf.size as i64,
                pdf.modified.to_rfc3339(),
                pdf.indexed_at.unwrap().to_rfc3339(),
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

    pub fn smart_search(
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
            let mut stmt = self.conn.prepare(&sql)?;
            let pdf_iter = stmt.query_map([&search_pattern], |row| {
                Ok(PdfSearchResult {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    filename: row.get(2)?,
                    size: row.get(3)?,
                })
            })?;

            for pdf in pdf_iter {
                results.push(pdf?);
            }
        }

        // 2. Search content (placeholder for now)
        if search_content {
            // For now, we'll just print a message
            println!("🔍 Content search not implemented yet");
        }

        // At this point, you need to return the results from the function.
        // Since your function signature is `-> Result<Vec<PdfEntry>>`, you should return `Ok(results)` at the end.
        Ok(results)
    }
}
