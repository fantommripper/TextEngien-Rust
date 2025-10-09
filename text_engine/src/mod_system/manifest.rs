use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub main: String,
    pub load_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub mod_info: ModInfo,
    pub dependencies: HashMap<String, String>,
}

impl Manifest {
    pub fn load_from_path(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_path = path.join("manifest.toml");
        let manifest_str = std::fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest.toml: {}", e))?;

        let manifest: Manifest = toml::from_str(&manifest_str)
            .map_err(|e| format!("Invalid manifest.toml: {}", e))?;

        Ok(manifest)
    }
}
