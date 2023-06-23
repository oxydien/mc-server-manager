use crate::files::server::get_servers_dir;
use std::{fs, io::Read};
use mcping::get_status;

pub fn get_server_info(server_id: &str) -> Result<String, String> {
    let server_info_path = get_servers_dir()
        .join("servers")
        .join(server_id)
        .join("server-info.json");

    let mut server_info_file = fs::File::open(server_info_path).map_err(|e| e.to_string())?;

    let mut server_info_bytes = Vec::new();
    server_info_file
        .read_to_end(&mut server_info_bytes)
        .map_err(|e| e.to_string())?;

    let server_info_string = String::from_utf8(server_info_bytes).map_err(|e| e.to_string())?;

    Ok(server_info_string)
}

pub async fn check_server_status(server_id: &str) -> Result<bool, String> {
    let pid_file_path = get_servers_dir()
        .join("servers")
        .join(server_id)
        .join("server_pid.txt");

    if !pid_file_path.exists() {
        return Ok(false);
    }

    let pid_str = tokio::fs::read_to_string(pid_file_path)
        .await
        .map_err(|e| e.to_string())?;

    if let Ok(pid) = pid_str.trim().parse::<u32>() {
        match tokio::process::Command::new("tasklist")
            .arg("/FI")
            .arg(format!("PID eq {}", pid))
            .output()
            .await
        {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let process_exists = output_str.contains(&pid.to_string());
                Ok(process_exists)
            }
            Err(e) => Err(e.to_string()),
        }
    } else {
        Ok(false)
    }
}

pub fn update_server_info_field(name: &str, value: &str, server_id: &str) -> Result<(), String> {
    let server_dir = get_servers_dir().join("servers").join(server_id);
    let server_info_path = server_dir.join("server-info.json");

    let server_info_content = match fs::read_to_string(&server_info_path) {
        Ok(content) => content,
        Err(err) => return Err(format!("Failed to read server-info.json: {}", err)),
    };

    let mut server_info: serde_json::Value = match serde_json::from_str(&server_info_content) {
        Ok(json) => json,
        Err(err) => return Err(format!("Failed to parse server-info.json: {}", err)),
    };

    server_info[name] = serde_json::json!(value);

    let updated_server_info_content = match serde_json::to_string_pretty(&server_info) {
        Ok(content) => content,
        Err(err) => return Err(format!("Failed to serialize updated server-info.json: {}", err)),
    };

    match fs::write(&server_info_path, updated_server_info_content) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Failed to write updated server-info.json: {}", err)),
    }
}

pub fn get_server_status(port: u32) -> Result<(u32, mcping::Response), String> {
    let address = format!("localhost:{}", port);

    let (latency, response) = mcping::get_status(&address, None)
        .map_err(|err| format!("Failed to get server status: {}", err))?;

    Ok((latency.try_into().unwrap(), response))
}