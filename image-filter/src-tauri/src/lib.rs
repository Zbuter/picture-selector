use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFile {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub thumbnail: Option<String>, // base64 encoded thumbnail
}

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub watched_path: std::sync::Arc<std::sync::Mutex<Option<PathBuf>>>,
}

#[tauri::command]
async fn select_folder(app: AppHandle) -> Result<Vec<ImageFile>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let (tx, rx) = std::sync::mpsc::channel();
    
    app.dialog()
        .file()
        .pick_folder(move |folder| {
            tx.send(folder.map(|p| p.to_string())).ok();
        });
    
    let folder_path = rx.recv().map_err(|e| e.to_string())?.ok_or("No folder selected")?;
    
    load_images_from_folder(&folder_path)
}

#[tauri::command]
fn load_images_from_folder(folder_path: &str) -> Result<Vec<ImageFile>, String> {
    let path = PathBuf::from(folder_path);
    
    if !path.exists() || !path.is_dir() {
        return Err("Invalid folder path".to_string());
    }
    
    let supported_extensions = [
        "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif",
        "arw", "nef", "cr2", "cr3", "dng", "raf", "orf", "pef",
        "srw", "rw2", "iiq", "3fr", "fff", "mos", "mef", "kdc",
    ];
    
    let mut images = Vec::new();
    
    for entry in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();
        
        if file_path.is_file() {
            if let Some(ext) = file_path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if supported_extensions.contains(&ext_lower.as_str()) {
                    let image = create_image_file(file_path)?;
                    images.push(image);
                }
            }
        }
    }
    
    Ok(images)
}

#[tauri::command]
fn list_images_in_folder(folder_path: &str) -> Result<Vec<ImageFile>, String> {
    load_images_from_folder(folder_path)
}

fn create_image_file(path: PathBuf) -> Result<ImageFile, String> {
    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let extension = path.extension().unwrap_or_default().to_string_lossy().to_string();
    let size = metadata.len();
    
    // Generate thumbnail for preview
    let thumbnail = generate_thumbnail(&path).ok();
    
    Ok(ImageFile {
        path: path.to_string_lossy().to_string(),
        name,
        extension,
        size,
        thumbnail,
    })
}

fn generate_thumbnail(path: &PathBuf) -> Result<String, String> {
    // Try to decode the image and create a thumbnail
    let img = image::open(path).map_err(|e| format!("Failed to open image: {}", e))?;
    let thumbnail = img.thumbnail(200, 200);
    
    // Convert to PNG and encode as base64
    let mut buffer = Vec::new();
    thumbnail
        .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
        .map_err(|e| format!("Failed to write thumbnail: {}", e))?;
    
    let base64_data = base64_encode(&buffer);
    Ok(format!("data:image/png;base64,{}", base64_data))
}

fn base64_encode(data: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.encode(data)
}

#[tauri::command]
fn get_image_preview(path: &str) -> Result<String, String> {
    let img_path = PathBuf::from(path);
    
    // For RAW files, we'll try to extract embedded JPEG preview if available
    let ext = img_path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
    
    match ext.as_str() {
        "arw" | "nef" | "cr2" | "cr3" | "dng" | "raf" | "orf" | "pef" | "srw" | "rw2" => {
            // Try to extract embedded preview from RAW file
            extract_raw_preview(&img_path)
        }
        _ => {
            // Regular image - generate full preview
            let img = image::open(&img_path).map_err(|e| format!("Failed to open image: {}", e))?;
            let mut buffer = Vec::new();
            img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
                .map_err(|e| format!("Failed to encode image: {}", e))?;
            Ok(format!("data:image/png;base64,{}", base64_encode(&buffer)))
        }
    }
}

fn extract_raw_preview(path: &PathBuf) -> Result<String, String> {
    // Try to open as regular image first (some RAW files have embedded previews)
    if let Ok(img) = image::open(path) {
        let mut buffer = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
            .map_err(|e| format!("Failed to encode image: {}", e))?;
        return Ok(format!("data:image/png;base64,{}", base64_encode(&buffer)));
    }
    
    // If that fails, try to read EXIF data for embedded preview
    // This is a simplified approach - in production you might want more robust RAW handling
    Err("Cannot preview RAW file without embedded preview".to_string())
}

