use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::python_bridge;
use crate::mod_system::{Manifest, LoadedMod, resolve_dependencies};

#[derive(Debug)]
pub enum ModLoadError {
    DirectoryNotFound,
    ManifestLoadError(String),
    PythonExecutionError(String),
    DependencyResolutionError(String),
}

pub fn load_all_mods(strict: bool) -> Result<(), ModLoadError> {
    let mods_dir = Path::new("./mods");
    
    if !mods_dir.exists() {
        return Err(ModLoadError::DirectoryNotFound);
    }

    let mut manifests: HashMap<String, (Manifest, PathBuf)> = HashMap::new();

    // Scan for mods
    for entry in fs::read_dir(mods_dir)
        .map_err(|_| ModLoadError::DirectoryNotFound)? 
    {
        let entry = entry.map_err(|_| ModLoadError::DirectoryNotFound)?;
        let path = entry.path();

        if path.is_dir() {
            let manifest_path = path.join("manifest.toml");
            if manifest_path.exists() {
                let manifest = Manifest::load_from_path(&path)
                    .map_err(|e| ModLoadError::ManifestLoadError(e.to_string()))?;
                manifests.insert(manifest.mod_info.id.clone(), (manifest, path));
            }
        }
    }

    // Convert to LoadedMod format for dependency resolution
    let loaded_mods: Vec<LoadedMod> = manifests
        .iter()
        .map(|(_, (manifest, _))| LoadedMod::new(
            manifest.mod_info.id.clone(),
            manifest.mod_info.load_order as u32,
            manifest.dependencies.keys().cloned().collect(),
        ))
        .collect();

    // Resolve dependencies
    let load_order = resolve_dependencies(loaded_mods, strict)
        .map_err(|e| ModLoadError::DependencyResolutionError(format!("{:?}", e)))?;

    println!("✅ Final load order: {:?}", load_order);

    // Load mods in resolved order
    for id in load_order {
        if let Some((manifest, path)) = manifests.get(&id) {
            load_mod(path, manifest)?;
        }
    }

    Ok(())
}

fn load_mod(path: &Path, manifest: &Manifest) -> Result<(), ModLoadError> {
    println!("📦 Loading mod '{}' from {:?}", manifest.mod_info.name, path);
    println!("✅ Loaded manifest: {:?}", manifest);

    let main_py = path.join(&manifest.mod_info.main);
    python_bridge::run_python(&main_py.to_string_lossy())
        .map_err(|e| ModLoadError::PythonExecutionError(format!("{:?}", e)))?;

    println!("✅ Mod '{}' loaded successfully", manifest.mod_info.name);
    Ok(())
}
