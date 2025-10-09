use egui::Context;
use crate::ui::panels::{render_top_panel, render_central_panel};
use crate::ui::window_renderer::render_windows;

pub fn render_ui(ctx: &Context, on_action: &dyn Fn(&str)) {
    render_top_panel(ctx, on_action);
    render_central_panel(ctx);
    
    render_windows(ctx, on_action);
    
    // Request repaint for the next frame if continuous animation is needed
    ctx.request_repaint();
}
