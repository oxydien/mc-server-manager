use crate::config::get::find_java_path;
use crate::config::get::find_ram_configuration;
use crate::files::get::get_server_folder;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::time::Duration;

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub async fn run_server_and_save_output(
    server_id: &str,
    server_type: &str,
) -> Result<bool, String> {
    println!("[run-server]: Starting server {}", server_id);
    let server_folder = get_server_folder(server_id);
    let server_jar_path: PathBuf;

    match server_type {
        "vanilla" => {
            server_jar_path = server_folder.join("server.jar");
        }
        "quilt" => {
            server_jar_path = server_folder.join("quilt-server-launch.jar");
        }
        "fabric" => {
            server_jar_path = server_folder.join("fabric-server-launch.jar");
        }
        _ => {
            return Err("[run-server]: Invalid server type".to_string());
        }
    }

    if !server_jar_path.exists() {
        return Err("[run-server]: server.jar file not found".to_string());
    }

    let java_bin = find_java_path(server_id).unwrap();
    let memory_m = find_ram_configuration(server_id).unwrap();
    let mut command = Command::new(java_bin);
    command
        .arg(format!("-Xmx{}M", memory_m))
        .arg("-jar")
        .arg(&server_jar_path)
        // .arg("nogui")
        .current_dir(server_folder.clone());

    let mut process = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let process_id: u32;
    
    loop {
        if let Some(pid) = process.id() {
            process_id = pid;
            break;
        }
    
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    println!("[run-server]: PID: {}", process_id);

    let pid_file_path = server_folder.join("server_pid.txt");
    let mut pid_file = File::create(pid_file_path)
        .await
        .map_err(|e| e.to_string())?;

    pid_file
        .write_all(process_id.to_string().as_bytes())
        .await
        .map_err(|e| e.to_string())?;

    println!("[run-server]: Trying stdout");
    let stdout = process
        .stdout
        .take()
        .ok_or("[run-server]: Failed to capture standard output")?;
    let stderr = process
        .stderr
        .take()
        .ok_or("[run-server]: Failed to capture standard error")?;

    let log_file_path = server_folder.join("server_latest_log.log");
    let mut log_file = File::create(log_file_path)
        .await
        .map_err(|e| e.to_string())?;

    let mut reader_stdout = tokio::io::BufReader::new(stdout);
    let mut reader_stderr = tokio::io::BufReader::new(stderr);

    let mut line_stdout = String::new();
    let mut line_stderr = String::new();

   println!("[run-server]: Starting server");
    loop {
        tokio::select! {
            result = reader_stdout.read_line(&mut line_stdout) => {
                let bytes_read = result.map_err(|e| e.to_string())?;
                if bytes_read == 0 {
                    break;
                }
                log_file.write_all(line_stdout.as_bytes()).await.map_err(|e| e.to_string())?;
                log_file.write_all(b"\n").await.map_err(|e| e.to_string())?;
                line_stdout.clear();
            }
            result = reader_stderr.read_line(&mut line_stderr) => {
                let bytes_read = result.map_err(|e| e.to_string())?;
                if bytes_read == 0 {
                    break;
                }
                log_file.write_all(line_stderr.as_bytes()).await.map_err(|e| e.to_string())?;
                log_file.write_all(b"\n").await.map_err(|e| e.to_string())?;
                line_stderr.clear();
            }
        }
    }

    let _ = process.wait().await.map_err(|e| e.to_string())?;

    let is_running = process.try_wait().map_err(|e| e.to_string())?.is_none();

    if !is_running {
        println!("[run-server]: Server stopped");
        log_file
            .write_all("[run-server]: Server stopped".as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        log_file.write_all(b"\n").await.map_err(|e| e.to_string())?;
        let pid_file_path = server_folder.join("server_pid.txt");
        if pid_file_path.exists() {
            tokio::fs::remove_file(pid_file_path)
                .await
                .map_err(|e| e.to_string())?;
            println!("PID file removed");
        }
    }

    Ok(is_running)
}

// pub async fn send_command_to_server(command: &str) -> Result<(), String> {
//     let sender = COMMAND_SENDER.get().ok_or("Server process not running")?;
//     let sender = sender.lock().map_err(|e| e.to_string())?;
//     let sender = match sender {
//         Ok(guard) => guard,
//         Err(poisoned) => poisoned.into_inner(),
//     };
//     let sender = sender.ok_or("Server process not running")?;
//     sender.send(command.to_string())
//         .await
//         .map_err(|e| e.to_string())?;
//     Ok(())
// }

pub async fn kill_server_process(server_id: &str) -> Result<(), String> {
    let pid_file_path = get_server_folder(server_id)
        .join("server_pid.txt");

    if !pid_file_path.exists() {
        return Ok(());
    }

    let pid_str = tokio::fs::read_to_string(pid_file_path)
        .await
        .map_err(|e| e.to_string())?;
    let output = Command::new("taskkill")
        .args(&["/PID", &pid_str, "/T", "/F"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Failed to kill server process".to_string())
    }
}
