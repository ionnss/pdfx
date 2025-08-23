// dash.rs

// Struct to store the data
pub struct DashData {
    pub total_counts: usize,
    pub total_size: u64,
    pub largest_file: Option<String>,
    pub newest_file: Option<String>,
    pub folders_scanned: usize, 
}

// Constructor for DashData
impl DashData {
    pub fn new() -> Self {
        Self {
            total_counts: 0,
            total_size: 0,
            largest_file: None,
            newest_file: None,
            folders_scanned: 0,
        }
    }
}