use pyo3::prelude::*;
use crate::python_bridge::registry::*;

// Functions for modifying elements from other mods
#[pyfunction]
pub fn modify_tab_icon(tab_id: String, new_icon: String) -> PyResult<()> {
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
pub fn modify_tab_name(tab_id: String, new_name: String) -> PyResult<()> {
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
pub fn add_menu_item_to_existing(tab_id: String, menu_name: String, item_name: String, function_name: String, icon: Option<String>) -> PyResult<()> {
    let mut tabs = REGISTERED_TABS.lock().unwrap();
    
    if let Some(tab) = tabs.iter_mut().find(|t| t.id == tab_id) {
        if let Some(menu) = tab.dropdown_menus.iter_mut().find(|m| m.name == menu_name) {
            let item = crate::python_bridge::types::MenuItem {
                name: item_name.clone(),
                icon,
                function_name,
            };
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
pub fn remove_menu_item(tab_id: String, menu_name: String, item_name: String) -> PyResult<()> {
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
pub fn modify_menu_item(tab_id: String, menu_name: String, old_item_name: String, new_item_name: String, new_function_name: String, new_icon: Option<String>) -> PyResult<()> {
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
pub fn remove_button(button_id: String) -> PyResult<()> {
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
pub fn modify_button(button_id: String, new_name: String, new_function_name: String, new_icon: Option<String>) -> PyResult<()> {
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

