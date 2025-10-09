use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct LoadedMod {
    pub id: String,
    pub load_order: u32,
    pub dependencies: Vec<String>,
}

impl LoadedMod {
    pub fn new(id: String, load_order: u32, dependencies: Vec<String>) -> Self {
        Self {
            id,
            load_order,
            dependencies,
        }
    }
}

#[derive(Debug)]
pub enum DependencyError {
    MissingDependency(String),
    CircularDependency(String),
    ResolutionFailed(Vec<String>),
}

pub fn resolve_dependencies(mods: Vec<LoadedMod>, strict: bool) -> Result<Vec<String>, DependencyError> {
    let mut mods = mods;
    mods.sort_by_key(|m| m.load_order);

    let mod_map: HashMap<String, &LoadedMod> =
        mods.iter().map(|m| (m.id.clone(), m)).collect();

    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    let mut result = Vec::new();
    let mut failed = Vec::new();

    fn visit(
        id: &str,
        mod_map: &HashMap<String, &LoadedMod>,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
        result: &mut Vec<String>,
        failed: &mut Vec<String>,
        strict: bool,
    ) -> bool {
        if visited.contains(id) {
            return true;
        }
        
        if !mod_map.contains_key(id) {
            eprintln!("⚠️ Dependency \"{}\" not found", id);
            failed.push(id.to_string());
            return false;
        }
        
        if stack.contains(id) {
            eprintln!("⚠️ Circular dependency detected at {}", id);
            failed.push(id.to_string());
            return false;
        }

        stack.insert(id.to_string());
        let m = mod_map.get(id).unwrap();

        let mut ok = true;
        for dep in &m.dependencies {
            if !visit(dep, mod_map, visited, stack, result, failed, strict) {
                ok = false;
            }
        }

        stack.remove(id);
        if ok {
            visited.insert(id.to_string());
            result.push(id.to_string());
            true
        } else {
            failed.push(id.to_string());
            false
        }
    }

    for m in &mods {
        visit(
            &m.id,
            &mod_map,
            &mut visited,
            &mut stack,
            &mut result,
            &mut failed,
            strict,
        );
    }

    if strict {
        if failed.is_empty() {
            Ok(result)
        } else {
            Err(DependencyError::ResolutionFailed(failed))
        }
    } else {
        Ok(result)
    }
}
