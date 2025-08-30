// types.rs

// imports
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Pdf data structure
#[derive(Debug)]
pub struct PdfEntry {
    pub id: Option<i64>,
    pub path: String,
    pub filename: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub indexed_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct PdfSearchResult {
    pub id: Option<i64>,
    pub path: String,
    pub filename: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfListResult {
    pub id: Option<i64>,
    pub path: String,
    pub filename: String,
    pub size: u64,
    pub size_human: String,
    pub modified: DateTime<Utc>,
}
