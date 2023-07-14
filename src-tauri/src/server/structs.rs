use serde::{Deserialize, Serialize};

pub struct ModInfo {
    file_name: String,
    name: String,
    version_id: String,
    project_id: String,
}

pub struct ServerInfo {
    id: String,
    name: String,
    s_type: String,
    mc_version: String,
    mods: Option<Vec<ModInfo>>,
    image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleServerInfo {
    id: String,
    name: String,
    mc_version: String,
    s_type: String,
    image: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub file_type: String,
    pub created: String,
    pub modified: String,
    pub size_bytes: u64,
}