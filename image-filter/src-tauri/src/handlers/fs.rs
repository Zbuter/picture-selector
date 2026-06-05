use crate::models::{ImageFile, is_supported_image, is_raw_file};
use std::fs;
use std::path::PathBuf;

/// Create ImageFile struct from a file path
pub fn create_image_file(path: PathBuf) -> Result<ImageFile, String> {
    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let size = metadata.len();
    let modified = metadata.modified()
        .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64)
        .unwrap_or(0);
    
    // Check if it's a RAW file
    let is_raw = path.extension()
        .map(|ext| is_raw_file(&ext.to_string_lossy().to_lowercase()))
        .unwrap_or(false);
    
    // Generate thumbnail for preview (skip for large RAW files to improve performance)
    let thumbnail = if !is_raw || size < 50_000_000 {
        crate::handlers::image::generate_thumbnail(&path, 400).ok()
    } else {
        None
    };
    
    Ok(ImageFile {
        path: path.to_string_lossy().to_string(),
        name,
        size,
        modified,
        is_raw,
        thumbnail,
    })
}

/// Load all images from a folder
pub fn load_images_from_folder(folder_path: &str) -> Result<Vec<ImageFile>, String> {
    let path = PathBuf::from(folder_path);
    
    if !path.exists() || !path.is_dir() {
        return Err("Invalid folder path".to_string());
    }
    
    let mut images = Vec::new();
    
    for entry in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();
        
        if file_path.is_file() {
            if let Some(ext) = file_path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if is_supported_image(&ext_lower) {
                    let image = create_image_file(file_path)?;
                    images.push(image);
                }
            }
        }
    }
    
    Ok(images)
}
