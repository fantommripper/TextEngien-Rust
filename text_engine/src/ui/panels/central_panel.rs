use egui::{Context, Ui};

pub fn render_central_panel(ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("The Text Engien");
    });
}
