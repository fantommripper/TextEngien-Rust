use eframe::{App, CreationContext, Frame};
use egui::Context;
use crate::app::AppState;
use crate::ui::render_ui;
use crate::python_bridge::call_python_function;

pub struct TextEngineApp {
    state: AppState,
}

impl TextEngineApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let state = AppState::new();
        
        Self { state }
    }

    fn handle_action(&mut self, function_name: &str) {
        println!("🔧 Calling Python function: {}", function_name);
        
        // Try to call the registered Python function
        match call_python_function(function_name) {
            Ok(_) => println!("✅ Function '{}' executed successfully", function_name),
            Err(e) => println!("❌ Error executing function '{}': {:?}", function_name, e),
        }
    }
}

impl App for TextEngineApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Update app state (Legion systems)
        self.state.update();

        // Render UI and collect actions
        let actions_to_execute = std::cell::RefCell::new(Vec::new());
        render_ui(ctx, &|function_name| {
            actions_to_execute.borrow_mut().push(function_name.to_string());
        });

        // Execute collected actions
        for action in actions_to_execute.into_inner() {
            self.handle_action(&action);
        }
    }
}
