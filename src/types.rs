// types.rs

// imports
use chrono::{DateTime, Utc};

// Pdf data structure
pub struct PdfEntry {
    pub id: Option<i64>,
    pub path: String,
    pub filename: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub indexed_at: Some(Utc::now()),
}


