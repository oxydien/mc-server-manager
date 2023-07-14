use crate::files::get::get_server_folder;
use mc_query;
use serde_json::json;
use std::{fs, io::Read};

pub fn get_server_info(server_id: &str) -> Result<String, String> {
    let server_info_path = get_server_folder(server_id).join("server-info.json");

    let mut server_info_file = fs::File::open(server_info_path).map_err(|e| e.to_string())?;

    let mut server_info_bytes = Vec::new();
    server_info_file
        .read_to_end(&mut server_info_bytes)
        .map_err(|e| e.to_string())?;

    let server_info_string = String::from_utf8(server_info_bytes).map_err(|e| e.to_string())?;

    Ok(server_info_string)
}

pub async fn check_server_status(server_id: &str) -> Result<String, String> {
    let pid_file_path = get_server_folder(server_id).join("server_pid.txt");

    if !pid_file_path.exists() {
        let offline_status = json!({
            "offline": true,
            "starting": false,
            "online": false,
            "data": {}
        });
        return Ok(offline_status.to_string());
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

                if process_exists {
                    if let Some(server_data) = get_server_info_data(
                        "127.0.0.1",
                        crate::server::files::get_server_port(server_id)?,
                    )
                    .await
                    {
                        let online_status = json!({
                            "offline": false,
                            "starting": false,
                            "online": true,
                            "data": server_data,
                        });
                        return Ok(online_status.to_string());
                    } else {
                        let starting_status = json!({
                            "offline": false,
                            "starting": true,
                            "online": false,
                            "data": {},
                        });
                        return Ok(starting_status.to_string());
                    }
                } else {
                    let offline_status = json!({
                        "offline": true,
                        "starting": false,
                        "online": false,
                        "data": {}
                    });
                    return Ok(offline_status.to_string());
                }
            }
            Err(e) => Err(e.to_string()),
        }
    } else {
        let offline_status = json!({
            "offline": true,
            "starting": false,
            "online": false,
            "data": {}
        });
        Ok(offline_status.to_string())
    }
}

async fn get_server_info_data(server_ip: &str, server_port: u16) -> Option<mc_query::status::StatusResponse> {
    let data = mc_query::status(server_ip, server_port).await.ok()?;
    Some(data)
}

pub fn update_server_info_field(name: &str, value: &str, server_id: &str) -> Result<(), String> {
    let server_dir = get_server_folder(server_id);
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
        Err(err) => {
            return Err(format!(
                "Failed to serialize updated server-info.json: {}",
                err
            ))
        }
    };

    match fs::write(&server_info_path, updated_server_info_content) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Failed to write updated server-info.json: {}", err)),
    }
}
