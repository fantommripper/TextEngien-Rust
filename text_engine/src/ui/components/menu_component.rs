use egui::Ui;
use crate::python_bridge::types::MenuItem;

pub fn render_menu_item(ui: &mut Ui, item: &MenuItem, on_item_click: &dyn Fn(&str)) {
    let item_icon = item.icon.as_deref().unwrap_or("");
    let item_text = if item_icon.is_empty() {
        item.name.clone()
    } else {
        format!("{} {}", item_icon, item.name)
    };
    
    if ui.button(&item_text).clicked() {
        on_item_click(&item.function_name);
        ui.close_menu();
    }
}
