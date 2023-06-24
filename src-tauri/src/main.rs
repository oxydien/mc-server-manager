// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod files;
mod server;
mod config;
use serde_json;
use std::fs;
use std::path::{Path,PathBuf};

fn get_servers_dir() -> PathBuf {
    let data_dir = dirs::data_dir().expect("Failed to get the data directory");
    let server_dir = data_dir.join("mc-svr-mng");
    let libs_dir = server_dir.join("libs");

    if !libs_dir.exists() {
        fs::create_dir_all(&libs_dir).expect("Failed to create libs directory");
    }
    server_dir
}

#[tauri::command]
fn list_servers() -> Result<String, String> {
    let servers = files::server::read_servers_json()?;
    let json = serde_json::to_string(&servers).map_err(|e| e.to_string())?;
    Ok(json)
}

#[tauri::command]
async fn add_server_command(
    name: &str,
    server_type: &str,
    mc_version: &str,
    image: Option<&str>,
) -> Result<String, String> {
    println!(
        "[add-server-command]: Creating new server: {} | {} | {} ",
        name.to_owned(),
        server_type.to_owned(),
        mc_version.to_owned()
    );
    files::server::create_server(name, mc_version, server_type, image).await;
    let server_path = get_servers_dir()
        .join("servers")
        .join(files::server::sanitize_folder_name(name));
    let mc_version_clone = mc_version.to_owned(); // Clone mc_version

    server::create::accept_eula(server_path.clone().as_path())
        .await
        .map_err(|e| e.to_string())?;
    match server_type {
        "quilt" => {
            println!("[add-server-command]: Trying to update quilt installer");
            server::create::check_and_update_quilt_installer().await?;
            println!("[add-server-command]: Finding quilt installer version");
            let installed_installer_version =
                server::create::get_installed_quilt_installer_version()
                    .ok_or("Installed installer version not found")?;
            println!(
                "[add-server-command]: Running newest quilt installer {}",
                installed_installer_version.clone()
            );
            server::create::run_quilt_installer(
                &server_path,
                &installed_installer_version,
                &mc_version_clone,
            )?;
        }
        "fabric" => {
            println!("[add-server-command]: Trying to update fabric installer");
            server::create::check_and_update_fabric_installer().await?;
            println!("[add-server-command]: Finding fabric installer version");
            let installed_installer_version =
                server::create::get_installed_fabric_installer_version()
                    .ok_or("Installed installer version not found")?;
            println!(
                "[add-server-command]: Running newest fabric installer {}",
                installed_installer_version.clone()
            );
            server::create::run_fabric_installer(
                &server_path,
                &installed_installer_version,
                &mc_version_clone,
            )?;
        }
        _ => {}
    }

    println!("[add-server-command]: Downloading Minecraft server");
    server::create::download_minecraft_server(&server_path, &mc_version_clone).await?;

    println!("[add-server-command]: Server created successfully.");
    Ok(format!("[success]: Server {} created successfully.", name))
}

#[tauri::command]
fn get_file_loc_command() -> Result<String, String> {
    Ok(files::server::get_file_loc().unwrap())
}

#[tauri::command]
fn get_server_info_command(server_id: &str) -> Result<String, String> {
    Ok(server::info::get_server_info(server_id).unwrap())
}

#[tauri::command]
async fn start_server_command(server_id: String, server_type: String) -> Result<bool, String> {
    Ok(server::run::run_server_and_save_output(&server_id, &server_type).await.unwrap())
}

#[tauri::command]
fn get_files_command(server_id: &str, path: Option<&str>) -> Result<String, String> {
    Ok(server::files::get_file_list(server_id, path).unwrap())
}

#[tauri::command]
fn remove_server_commamd(server_id: &str) -> Result<(), String> {
    Ok(files::server::remove_server(server_id).unwrap())
}

#[tauri::command]
fn get_mods_as_string(server_id: &str) -> Result<String, String> {
    let servers_dir = get_servers_dir();
    let server_folder = servers_dir.join("servers").join(server_id);
    let server_info = server::mods::get_server_mods(server_folder.as_path()).map_err(|e| e.to_string())?;
    let mods_string = serde_json::to_string_pretty(&server_info).map_err(|e| e.to_string())?;
    Ok(mods_string)
}

#[tauri::command]
async fn download_mod_command(server_id: &str, mod_url: &str, mod_info: &str) -> Result<String, String> {
    Ok(server::mods::add_mod_to_server(server_id, mod_url, mod_info).await?)
}

#[tauri::command]
async fn execute_command_on_server(command: &str) -> Result<(), String> {
    //Ok(server::run::send_command_to_server(command).await?)
    Some(command);
    Ok(())
}

#[tauri::command]
async fn kill_server_command(server_id: String) -> Result<(),String> {
    Ok(server::run::kill_server_process(&server_id).await.unwrap())
}

#[tauri::command]
fn get_server_log(server_id: &str) -> Result<String, String> {
    Ok(server::log::get_server_log(server_id).unwrap())
}

#[tauri::command]
fn get_config_command() -> Result<String, String> {
    Ok(config::get::get_settings().unwrap())
}

#[tauri::command]
fn set_config_command(name: &str, value: &str) -> Result<(),String> {
    Ok(config::set::update_setting(name,value).unwrap())
}

#[tauri::command]
async fn get_server_status_command(server_id: &str) -> Result<bool,String> {
    Ok(server::info::check_server_status(server_id).await.unwrap())
}

#[tauri::command]
fn get_server_file(file_path: &str, server_id: &str) -> Result<String,String> {
    Ok(server::files::read_file(file_path, server_id).unwrap())
}

#[tauri::command]
fn set_server_file(file_path: &str, contents: &str, server_id: &str) -> Result<(),String> {
    Ok(server::files::write_file(file_path, contents, server_id).unwrap())
}

#[tauri::command]
async fn edit_server(server_id: &str, field: &str, value: &str) -> Result<(), String> {
    server::info::update_server_info_field(field, value, server_id)?;
    files::server::update_server_field(server_id, field, value)?;
    Ok(())
}


// #[derive( Deserialize)]
// struct ServerStatus {
//     latency: u32,
//     response: mcping::Response,
// }

// #[tauri::command]
// fn get_server_status_json(port: u32) -> Result<String, String> {
//     let server_status = match server::info::get_server_status(port) {
//         Ok((latency, response)) => {
//             let server_status = ServerStatus {
//                 latency,
//                 response,
//             };

//             serde_json::to_string(&server_status)
//                 .map_err(|err| format!("Failed to serialize server status as JSON: {}", err))
//         }
//         Err(err) => Err(format!("Failed to get server status: {}", err)),
//     };

//     server_status
// }

fn main() {
    let _run_config = config::get::get_settings().map_err(|e| e.to_string());
    let app = tauri::Builder::default();

    app.invoke_handler(tauri::generate_handler![
        list_servers,
        add_server_command,
        get_file_loc_command,
        get_server_info_command,
        start_server_command,
        get_config_command,
        set_config_command,
        get_server_log,
        get_server_status_command,
        kill_server_command,
        execute_command_on_server,
        get_files_command,
        get_server_file,
        set_server_file,
        server::files::open_file_or_explorer,
        server::files::read_server_properties,
        server::files::write_server_properties,
        edit_server,
        remove_server_commamd,
        download_mod_command,
        get_mods_as_string,
    ])
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application");
}