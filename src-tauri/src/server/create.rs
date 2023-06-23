use std::fs::{self, File};
use std::io::{ Write};
use std::process::{Command};
use std::path::{Path, PathBuf};
use reqwest::{Client,Response};
use semver::Version;
use regex::Regex;
use serde::de::DeserializeOwned;

#[cfg(target_os = "windows")]
fn get_servers_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Failed to get the data directory")
        .join("mc-svr-mng")
}

#[cfg(target_os = "linux")]
const SERVERS_DIR: &str = "/etc/mc-svr-mng";

async fn get_json<T: DeserializeOwned>(response: Response) -> Result<T, Box<dyn std::error::Error>> {
    let text = response.text().await?;
    let json = serde_json::from_str(&text)?;
    Ok(json)
}

pub async fn download_minecraft_server(server_folder: &Path, minecraft_version: &str) -> Result<(), String> {
    let version_manifest_url = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";
    let response = reqwest::get(version_manifest_url).await.map_err(|e| e.to_string())?;
    let version_manifest: serde_json::Value = get_json(response).await.map_err(|e| e.to_string())?;
    let version_info = version_manifest["versions"]
        .as_array()
        .and_then(|versions| {
            versions
                .iter()
                .find(|version| version["id"].as_str() == Some(minecraft_version))
        })
        .and_then(|version| version["url"].as_str())
        .ok_or("[download-minecraft-server]: Failed to find version information")?;
    println!("[download-minecraft-server]: Found version");
    let response = reqwest::get(version_info).await.map_err(|e| e.to_string())?;
    let version_data: serde_json::Value = get_json(response).await.map_err(|e| e.to_string())?;
    let server_download_url = version_data["downloads"]["server"]["url"]
        .as_str()
        .ok_or("[download-minecraft-server]: Failed to find server download URL").map_err(|e| e.to_string())?;
    
    println!("[download-minecraft-server]: Download URL, Downloading");
    let server_path = server_folder.join("server.jar");
    let mut server_file = File::create(&server_path).map_err(|e| e.to_string())?;
    let server_bytes = reqwest::get(server_download_url).await.map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;
    server_file.write_all(&server_bytes).map_err(|e| e.to_string())?;
    println!("[download-minecraft-server]: Saved");

    Ok(())
}

pub async fn check_and_update_quilt_installer() -> Result<(), String> {
    let repository_url = "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-installer/";
    let client = Client::new();
    let response = client.get(repository_url).send().await.map_err(|e| e.to_string())?;
    let body = response.text().await.map_err(|e| e.to_string())?;
    let available_versions: Vec<Version> = extract_versions(&body);

    if let Some(latest_version) = available_versions.iter().max() {
        let installed_version = get_installed_quilt_installer_version();

        if installed_version.is_none() || installed_version.unwrap() < *latest_version {
            println!("[check-quilt-installer]: Downloading Quilt installer version {}...", latest_version);

            let download_url = format!(
                "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-installer/{}/quilt-installer-{}.jar",
                latest_version, latest_version
            );

            let installer_bytes = client.get(&download_url).send().await.map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;

            let servers_dir = get_servers_dir();
            let libs_dir = servers_dir.join("libs");
            fs::create_dir_all(&libs_dir).map_err(|e| e.to_string())?;

            let installer_path = libs_dir.join(format!("quilt-installer-{}.jar", latest_version));
            let mut installer_file = fs::File::create(&installer_path).map_err(|e| e.to_string())?;
            installer_file.write_all(&installer_bytes).map_err(|e| e.to_string())?;

            println!("[check-quilt-installer]: Quilt installer downloaded successfully.");
        } else {
            println!("[check-quilt-installer]: Quilt installer is up to date.");
        }
    } else {
        println!("[check-quilt-installer]: No Quilt installer versions found.");
    }

    Ok(())
}

pub async fn check_and_update_fabric_installer() -> Result<(), String> {
    let repository_url = "https://maven.fabricmc.net/net/fabricmc/fabric-installer/";
    let client = Client::new();
    let response = client.get(repository_url).send().await.map_err(|e| e.to_string())?;
    let body = response.text().await.map_err(|e| e.to_string())?;
    let available_versions: Vec<Version> = extract_versions(&body);

    if let Some(latest_version) = available_versions.iter().max() {
        let installed_version = get_installed_fabric_installer_version();

        if installed_version.is_none() || installed_version.unwrap() < *latest_version {
            println!("[check-fabric-installer]: Downloading Fabric installer version {}...", latest_version);

            let download_url = format!(
                "https://maven.fabricmc.net/net/fabricmc/fabric-installer/{}/fabric-installer-{}.jar",
                latest_version, latest_version
            );

            let installer_bytes = client.get(&download_url).send().await.map_err(|e| e.to_string())?.bytes().await.map_err(|e| e.to_string())?;

            let servers_dir = get_servers_dir();
            let libs_dir = servers_dir.join("libs");
            fs::create_dir_all(&libs_dir).map_err(|e| e.to_string())?;

            let installer_path = libs_dir.join(format!("fabric-installer-{}.jar", latest_version));
            let mut installer_file = fs::File::create(&installer_path).map_err(|e| e.to_string())?;
            installer_file.write_all(&installer_bytes).map_err(|e| e.to_string())?;

            println!("[check-fabric-installer]: Fabric installer downloaded successfully.");
        } else {
            println!("[check-fabric-installer]: Fabric installer is up to date.");
        }
    } else {
        println!("[check-fabric-installer]: No Fabric installer versions found.");
    }

    Ok(())
}

