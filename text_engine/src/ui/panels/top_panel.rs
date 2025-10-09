use egui::{Context, Ui};
use crate::python_bridge::registry::{REGISTERED_TABS, REGISTERED_BUTTONS};
use crate::ui::components::{render_tab, render_buttons_group};

pub fn render_top_panel(ctx: &Context, on_action: &dyn Fn(&str)) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Display registered tabs
            if let Ok(tabs) = REGISTERED_TABS.lock() {
                for tab in tabs.iter() {
                    render_tab(ui, tab, on_action);
                }
            }
            
            // Display registered buttons
            if let Ok(buttons) = REGISTERED_BUTTONS.lock() {
                render_buttons_group(ui, &buttons, on_action);
            }
        });
    });
}
