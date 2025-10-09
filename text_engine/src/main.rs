mod python_bridge;
mod mod_system;
mod ui;
mod app;

use eframe;
use crate::python_bridge::init_module;
use crate::mod_system::load_all_mods;
use crate::app::TextEngineApp;

fn initialize_app() {
    // Initialize Python module
    init_module();
    
    // Load all mods
    match load_all_mods(false) {
        Ok(_) => println!("✅ All mods loaded successfully"),
        Err(e) => println!("❌ Error loading mods: {:?}", e),
    }
}

fn main() -> Result<(), eframe::Error> {
    // Initialize application
    initialize_app();
    
    // Application window settings
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_title("The Text Engien"),
        ..Default::default()
    };

    // Launch native application
    eframe::run_native(
        "The Text Engien",
        options,
        Box::new(|cc| Ok(Box::new(TextEngineApp::new(cc)))),
    )
}