pub fn extract_versions(html: &str) -> Vec<Version> {
    let version_regex = Regex::new(r#"<a href="([\d\.]+)/">([\d\.]+)/</a>"#).unwrap();
    let mut versions = Vec::new();

    for captures in version_regex.captures_iter(html) {
        if let (Some(version_str), Some(_)) = (captures.get(1), captures.get(2)) {
            if let Ok(version) = Version::parse(version_str.as_str()) {
                versions.push(version);
            }
        }
    }
    versions
}

pub fn get_installed_quilt_installer_version() -> Option<Version> {
    let servers_dir = get_servers_dir();
    let libs_dir = servers_dir.join("libs");
    let installer_files = fs::read_dir(libs_dir.clone())
        .unwrap_or_else(|_| panic!("[get-quilt-installer]: Failed to read the libs directory: {:?}", libs_dir));

    let installer_version_regex = Regex::new(r#"quilt-installer-(\d+\.\d+\.\d+)\.jar"#).unwrap();
    let mut installed_versions = Vec::new();

    for entry in installer_files {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if let Some(captures) = installer_version_regex.captures(file_name) {
                    if let Some(version_str) = captures.get(1) {
                        if let Ok(version) = Version::parse(version_str.as_str()) {
                            installed_versions.push(version);
                        }
                    }
                }
            }
        }
    }

    installed_versions.into_iter().max()
}

pub fn get_installed_fabric_installer_version() -> Option<Version> {
    let servers_dir = get_servers_dir();
    let libs_dir = servers_dir.join("libs");
    let installer_files = fs::read_dir(libs_dir.clone())
        .unwrap_or_else(|_| panic!("[get-fabric-installer]: Failed to read the libs directory: {:?}", libs_dir));

    let installer_version_regex = Regex::new(r#"fabric-installer-(\d+\.\d+\.\d+)\.jar"#).unwrap();
    let mut installed_versions = Vec::new();

    for entry in installer_files {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if let Some(captures) = installer_version_regex.captures(file_name) {
                    if let Some(version_str) = captures.get(1) {
                        if let Ok(version) = Version::parse(version_str.as_str()) {
                            installed_versions.push(version);
                        }
                    }
                }
            }
        }
    }

    installed_versions.into_iter().max()
}

pub fn run_quilt_installer(
    server_path: &Path,
    installer_version: &Version,
    minecraft_version: &str,
) -> Result<(), String> {
    let installer_file = format!("quilt-installer-{}.jar", installer_version);
    let installer_path = get_servers_dir().join("libs").join(&installer_file);

    let mut command = Command::new("java");
    command
        .arg("-jar")
        .arg(&installer_path)
        .arg("install")
        .arg("server")
        .arg(minecraft_version)
        .arg("--create-scripts")
        .arg("--install-dir=".to_owned() + server_path.to_str().unwrap())
        .current_dir(server_path);

    println!("Running Quilt installer...");
    let status = command.status().map_err(|err| format!("[running quilt installer]: {}", err))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("[running quilt installer]: Quilt installer failed"))
    }
}

pub fn run_fabric_installer(
    server_path: &Path,
    installer_version: &Version,
    minecraft_version: &str,
) -> Result<(), String> {
    let installer_file = format!("fabric-installer-{}.jar", installer_version);
    let installer_path = get_servers_dir().join("libs").join(&installer_file);

    let mut command = Command::new("java");
    command
        .arg("-jar")
        .arg(&installer_path)
        .arg("server")
        .arg("-mcversion")
        .arg(minecraft_version)
        .arg("-dir")
        .arg(server_path.to_str().unwrap())
        .current_dir(server_path);

    println!("Running fabric installer...");
    let status = command.status().map_err(|err| format!("[running fabric installer]: {}", err))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("[running fabric installer]: Fabric installer failed"))
    }
}

pub async fn accept_eula(server_path: &Path) -> Result<(), String> {
    let mut eula_file = fs::File::create(server_path.join("eula.txt")).map_err(|e| e.to_string())?;
    eula_file.write_all("#By changing the setting below to TRUE you are indicating your agreement to minecraft's EULA (https://account.mojang.com/documents/minecraft_eula).\neula=true".as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}