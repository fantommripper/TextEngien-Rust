use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::Deserialize;
use crate::python_api;
use crate::dependency_resolver::resolve_dependencies;
use crate::dependency_resolver::LoadedMod;

#[derive(Debug, Deserialize)]
struct ModInfo {
    id: String,
    name: String,
    version: String,
    authors: Vec<String>,
    main: String,
    load_order: i32,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    mod_info: ModInfo,
    dependencies: HashMap<String, String>,
}

fn load_manifest(path: &Path) -> Manifest {
    let manifest_path = path.join("manifest.toml");
    let manifest_str = fs::read_to_string(&manifest_path)
        .expect("❌ Failed to read manifest.toml");

    toml::from_str(&manifest_str)
        .expect("❌ Invalid manifest.toml")
}

pub fn load_mod(path: &Path, manifest: &Manifest) {
    println!("📦 Loading mod from {:?}", path);
    println!("✅ Loaded manifest: {:?}", manifest);

    let main_py = path.join(&manifest.mod_info.main);
    python_api::run_python(&main_py.to_string_lossy())
        .expect("❌ Failed to run Python script");
}

pub fn load_all_mods(strict: bool) {
    let mods_dir = Path::new("./mods");

    // Собираем все манифесты
    let mut manifests: HashMap<String, (Manifest, PathBuf)> = HashMap::new();

    for entry in fs::read_dir(mods_dir).expect("❌ Failed to read ./mods") {
        let entry = entry.expect("❌ Failed to read dir entry");
        let path = entry.path();

        if path.is_dir() {
            let manifest_path = path.join("manifest.toml");
            if manifest_path.exists() {
                let manifest = load_manifest(&path);
                manifests.insert(manifest.mod_info.id.clone(), (manifest, path));
            }
        }
    }

    // Преобразуем в LoadedMod для резолвера
    let loaded_mods: Vec<LoadedMod> = manifests
        .iter()
        .map(|(_, (manifest, _))| LoadedMod {
            id: manifest.mod_info.id.clone(),
            load_order: manifest.mod_info.load_order as u32,
            dependencies: manifest.dependencies.keys().cloned().collect(),
        })
        .collect();

    // Резолвим порядок
    match resolve_dependencies(loaded_mods, strict) {
        Ok(order) => {
            println!("✅ Final load order: {:?}", order);
            for id in order {
                if let Some((manifest, path)) = manifests.get(&id) {
                    load_mod(path, manifest);
                }
            }
        }
        Err(failed) => {
            println!("❌ Failed to resolve dependencies for mods: {:?}", failed);
        }
    }
}
