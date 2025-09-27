use std::ffi::CString;
use std::sync::Mutex;
use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    pub static ref REGISTERED_TABS: Mutex<Vec<TabDefinition>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_BUTTONS: Mutex<Vec<ButtonDefinition>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_FUNCTIONS: Mutex<HashMap<String, PythonFunction>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabDefinition {
    pub id: String,
    pub name: String,
    pub icon: Option<String>, // Add icon support
    pub dropdown_menus: Vec<DropdownMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropdownMenu {
    pub name: String,
    pub icon: Option<String>, // Add icon support
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub name: String,
    pub icon: Option<String>, // Add icon support
    pub function_name: String, // Changed from action to function_name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonDefinition {
    pub id: String,
    pub name: String,
    pub icon: Option<String>, // Add icon support
    pub function_name: String, // Changed from action to function_name
}

#[derive(Debug)]
pub struct PythonFunction {
    pub name: String,
    pub module: String,
    pub function: Py<PyAny>,
}


#[pyfunction]
fn register_tab(tab_id: String, tab_name: String, icon: Option<String>) -> PyResult<()> {
    let tab = TabDefinition {
        id: tab_id,
        name: tab_name,
        icon,
        dropdown_menus: Vec::new(),
    };

    REGISTERED_TABS.lock().unwrap().push(tab);
    Ok(())
}

#[pyfunction]
fn add_dropdown_menu(tab_id: String, menu_name: String, icon: Option<String>) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();

    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        let dropdown = DropdownMenu {
            name: menu_name,
            icon,
            items: Vec::new(),
        };
        tab.dropdown_menus.push(dropdown);
    }
    
    Ok(())
}

#[pyfunction]
fn add_menu_item(tab_id: String, menu_name: String, item_name: String, function_name: String, icon: Option<String>) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();

    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        if let Some(menu) = tab.dropdown_menus.iter_mut().find(|m| m.name == menu_name) {
            let item = MenuItem {
                name: item_name,
                icon,
                function_name,
            };
            menu.items.push(item);
        }
    }
    
    Ok(())
}

#[pyfunction]
fn register_button(button_id: String, button_name: String, function_name: String, icon: Option<String>) -> PyResult<()> {
    let button = ButtonDefinition {
        id: button_id,
        name: button_name,
        icon,
        function_name,
    };
    
    REGISTERED_BUTTONS.lock().unwrap().push(button);
    Ok(())
}

#[pyfunction]
fn register_function(function_name: String, function: Py<PyAny>) -> PyResult<()> {
    Python::attach(|py| -> PyResult<()> {
        // Get module name from function
        let module_name = match function.getattr(py, "__module__") {
            Ok(module) => module.extract::<String>(py).unwrap_or("unknown".to_string()),
            Err(_) => "unknown".to_string(),
        };
        
        let python_function = PythonFunction {
            name: function_name.clone(),
            module: module_name,
            function: function.clone_ref(py),
        };
        
        REGISTERED_FUNCTIONS.lock().unwrap().insert(function_name, python_function);
        Ok(())
    })?;
    
    Ok(())
}

#[pyfunction]
fn print_to_console(msg: &str) -> PyResult<()> {
    println!("[Python Mod] {}", msg);
    Ok(())
}

// Functions for modifying elements from other mods
#[pyfunction]
fn modify_tab_icon(tab_id: String, new_icon: String) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();
    
    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        tab.icon = Some(new_icon);
        println!("✅ Tab icon '{}' changed", tab_id);
    } else {
        println!("⚠️ Tab '{}' not found", tab_id);
    }
    
    Ok(())
}

#[pyfunction]
fn modify_tab_name(tab_id: String, new_name: String) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();
    
    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        let old_name = tab.name.clone();
        tab.name = new_name.clone();
        println!("✅ Tab name changed from '{}' to '{}'", old_name, new_name);
    } else {
        println!("⚠️ Tab '{}' not found", tab_id);
    }
    
    Ok(())
}

#[pyfunction]
fn add_menu_item_to_existing(tab_id: String, menu_name: String, item_name: String, function_name: String, icon: Option<String>) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();
    
    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        if let Some(menu) = tab.dropdown_menus.iter_mut().find(|m| m.name == menu_name) {
            let item = MenuItem {
                name: item_name,
                icon,
                function_name,
            };
            let item_name = item.name.clone();
            menu.items.push(item);
            println!("✅ Item '{}' added to menu '{}' in tab '{}'", item_name, menu_name, tab_id);
        } else {
            println!("⚠️ Menu '{}' not found in tab '{}'", menu_name, tab_id);
        }
    } else {
        println!("⚠️ Tab '{}' not found", tab_id);
    }
    
    Ok(())
}

