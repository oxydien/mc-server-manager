use std::path::PathBuf;




pub fn get_app_folder() -> PathBuf {
  dirs::data_dir()
      .expect("Failed to get the data directory")
      .join("mc-svr-mng")
}

pub fn get_libs_folder() -> PathBuf {
  get_app_folder().join("libs")
}

pub fn get_server_folder(server_id: &str) -> PathBuf {
  get_app_folder().join("servers").join(server_id)
}

pub fn get_servers_json() -> PathBuf {
  get_app_folder().join("servers.json")
}