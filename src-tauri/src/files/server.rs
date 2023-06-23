use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
pub fn get_servers_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Failed to get the data directory")
        .join("mc-svr-mng")
}

#[cfg(target_os = "linux")]
const SERVERS_DIR: &str = "/etc/mc-svr-mng";

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    id: String,
    name: String,
    server_type: String,
    mc_version: String,
    mods: Option<Vec<Mod>>,
    image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mod {
    name: String,
    version: String,
}

fn update_servers_json(server_info: &serde_json::Value) {
    let servers_dir = get_servers_dir();
    let servers_file = servers_dir.join("servers.json");
    let servers_data = if servers_file.exists() {
        let data = fs::read_to_string(&servers_file).expect("Failed to read servers.json");
        serde_json::from_str::<serde_json::Value>(&data).expect("Failed to parse servers.json")
    } else {
        serde_json::json!({"servers": []})
    };

    let _servers_array = servers_data
        .get("servers")
        .expect("Invalid servers.json structure")
        .as_array()
        .expect("Invalid servers.json structure")
        .clone();

    let mut updated_servers_data = servers_data.clone();
    let mut _updated_servers_array = updated_servers_data
        .get_mut("servers")
        .expect("Invalid servers.json structure")
        .as_array_mut()
        .expect("Invalid servers.json structure");

    _updated_servers_array.push(server_info.clone());

    fs::write(
        &servers_file,
        serde_json::to_string_pretty(&updated_servers_data).unwrap(),
    )
    .expect("Failed to update servers.json");
}

fn create_server_folder(server_name: &str) -> PathBuf {
    let sanitized_server_name = sanitize_folder_name(server_name);
    let servers_dir = get_servers_dir();
    let server_folder = servers_dir.join("servers").join(&sanitized_server_name);

    fs::create_dir_all(&server_folder).expect("Failed to create server folder");

    server_folder
}

fn create_server_info_json(
    server_folder: &Path,
    id: String,
    server_name: &str,
    mc_version: &str,
    server_type: &str,
    image: Option<&str>,
) {
    let server_info_file = server_folder.join("server-info.json");
    let server_info_data = json!({
        "id": id,
        "name": server_name,
        "server_type": server_type,
        "mc_version": mc_version,
        "mods": [],
        "image": image,
    });

    let server_info_json = serde_json::to_string_pretty(&server_info_data).unwrap();
    fs::write(&server_info_file, server_info_json).expect("Failed to create server-info.json");
}

async fn handle_image(server_folder: &Path, image: Option<&str>) {
    if let Some(image_str) = image {
        if image_str.starts_with("data:image/") {
            let image_bytes = general_purpose::STANDARD.decode(image_str.as_bytes());
            if let Ok(image_bytes) = image_bytes {
                fs::write(server_folder.join("icon.png"), &image_bytes)
                    .expect("Failed to save base64 image as icon.png");
            } else {
                eprintln!("Failed to decode base64 image");
            }
        } else if image_str.starts_with("http://") || image_str.starts_with("https://") {
            // Image is a URL
            let image_url = image_str.to_string();
            let image_path = server_folder.join("icon.png");
            if let Err(err) = download_image(&image_url, &image_path).await {
                eprintln!("Failed to download image: {}", err);
            }
        } else {
            // Image is a file path
            if let Ok(image_path) = Path::new(image_str).canonicalize() {
                let image_file_name = image_path.file_name().and_then(|name| name.to_str());
                if let Some(_image_file_name) = image_file_name {
                    let destination_path = server_folder.join("icon.png");
                    if let Err(err) = fs::copy(&image_path, &destination_path) {
                        eprintln!("Failed to copy image file to server folder: {}", err);
                    }
                } else {
                    println!("Invalid image file name");
                }
            } else {
                println!("Invalid image file path");
            }
        }
    }
}

async fn download_image(
    url: &str,
    destination_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut dest_file = File::create(destination_path)?;
    let mut _response_body = response.bytes().await?;
    std::io::copy(&mut _response_body.as_ref(), &mut dest_file)?;
    Ok(())
}

