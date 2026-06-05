use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFile {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub modified: u64,
    pub is_raw: bool,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    pub name: String,
    pub path: String,
    pub is_removable: bool,
    pub total_space: u64,
    pub free_space: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyResult {
    pub success: bool,
    pub copied: u32,
    pub failed: u32,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEvent {
    pub event_type: String,
    pub path: String,
}

// Supported image extensions
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif", "svg",
    "arw", "nef", "cr2", "cr3", "dng", "raf", "orf", "pef",
    "srw", "rw2", "iiq", "3fr", "fff", "mos", "mef", "kdc", "rwl", "nrw",
];

// RAW format extensions
pub const RAW_EXTENSIONS: &[&str] = &[
    "arw", "nef", "cr2", "cr3", "dng", "raf", "orf", "pef",
    "srw", "rw2", "iiq", "3fr", "fff", "mos", "mef", "kdc", "rwl", "nrw",
];

pub fn is_supported_image(ext: &str) -> bool {
    SUPPORTED_EXTENSIONS.contains(&ext)
}

pub fn is_raw_file(ext: &str) -> bool {
    RAW_EXTENSIONS.contains(&ext)
}