#[pyfunction]
fn remove_menu_item(tab_id: String, menu_name: String, item_name: String) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();
    
    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        if let Some(menu) = tab.dropdown_menus.iter_mut().find(|m| m.name == menu_name) {
            if let Some(pos) = menu.items.iter().position(|item| item.name == item_name) {
                menu.items.remove(pos);
                println!("✅ Item '{}' removed from menu '{}'", item_name, menu_name);
            } else {
                println!("⚠️ Item '{}' not found in menu '{}'", item_name, menu_name);
            }
        } else {
            println!("⚠️ Menu '{}' not found in tab '{}'", menu_name, tab_id);
        }
    } else {
        println!("⚠️ Tab '{}' not found", tab_id);
    }
    
    Ok(())
}

#[pyfunction]
fn modify_menu_item(tab_id: String, menu_name: String, old_item_name: String, new_item_name: String, new_function_name: String, new_icon: Option<String>) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();
    
    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        if let Some(menu) = tab.dropdown_menus.iter_mut().find(|m| m.name == menu_name) {
            if let Some(item) = menu.items.iter_mut().find(|item| item.name == old_item_name) {
                item.name = new_item_name.clone();
                item.function_name = new_function_name;
                item.icon = new_icon;
                println!("✅ Menu item '{}' changed to '{}'", old_item_name, new_item_name);
            } else {
                println!("⚠️ Item '{}' not found in menu '{}'", old_item_name, menu_name);
            }
        } else {
            println!("⚠️ Menu '{}' not found in tab '{}'", menu_name, tab_id);
        }
    } else {
        println!("⚠️ Tab '{}' not found", tab_id);
    }
    
    Ok(())
}

#[pyfunction]
fn remove_button(button_id: String) -> PyResult<()> {
    let mut buttons = REGISTERED_BUTTONS.lock().unwrap();
    
    if let Some(pos) = buttons.iter().position(|b| b.id == button_id) {
        let button = buttons.remove(pos);
        println!("✅ Button '{}' removed", button.name);
    } else {
        println!("⚠️ Button '{}' not found", button_id);
    }
    
    Ok(())
}

#[pyfunction]
fn modify_button(button_id: String, new_name: String, new_function_name: String, new_icon: Option<String>) -> PyResult<()> {
    let mut buttons = REGISTERED_BUTTONS.lock().unwrap();
    
    if let Some(button) = buttons.iter_mut().find(|b| b.id == button_id) {
        let old_name = button.name.clone();
        button.name = new_name.clone();
        button.function_name = new_function_name;
        button.icon = new_icon;
        println!("✅ Button changed from '{}' to '{}'", old_name, new_name);
    } else {
        println!("⚠️ Button '{}' not found", button_id);
    }
    
    Ok(())
}

#[pymodule(name = "text_engien")]
fn text_engien(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Main registration functions
    m.add_function(wrap_pyfunction!(register_tab, m)?)?;
    m.add_function(wrap_pyfunction!(add_dropdown_menu, m)?)?;
    m.add_function(wrap_pyfunction!(add_menu_item, m)?)?;
    m.add_function(wrap_pyfunction!(register_button, m)?)?;
    m.add_function(wrap_pyfunction!(register_function, m)?)?;
    
    // Modification functions
    m.add_function(wrap_pyfunction!(modify_tab_icon, m)?)?;
    m.add_function(wrap_pyfunction!(modify_tab_name, m)?)?;
    m.add_function(wrap_pyfunction!(add_menu_item_to_existing, m)?)?;
    m.add_function(wrap_pyfunction!(remove_menu_item, m)?)?;
    m.add_function(wrap_pyfunction!(modify_menu_item, m)?)?;
    m.add_function(wrap_pyfunction!(remove_button, m)?)?;
    m.add_function(wrap_pyfunction!(modify_button, m)?)?;
    
    // Utilities
    m.add_function(wrap_pyfunction!(print_to_console, m)?)?;
    Ok(())
}

pub fn inite_module(){
    pyo3::append_to_inittab!(text_engien);
}

pub fn call_python_function(function_name: &str) -> PyResult<()> {
    if let Ok(functions) = REGISTERED_FUNCTIONS.lock() {
        if let Some(python_function) = functions.get(function_name) {
            Python::attach(|py| {
                python_function.function.call0(py)?;
                Ok(())
            })
        } else {
            println!("⚠️ Function '{}' not found in registered functions", function_name);
            Ok(())
        }
    } else {
        println!("❌ Error accessing registered functions");
        Ok(())
    }
}

pub fn run_python(file_path: &str) -> PyResult<()> {
    let main_content = std::fs::read_to_string(file_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e)))?;

    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("main.py");

    Python::attach(|py| -> PyResult<()>{

        let main_content_cstr = CString::new(main_content).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to convert code to CString: {}", e)))?;
        let file_name_cstr = CString::new(file_name).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to convert filename to CString: {}", e)))?;
        let empty_cstr = CString::new("").unwrap();

        let run_fn: Py<PyAny> = PyModule::from_code(
            py,
            &main_content_cstr,
            &file_name_cstr,
            &empty_cstr
        )?
        .getattr("run")?
        .into();

        run_fn.call0(py)?;
        Ok(())
    })?;
    Ok(())
}