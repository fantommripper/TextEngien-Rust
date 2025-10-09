use egui::Context;
use crate::ui::panels::{render_top_panel, render_central_panel};

pub fn render_ui(ctx: &Context, on_action: &dyn Fn(&str)) {
    render_top_panel(ctx, on_action);
    render_central_panel(ctx);
    
    // Request repaint for the next frame if continuous animation is needed
    ctx.request_repaint();
}
