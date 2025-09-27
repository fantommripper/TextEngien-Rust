mod mod_loader;
mod python_api;
mod dependency_resolver;

use eframe::egui;
use legion::{World, Resources, Schedule};
use crate::python_api::{REGISTERED_TABS, REGISTERED_BUTTONS, call_python_function};

// Your main application, which stores state
struct MyApp {
    world: World,
    schedule: Schedule,
    windows: Vec<(String, bool)>,
    // Add other states you need here
}

impl MyApp {
    fn handle_action(&mut self, function_name: &str) {
        println!("🔧 Calling Python function: {}", function_name);
        
        // Try to call the registered Python function
        match call_python_function(function_name) {
            Ok(_) => println!("✅ Function '{}' executed successfully", function_name),
            Err(e) => println!("❌ Error executing function '{}': {:?}", function_name, e),
        }
    }

    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Here you can set up fonts, styles, etc.
        // setup_custom_fonts(&cc.egui_ctx); // Example from search:cite[3]

        python_api::inite_module();
        mod_loader::load_all_mods(false);

        let world = World::default();
        let schedule = Schedule::builder().build();

        Self {
            world,
            schedule,
            windows: vec![
                ("Window 1".to_string(), false)
            ],
        }
    }
}

// Implementation of eframe::App trait - the heart of the application
impl eframe::App for MyApp {
    // This function is called every frame to update the UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Your logic (executing Legion systems)
        let mut resources = Resources::default();
        self.schedule.execute(&mut self.world, &mut resources);

        // 2. Drawing the UI
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Display registered tabs
                if let Ok(tabs) = REGISTERED_TABS.lock() {
                    for tab in tabs.iter() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                // Tab name with icon
                                let tab_icon = tab.icon.as_deref().unwrap_or("📁");
                                if !tab_icon.is_empty() || !tab.name.is_empty(){
                                    ui.label(format!("{} {}", tab_icon, tab.name));
                                }
                                // Dropdown menus for the tab
                                for dropdown in &tab.dropdown_menus {
                                    let dropdown_icon = dropdown.icon.as_deref().unwrap_or("▼");
                                    let dropdown_text = format!("{} {}", dropdown_icon, dropdown.name);
                                    
                                    ui.menu_button(&dropdown_text, |ui| {
                                        for item in &dropdown.items {
                                            let item_icon = item.icon.as_deref().unwrap_or("");
                                            let item_text = if item_icon.is_empty() {
                                                item.name.clone()
                                            } else {
                                                format!("{} {}", item_icon, item.name)
                                            };
                                            
                                            if ui.button(&item_text).clicked() {
                                                self.handle_action(&item.function_name);
                                                ui.close();
                                            }
                                        }
                                    });
                                }
                            });
                        });
                    }
                }
                // Display registered buttons
                if let Ok(buttons) = REGISTERED_BUTTONS.lock() {
                    if !buttons.is_empty(){
                        ui.group(|ui| {
                            for button in buttons.iter() {
                                let button_icon = button.icon.as_deref().unwrap_or("🔘");
                                let button_text = format!("{} {}", button_icon, button.name);

                                if ui.button(&button_text).clicked() {
                                    self.handle_action(&button.function_name);
                                }
                            }
                        });
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("The Text Egien");
        });

        // Optionally: request repaint for the next frame if continuous animation is needed
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    // Application window settings:cite[3]:cite[7]
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_title("The Text Engien"),
        ..Default::default()
    };

    // Launch native application:cite[5]:cite[7]
    eframe::run_native(
        "The Text Engien",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}