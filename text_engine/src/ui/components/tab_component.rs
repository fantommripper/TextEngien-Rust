use egui::Ui;
use crate::python_bridge::types::TabDefinition;

pub fn render_tab(ui: &mut Ui, tab: &TabDefinition, on_menu_item_click: &dyn Fn(&str)) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            // Tab name with icon
            let tab_icon = tab.icon.as_deref().unwrap_or("📁");
            if !tab_icon.is_empty() || !tab.name.is_empty() {
                ui.label(format!("{} {}", tab_icon, tab.name));
            }
            
            // Dropdown menus for the tab
            for dropdown in &tab.dropdown_menus {
                render_dropdown_menu(ui, dropdown, on_menu_item_click);
            }
        });
    });
}

fn render_dropdown_menu(ui: &mut Ui, dropdown: &crate::python_bridge::types::DropdownMenu, on_menu_item_click: &dyn Fn(&str)) {
    let dropdown_icon = dropdown.icon.as_deref().unwrap_or("▼");
    let dropdown_text = format!("{} {}", dropdown_icon, dropdown.name);
    
    ui.menu_button(&dropdown_text, |ui| {
        for item in &dropdown.items {
            render_menu_item(ui, item, on_menu_item_click);
        }
    });
}

fn render_menu_item(ui: &mut Ui, item: &crate::python_bridge::types::MenuItem, on_menu_item_click: &dyn Fn(&str)) {
    let item_icon = item.icon.as_deref().unwrap_or("");
    let item_text = if item_icon.is_empty() {
        item.name.clone()
    } else {
        format!("{} {}", item_icon, item.name)
    };
    
    if ui.button(&item_text).clicked() {
        on_menu_item_click(&item.function_name);
        ui.close_menu();
    }
}
