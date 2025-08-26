// database.rs

// imports
use rusqlite::{Connection, Result, params};
use crate::types::PdfEntry;

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
                path TEXT NOT NULL,
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
        // 1. Insert the PDF into the database
        self.conn.execute(
            "INSERT INTO pdfs (path, filename, size, modified, indexed_at)
            VALUES (?, ?, ?, ?, ?)",
           params![
                pdf.path,
                pdf.filename,
                pdf.size as i64,
                pdf.modified.to_rfc3339(),
                pdf.indexed_at.unwrap().to_rfc3339(),
            ]
        )?;

        Ok(())
    }
}

