use crate::config::get::get_settings;
use crate::files::get::get_app_folder;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;

pub fn set_jdk8(jdk8: &str) -> Result<(), String> {
    update_setting("jdk8", jdk8)
}

pub fn set_jdk17(jdk17: &str) -> Result<(), String> {
    update_setting("jdk17", jdk17)
}

pub fn set_arguments(arguments: &str) -> Result<(), String> {
    update_setting("arguments", arguments)
}

pub fn set_memory_m(memory_m: &str) -> Result<(), String> {
    update_setting("memory_m", memory_m)
}

pub fn update_setting(setting_key: &str, setting_value: &str) -> Result<(), String> {
    let appdata = get_app_folder();
    let settings_path = appdata.join("config.json");

    let mut content = get_settings()?;
    let mut json: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    json[setting_key] = json!(setting_value);

    content = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?;

    let mut file = File::create(&settings_path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}
