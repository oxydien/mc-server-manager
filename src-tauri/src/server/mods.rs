use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path};
use serde_json::{Value, json};
use reqwest;

use crate::files::get::{get_server_folder};

pub async fn add_mod_to_server(server_id: &str, url: &str, mod_info: &str) -> Result<String, String> {
    let server_folder = get_server_folder(server_id);
    let mods_folder = server_folder.join("mods");

    fs::create_dir_all(&mods_folder).map_err(|e| e.to_string())?;
    let mod_file_name = get_filename_from_url(url).map_err(|e| e.to_string())?;
    let mod_file_path = mods_folder.join(&mod_file_name);
    download_file(url, &mod_file_path).await.map_err(|e| e.to_string())?;

    let server_info_path = server_folder.join("server-info.json");
    let mut server_info = read_server_info(&server_info_path).map_err(|e| e.to_string())?;

    let mod_info_value: Value = serde_json::from_str(mod_info).map_err(|e| e.to_string())?;

    if let Some(mods) = server_info.get_mut("mods") {
        if let Some(mods_array) = mods.as_array_mut() {
            mods_array.push(mod_info_value);
        }
    } else {
        server_info["mods"] = json!([mod_info_value]);
    }

    write_server_info(&server_info_path, &server_info).map_err(|e| e.to_string())?;

    Ok("[success]: Downloaded mod".to_string())
}

fn get_filename_from_url(url: &str) -> Result<String,String> {
    let parsed_url = url::Url::parse(url).map_err(|e| e.to_string())?;
    let path_segments = parsed_url.path_segments().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid URL")
    }).map_err(|e| e.to_string())?;

    if let Some(last_segment) = path_segments.last() {
        Ok(last_segment.to_string())
    } else {
        Err("[get-file-name-url]: Invalid URL".to_string())
    }
}

async fn download_file(url: &str, dest_path: &Path) -> Result<(), String> {
  let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
  let mut dest_file = fs::File::create(dest_path).map_err(|e| e.to_string())?;

  let content = response.bytes().await.map_err(|e| e.to_string())?;
  dest_file.write_all(&content).map_err(|e| e.to_string())?;

  Ok(())
}

fn read_server_info(path: &Path) -> io::Result<Value> {
    let mut file = fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let server_info: Value = serde_json::from_str(&data)?;
    Ok(server_info)
}

fn write_server_info(path: &Path, server_info: &Value) -> io::Result<()> {
    let data = serde_json::to_string_pretty(&server_info)?;
    let mut file = fs::File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn get_server_mods(path: &Path) -> io::Result<Vec<Value>> {
    let server_info: Value = read_server_info(path.join("server-info.json").as_path()).unwrap();
    
    if let Some(mods) = server_info["mods"].as_array() {
        Ok(mods.to_owned())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid mods data"))
    }
}
