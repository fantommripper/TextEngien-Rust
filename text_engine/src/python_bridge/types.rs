use serde::{Deserialize, Serialize};
use pyo3::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabDefinition {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub dropdown_menus: Vec<DropdownMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownMenu {
    pub name: String,
    pub icon: Option<String>,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub name: String,
    pub icon: Option<String>,
    pub function_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonDefinition {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub function_name: String,
}

#[derive(Debug)]
pub struct PythonFunction {
    pub name: String,
    pub module: String,
    pub function: Py<PyAny>,
}

