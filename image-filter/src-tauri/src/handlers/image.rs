use crate::models::is_raw_file;
use std::path::PathBuf;

/// Generate a thumbnail from an image file
pub fn generate_thumbnail(path: &PathBuf, width: u32) -> Result<String, String> {
    // For RAW files, try to extract embedded preview or use image crate
    if is_raw_file(
        &path.extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
    ) {
        // Try to open with image crate (some RAW formats are supported)
        match image::open(path) {
            Ok(img) => {
                let thumbnail = img.thumbnail(width, width);
                let mut buffer = Vec::new();
                thumbnail
                    .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
                    .map_err(|e| format!("Failed to write thumbnail: {}", e))?;
                return Ok(format!("data:image/png;base64,{}", base64_encode(&buffer)));
            }
            Err(_) => {
                // If image crate fails, try to read embedded JPEG preview from RAW
                return extract_raw_preview(path, Some(width));
            }
        }
    }
    
    // Regular image - generate thumbnail
    let img = image::open(path).map_err(|e| format!("Failed to open image: {}", e))?;
    let thumbnail = img.thumbnail(width, width);
    
    let mut buffer = Vec::new();
    thumbnail
        .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
        .map_err(|e| format!("Failed to write thumbnail: {}", e))?;
    
    Ok(format!("data:image/png;base64,{}", base64_encode(&buffer)))
}

/// Extract preview from RAW file
fn extract_raw_preview(path: &PathBuf, max_width: Option<u32>) -> Result<String, String> {
    // Try to open as regular image first (some RAW files have embedded previews)
    if let Ok(img) = image::open(path) {
        let mut buffer = Vec::new();
        
        if let Some(width) = max_width {
            let thumbnail = img.thumbnail(width, width);
            thumbnail
                .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
                .map_err(|e| format!("Failed to encode image: {}", e))?;
        } else {
            img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
                .map_err(|e| format!("Failed to encode image: {}", e))?;
        }
        
        return Ok(format!("data:image/png;base64,{}", base64_encode(&buffer)));
    }
    
    // If that fails, the file might need specialized RAW decoding
    Err("Cannot preview RAW file - no embedded preview available".to_string())
}

/// Base64 encode data
pub fn base64_encode(data: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.encode(data)
}
