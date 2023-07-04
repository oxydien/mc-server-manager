use crate::files::get::get_app_folder;
use crate::server::info::get_server_info;
use std::fs::{ File};
use std::io::{Read,Write};
use serde_json::Value;

pub fn get_settings() -> Result<String, String> {
  let appdata = get_app_folder();
  let settings_path = appdata.join("config.json");

  if settings_path.exists() {
      let mut file = File::open(&settings_path).map_err(|e| e.to_string())?;
      let mut content = String::new();
      file.read_to_string(&mut content).map_err(|e| e.to_string())?;
      Ok(content)
  } else {
      let default_content = r#"{"jdk8":"","jdk17":"","arguments":"","memory_m":""}"#;

      let mut file = File::create(&settings_path).map_err(|e| e.to_string())?;
      file.write_all(default_content.as_bytes())
          .map_err(|e| e.to_string())?;

      Ok(default_content.to_string())
  }
}

pub fn get_jdk8() -> Result<String, String> {
  let settings = get_settings()?;
  let json: Value = serde_json::from_str(&settings).map_err(|e| e.to_string())?;
  let jdk8 = json["jdk8"].as_str().unwrap_or("").to_string();
  Ok(jdk8)
}

pub fn get_jdk17() -> Result<String, String> {
  let settings = get_settings()?;
  let json: Value = serde_json::from_str(&settings).map_err(|e| e.to_string())?;
  let jdk17 = json["jdk17"].as_str().unwrap_or("").to_string();
  Ok(jdk17)
}

pub fn get_arguments() -> Result<String, String> {
  let settings = get_settings()?;
  let json: Value = serde_json::from_str(&settings).map_err(|e| e.to_string())?;
  let arguments = json["arguments"].as_str().unwrap_or("").to_string();
  Ok(arguments)
}

pub fn get_memory_m() -> Result<String, String> {
  let settings = get_settings()?;
  let json: Value = serde_json::from_str(&settings).map_err(|e| e.to_string())?;
  let memory_m = json["memory_m"].as_str().unwrap_or("").to_string();
  Ok(memory_m)
}

pub fn find_java_path(server_id: &str) -> Result<String, String> {
  let server_info_str = get_server_info(server_id)?;

  let server_info: Value = serde_json::from_str(&server_info_str).map_err(|e| e.to_string())?;

  let java_path = server_info["java_path"].as_str().unwrap_or("");

  if !java_path.is_empty() {
      return Ok(java_path.to_string());
  }

  let mc_version = server_info["mc_version"].as_str().unwrap_or("");

  if mc_version > "1.16.5" || mc_version >= "20w45a" {
      return get_jdk17();
  } else {
      return get_jdk8();
  }
}

pub fn find_ram_configuration(server_id: &str) -> Result<String, String> {
  let server_info_str = get_server_info(server_id)?;

  let server_info: Value = serde_json::from_str(&server_info_str).map_err(|e| e.to_string())?;

  let ram_config = server_info["memory_m"].as_str();

  if let Some(ram) = ram_config {
      return Ok(ram.to_string());
  }

  Ok(get_memory_m().unwrap())
}