use pyo3::prelude::*;
use crate::python_bridge::types::*;
use crate::python_bridge::registry::*;
use crate::python_bridge::modification::*;

// Main registration functions
#[pyfunction]
pub fn register_tab(tab_id: String, tab_name: String, icon: Option<String>) -> PyResult<()> {
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
pub fn add_dropdown_menu(tab_id: String, menu_name: String, icon: Option<String>) -> PyResult<()> {
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
pub fn add_menu_item(tab_id: String, menu_name: String, item_name: String, function_name: String, icon: Option<String>) -> PyResult<()> {
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
pub fn register_button(button_id: String, button_name: String, function_name: String, icon: Option<String>) -> PyResult<()> {
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
pub fn register_function(function_name: String, function: Py<PyAny>) -> PyResult<()> {
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
pub fn print_to_console(msg: &str) -> PyResult<()> {
    println!("[Python Mod] {}", msg);
    Ok(())
}

pub fn init_module() {
    pyo3::append_to_inittab!(text_engien);
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
