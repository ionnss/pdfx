// types.rs

// imports
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
    pub content: Option<String>,
}

#[derive(Debug)]
pub struct PdfSearchResult {
    pub id: Option<i64>,
    pub path: String,
    pub filename: String,
    pub size: u64,
    pub content: Option<Vec<ContentMatch>>,
}

#[derive(Debug)]
pub struct ContentMatch {
    pub line_number: usize,
    pub context_before: String,
    pub context_before_count: usize,
    pub matched_line: String,
    pub context_after: String,
    pub context_after_count: usize,
    pub match_count: usize,
}
