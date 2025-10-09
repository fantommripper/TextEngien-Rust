use egui::Ui;
use crate::python_bridge::types::ButtonDefinition;

pub fn render_button(ui: &mut Ui, button: &ButtonDefinition, on_button_click: &dyn Fn(&str)) {
    let button_icon = button.icon.as_deref().unwrap_or("🔘");
    let button_text = format!("{} {}", button_icon, button.name);

    if ui.button(&button_text).clicked() {
        on_button_click(&button.function_name);
    }
}

pub fn render_buttons_group(ui: &mut Ui, buttons: &[ButtonDefinition], on_button_click: &dyn Fn(&str)) {
    if !buttons.is_empty() {
        ui.group(|ui| {
            for button in buttons {
                render_button(ui, button, on_button_click);
            }
        });
    }
}
