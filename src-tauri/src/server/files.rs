use crate::files::get::{get_app_folder, get_server_folder};
use crate::server::structs::FileInfo;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use chrono::{DateTime, Local};
use std::path::Path;

pub fn get_file_list(server_id: &str, path: Option<&str>) -> Result<String, String> {
    let mut server_path = get_server_folder(server_id);

    if let Some(sub_path) = path {
        println!(
            "[get-files]: {}:{}",
            server_path.clone().display(),
            sub_path
        );
        server_path = server_path.join(sub_path);
    }

    let mut file_list: Vec<FileInfo> = Vec::new();

    if let Ok(entries) = fs::read_dir(&server_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                let file_type = if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    "directory".to_string()
                } else {
                    match Path::new(&file_name).extension() {
                        Some(extension) => extension.to_string_lossy().to_string(),
                        None => "unknown".to_string(),
                    }
                };

                let created = entry.metadata().map_or("unknown".to_string(), |m| {
                    m.created()
                        .map_or("unknown".to_string(), |ct| {
                            DateTime::<Local>::from(ct).to_string()
                        })
                });

                let modified = entry.metadata().map_or("unknown".to_string(), |m| {
                    m.modified()
                        .map_or("unknown".to_string(), |mt| {
                            DateTime::<Local>::from(mt).to_string()
                        })
                });

                let size_bytes = entry.metadata().map_or(0, |m| m.len());

                let file_info = FileInfo {
                    name: file_name,
                    file_type,
                    created,
                    modified,
                    size_bytes,
                };

                file_list.push(file_info);
            }
        }

        match serde_json::to_string(&file_list) {
            Ok(json_data) => Ok(json_data),
            Err(err) => Err(format!("Failed to serialize file list: {}", err)),
        }
    } else {
        Err(format!(
            "Failed to read directory: {}",
            server_path.display()
        ))
    }
}

pub fn read_file(file_path: &str, server_id: &str) -> Result<String, String> {
    let full_path = get_server_folder(server_id).join(file_path);
    println!("[read-file]: {}", full_path.clone().display());

    let mut contents = String::new();
    match fs::File::open(&full_path) {
        Ok(mut file) => {
            if let Err(err) = file.read_to_string(&mut contents) {
                return Err(format!("Error reading file: {}", err));
            }
            Ok(contents)
        }
        Err(err) => Err(format!("Error opening file: {}", err)),
    }
}

pub fn write_file(file_path: &str, contents: &str, server_id: &str) -> Result<(), String> {
    let server_dir = get_server_folder(server_id);
    let full_path = server_dir.join(file_path);

    match fs::File::create(&full_path) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(contents.as_bytes()) {
                return Err(format!("Error writing file: {}", err));
            }
            Ok(())
        }
        Err(err) => Err(format!("Error creating file: {}", err)),
    }
}

#[tauri::command]
pub async fn open_file_or_explorer(server_id: &str, path: &str) -> Result<(), String> {
    let server_dir: std::path::PathBuf = get_server_folder(server_id).join(path);
    Ok(open::that(server_dir).unwrap())
}

#[tauri::command]
pub fn read_server_properties(server_id: &str) -> Result<String, String> {
    let server_dir = format!(
        "{}/servers/{}/server.properties",
        get_app_folder().display(),
        server_id
    );
    let mut file = File::open(server_dir).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let mut properties: HashMap<String, String> = HashMap::new();
    for line in contents.lines() {
        if !line.starts_with('#') && !line.is_empty() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                properties.insert(key, value);
            }
        }
    }

    let json_data = serde_json::json!(properties);
    let json_string = serde_json::to_string(&json_data).map_err(|e| e.to_string())?;

    Ok(json_string)
}

#[tauri::command]
pub fn write_server_properties(server_id: &str, json_string: &str) -> Result<(), String> {
    let server_dir = format!(
        "{}/servers/{}/server.properties",
        get_app_folder().display(),
        server_id
    );
    let mut file = File::create(&server_dir).map_err(|e| e.to_string())?;

    let json_data: serde_json::Value =
        serde_json::from_str(json_string).map_err(|e| e.to_string())?;
    if let Some(properties) = json_data.as_object() {
        for (key, value) in properties {
            if let Some(value_str) = value.as_str() {
                let line = format!("{}={}\n", key, value_str);
                file.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}
