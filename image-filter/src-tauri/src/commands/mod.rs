use crate::models::{ImageFile, DriveInfo, CopyResult};
use std::path::PathBuf;
use tauri::AppHandle;

/// Select a folder using system dialog
#[tauri::command]
pub async fn select_folder(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let (tx, rx) = std::sync::mpsc::channel();
    
    app.dialog()
        .file()
        .pick_folder(move |folder| {
            tx.send(folder.map(|p| p.to_string())).ok();
        });
    
    let folder_path = rx.recv().map_err(|e| e.to_string())?.ok_or("No folder selected")?;
    
    Ok(folder_path)
}

/// Load images from a folder
#[tauri::command]
pub fn load_images(path: &str) -> Result<Vec<ImageFile>, String> {
    crate::handlers::fs::load_images_from_folder(path)
}

/// Generate thumbnail for an image
#[tauri::command]
pub fn generate_thumbnail(path: &str, width: u32) -> Result<String, String> {
    let img_path = PathBuf::from(path);
    crate::handlers::image::generate_thumbnail(&img_path, width)
}

/// Get list of external drives
#[tauri::command]
pub fn get_drives() -> Result<Vec<DriveInfo>, String> {
    detect_external_drives()
}

/// Select target folder for copying
#[tauri::command]
pub async fn select_target_folder(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let (tx, rx) = std::sync::mpsc::channel();
    
    app.dialog()
        .file()
        .pick_folder(move |folder| {
            tx.send(folder.map(|p| p.to_string())).ok();
        });
    
    let folder_path = rx.recv().map_err(|e| e.to_string())?.ok_or("No folder selected")?;
    
    Ok(folder_path)
}

/// Copy images to target folder
#[tauri::command]
pub fn copy_images(source_paths: Vec<String>, target_folder: String) -> Result<CopyResult, String> {
    let dest_path = PathBuf::from(&target_folder);
    
    if !dest_path.exists() {
        std::fs::create_dir_all(&dest_path).map_err(|e| e.to_string())?;
    }
    
    let mut copied = 0;
    let mut failed = 0;
    let mut errors = Vec::new();
    
    for source_path in source_paths {
        let src = PathBuf::from(&source_path);
        let filename = src.file_name().unwrap_or_default();
        let dst = dest_path.join(filename);
        
        match std::fs::copy(&src, &dst) {
            Ok(_) => copied += 1,
            Err(e) => {
                failed += 1;
                errors.push(format!("Failed to copy {}: {}", source_path, e));
            }
        }
    }
    
    Ok(CopyResult {
        success: copied > 0,
        copied,
        failed,
        errors,
    })
}

/// Detect external drives (Windows/macOS/Linux)
fn detect_external_drives() -> Result<Vec<DriveInfo>, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("wmic")
            .args(&["logicaldisk", "where", "DriveType=2", "get", "DeviceID"])
            .output()
            .map_err(|e| e.to_string())?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let drives: Vec<DriveInfo> = stdout
            .lines()
            .skip(1)
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let path = line.trim().to_string();
                DriveInfo {
                    name: path.clone(),
                    path: path.clone(),
                    is_removable: true,
                    total_space: 0,
                    free_space: 0,
                }
            })
            .collect();
        
        Ok(drives)
    }
    
    #[cfg(target_os = "macos")]
    {
        let volumes_path = PathBuf::from("/Volumes");
        if volumes_path.exists() {
            let drives: Vec<DriveInfo> = std::fs::read_dir(&volumes_path)
                .map_err(|e| e.to_string())?
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            Some(DriveInfo {
                                name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                path: path.to_string_lossy().to_string(),
                                is_removable: true,
                                total_space: 0,
                                free_space: 0,
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect();
            Ok(drives)
        } else {
            Ok(vec![])
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        let media_path = PathBuf::from("/media");
        let mnt_path = PathBuf::from("/mnt");
        
        let mut drives = Vec::new();
        
        if media_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&media_path) {
                drives.extend(entries.filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            Some(DriveInfo {
                                name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                path: path.to_string_lossy().to_string(),
                                is_removable: true,
                                total_space: 0,
                                free_space: 0,
                            })
                        } else {
                            None
                        }
                    })
                }));
            }
        }
        
        if mnt_path.exists() {
            if let Ok(entries) = std::fs::read_dir(&mnt_path) {
                drives.extend(entries.filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            Some(DriveInfo {
                                name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                                path: path.to_string_lossy().to_string(),
                                is_removable: true,
                                total_space: 0,
                                free_space: 0,
                            })
                        } else {
                            None
                        }
                    })
                }));
            }
        }
        
        Ok(drives)
    }
}
