use crate::files::server::get_servers_dir;
use std::fs;
use std::io::Read;

pub fn get_server_log(server_id: &str) -> Result<String, String> {
    let server_info_path = get_servers_dir()
        .join("servers")
        .join(server_id)
        .join("server_latest_log.log");

    if server_info_path.exists() {
        let mut file = fs::File::open(&server_info_path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        Ok(content)
    } else {
        Ok("".to_string())
    }
}