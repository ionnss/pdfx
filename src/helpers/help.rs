// helper.rs

use dirs;
use std::path::PathBuf;
use std::time::Instant;

// Nested helper functions
pub fn yes_no(flag: bool) -> &'static str {
    if flag {
        "✅"
    } else {
        "❌"
    }
}

// Human readable size
pub fn human_readable_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
    } else {
        format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
    }
}

// Truncate long strings
pub fn truncate(s: &str, len: usize) -> String {
    if s.len() > len {
        format!("{}...", &s[..len])
    } else {
        s.to_string()
    }
}

// Make terminal hyperlink (OSC 8)
pub fn hyperlink(text: &str, path: &str) -> String {
    format!("\x1b]8;;file://{}\x07{}\x1b]8;;\x07", path, text)
}

// /// Shorten a file path for display purposes.
// /// If the path is longer than `max_len`, it will be shortened to show only the last two components,
// /// prefixed with ".../". Otherwise, the original path is returned.
// // pub fn shorten_path(path: &str, max_len: usize) -> String {
// //     if path.len() <= max_len {
// //         path.to_string()
// //     } else {
// //         let parts: Vec<&str> = path.split('/').collect();
// //         if parts.len() > 3 {
// //             format!(".../{}/{}", parts[parts.len() - 2], parts[parts.len() - 1])
// //         } else {
// //             path.to_string()
// //         }
// //     }
// // }

// Calculate search duration
pub fn calculate_search_duration(start_time: Instant) -> u64 {
    let end_time = Instant::now();
    end_time.duration_since(start_time).as_millis() as u64
}

// Get Downloads directory for exporting PDFs
pub fn get_downloads_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let download_dir = dirs::download_dir().ok_or("Failed to get downloads directory")?;
    let export_dir = download_dir.join("pdfx_exports");
    std::fs::create_dir_all(&export_dir)?;
    Ok(export_dir)
}

//