#[tauri::command]
async fn copy_selected_images(
    source_paths: Vec<String>,
    dest_folder: String,
) -> Result<Vec<String>, String> {
    let dest_path = PathBuf::from(&dest_folder);
    
    if !dest_path.exists() {
        fs::create_dir_all(&dest_path).map_err(|e| e.to_string())?;
    }
    
    let mut copied_files = Vec::new();
    
    for source_path in source_paths {
        let src = PathBuf::from(&source_path);
        let filename = src.file_name().unwrap_or_default();
        let dst = dest_path.join(filename);
        
        fs::copy(&src, &dst).map_err(|e| {
            format!("Failed to copy {} to {}: {}", source_path, dest_folder, e)
        })?;
        
        copied_files.push(dst.to_string_lossy().to_string());
    }
    
    Ok(copied_files)
}

#[tauri::command]
fn start_watching_folder(
    app: AppHandle,
    state: State<AppState>,
    folder_path: String,
) -> Result<(), String> {
    let path = PathBuf::from(&folder_path);
    
    if !path.exists() || !path.is_dir() {
        return Err("Invalid folder path".to_string());
    }
    
    // Store the watched path
    *state.watched_path.lock().unwrap() = Some(path.clone());
    
    let (tx, rx) = channel();
    
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                // Check if it's a create or modify event for image files
                for event_path in &event.paths {
                    if let Some(ext) = event_path.extension() {
                        let ext_lower = ext.to_string_lossy().to_lowercase();
                        let supported_extensions = [
                            "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif",
                            "arw", "nef", "cr2", "cr3", "dng", "raf", "orf", "pef",
                            "srw", "rw2", "iiq", "3fr", "fff", "mos", "mef", "kdc",
                        ];
                        
                        if supported_extensions.contains(&ext_lower.as_str()) {
                            let _ = app.emit("file-changed", {
                                let mut map = std::collections::HashMap::new();
                                map.insert("type", format!("{:?}", event.kind));
                                map.insert("path", event_path.to_string_lossy().to_string());
                                map
                            });
                        }
                    }
                }
            }
        },
        Config::default(),
    )
    .map_err(|e| e.to_string())?;
    
    watcher
        .watch(&path, RecursiveMode::Recursive)
        .map_err(|e| e.to_string())?;
    
    // Store watcher in app state (simplified - in production you'd want proper lifetime management)
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_millis(100));
            if let Ok(_) = rx.try_recv() {
                // Events are handled in the callback
            }
        }
    });
    
    Ok(())
}

#[tauri::command]
fn detect_external_drives() -> Result<Vec<String>, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("wmic")
            .args(&["logicaldisk", "where", "DriveType=2", "get", "DeviceID"])
            .output()
            .map_err(|e| e.to_string())?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let drives: Vec<String> = stdout
            .lines()
            .skip(1) // Skip header
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();
        
        Ok(drives)
    }
    
    #[cfg(target_os = "macos")]
    {
        let volumes_path = PathBuf::from("/Volumes");
        if volumes_path.exists() {
            let drives: Vec<String> = fs::read_dir(&volumes_path)
                .map_err(|e| e.to_string())?
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            Some(path.to_string_lossy().to_string())
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
            if let Ok(entries) = fs::read_dir(&media_path) {
                drives.extend(entries.filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_dir() {
                            Some(path.to_string_lossy().to_string())
                        } else {
                            None
                        }
                    })
                }));
            }
        }
        
        if mnt_path.exists() {
            if let Ok(entries) = fs::read_dir(&mnt_path) {
                drives.extend(entries.filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_dir() && !drives.contains(&path.to_string_lossy().to_string()) {
                            Some(path.to_string_lossy().to_string())
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

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            select_folder,
            load_images_from_folder,
            list_images_in_folder,
            get_image_preview,
            copy_selected_images,
            start_watching_folder,
            detect_external_drives,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