pub async fn create_server(server_name: &str, mc_version: &str, server_type: &str, image: Option<&str>) {
    let sanitized_name = sanitize_folder_name(server_name);

    let servers_dir = get_servers_dir();
    let servers_file = servers_dir.join("servers.json");

    let servers_data = if servers_file.exists() {
        let data = fs::read_to_string(&servers_file).expect("Failed to read servers.json");
        serde_json::from_str::<serde_json::Value>(&data).expect("Failed to parse servers.json")
    } else {
        serde_json::json!({"servers": []})
    };

    let _servers_array = servers_data
        .get("servers")
        .expect("Invalid servers.json structure")
        .as_array()
        .expect("Invalid servers.json structure");

    let server_folder = create_server_folder(&sanitized_name);
    handle_image(&server_folder, image).await;

    let server_info = json!({
        "id": sanitize_folder_name(server_name),
        "name": server_name,
        "mc_version": mc_version,
        "server_type": server_type,
        "image": image,
    });

    update_servers_json(&server_info);

    create_server_info_json(
        &server_folder,
        sanitize_folder_name(server_name).to_string(),
        server_name,
        mc_version,
        server_type,
        image,
    );
}

pub fn sanitize_folder_name(name: &str) -> String {
    // Remove special characters and convert to lowercase
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase()
}

pub fn read_servers_json() -> Result<Vec<Server>, String> {
    #[cfg(target_os = "windows")]
    let file_path = get_servers_dir().join("servers.json");

    #[cfg(target_os = "linux")]
    let file_path = Path::new(SERVERS_DIR).join("servers.json");

    let path = Path::new(&file_path);


    if !path.exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)
            .map_err(|e| e.to_string())?;

        let initial_content = r#"{"servers":[]}"#;
        file.write_all(initial_content.as_bytes())
            .map_err(|e| e.to_string())?;

        return Ok(Vec::new());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .map_err(|e| e.to_string())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|e| e.to_string())?;

    let servers = json["servers"]
        .as_array()
        .map_or_else(
            || Ok(Vec::new()),
            |arr| {
                arr.iter()
                    .map(|server| {
                        serde_json::from_value::<Server>(server.clone())
                            .map_err(|e| e.to_string())
                    })
                    .collect()
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(servers)
}

pub fn get_file_loc() -> Result<String, String> {
    let servers_dir = get_servers_dir();
    if let Some(path_str) = servers_dir.to_str() {
        Ok(path_str.to_owned())
    } else {
        Err("Failed to convert file location to string".to_owned())
    }
}


pub fn update_server_field(server_id: &str, field: &str, value: &str) -> Result<(), String> {
    let servers_dir = get_servers_dir();
    let servers_json_path = servers_dir.join("servers.json");

    let servers_json_content = match fs::read_to_string(&servers_json_path) {
        Ok(content) => content,
        Err(err) => return Err(format!("Failed to read servers.json: {}", err)),
    };

    let mut servers_json: serde_json::Value = match serde_json::from_str(&servers_json_content) {
        Ok(json) => json,
        Err(err) => return Err(format!("Failed to parse servers.json: {}", err)),
    };

    let server_index = servers_json["servers"]
        .as_array()
        .and_then(|arr| arr.iter().position(|obj| obj["id"] == server_id))
        .ok_or_else(|| format!("Server ID '{}' not found in servers.json", server_id))?;

    servers_json["servers"][server_index][field] = json!(value);

    let updated_servers_json_content = match serde_json::to_string_pretty(&servers_json) {
        Ok(content) => content,
        Err(err) => return Err(format!("Failed to serialize updated servers.json: {}", err)),
    };

    match fs::write(&servers_json_path, updated_servers_json_content) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Failed to write updated servers.json: {}", err)),
    }
}

pub fn remove_server(server_id: &str) -> Result<(),String> {
    let servers_dir = get_servers_dir();
    let server_folder = servers_dir.join("servers").join(server_id);

    fs::remove_dir_all(server_folder).map_err(|e| e.to_string())?;

    let servers_json_path = servers_dir.join("servers.json");
    let mut servers_data = read_servers_data(&servers_json_path).map_err(|e| e.to_string())?;

    if let Some(servers) = servers_data.get_mut("servers") {
        if let Some(servers_array) = servers.as_array_mut() {
            servers_array.retain(|server| {
                if let Some(id) = server.get("id") {
                    if let Some(id_string) = id.as_str() {
                        return id_string != server_id;
                    }
                }
                true
            });
        }
    }

    write_servers_data(&servers_json_path, &servers_data).map_err(|e| e.to_string())?;

    Ok(())
}

fn read_servers_data(path: &Path) -> std::io::Result<serde_json::Value> {
    let mut file = fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let servers_data: serde_json::Value = serde_json::from_str(&data)?;
    Ok(servers_data)
}

fn write_servers_data(path: &Path, servers_data: &serde_json::Value) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(&servers_data)?;
    let mut file = fs::File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}