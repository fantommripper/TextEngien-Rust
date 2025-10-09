import text_engien

# Define functions for window example
def on_button_click():
    text_engien.print_to_console("🎉 Button was clicked!")
    text_value = text_engien.get_text_value("example_window", "name_input")
    slider_value = text_engien.get_float_value("example_window", "volume_slider")
    text_engien.print_to_console(f"📝 Text: {text_value}, 🎚️ Slider: {slider_value}")

def create_example_window():
    text_engien.create_window("example_window", "📊 Example Window")
    text_engien.set_window_size("example_window", 400.0, 300.0)
    text_engien.set_window_position("example_window", 100.0, 100.0)
    
    text_engien.add_heading("example_window", "Welcome to Custom Windows!", 1)
    text_engien.add_separator("example_window")
    text_engien.add_label("example_window", "This is a custom window from Python!", True, False)
    text_engien.add_spacing("example_window", 10.0)
    
    text_engien.add_label("example_window", "Enter your name:", False, False)
    text_engien.add_text_edit("example_window", "name_input", False, "Your name...", False)
    
    text_engien.add_slider("example_window", "volume_slider", 0.0, 100.0, "Volume", False)
    text_engien.add_checkbox("example_window", "enabled_checkbox", "Enable feature")
    
    text_engien.add_separator("example_window")
    text_engien.add_button("example_window", "🎯 Click Me!", "on_button_click", False)

# Define functions for various actions
def new_file():
    text_engien.print_to_console("📄 Creating new file...")
    # Here could be logic for creating a new file

def open_file():
    text_engien.print_to_console("📂 Opening file...")
    # Here could be logic for opening a file

def save_file():
    text_engien.print_to_console("💾 Saving file...")
    # Here could be logic for saving a file

def undo_action():
    text_engien.print_to_console("↶ Undoing last action...")
    # Here could be logic for undo

def redo_action():
    text_engien.print_to_console("↷ Redoing last action...")
    # Here could be logic for redo

def copy_text():
    text_engien.print_to_console("📋 Copying text...")
    # Here could be logic for copying

def paste_text():
    text_engien.print_to_console("📋 Pasting text...")
    # Here could be logic for pasting

def open_settings():
    text_engien.print_to_console("⚙️ Opening settings...")
    # Here could be logic for opening settings

def change_theme():
    text_engien.print_to_console("🎨 Changing theme...")
    # Here could be logic for theme change

def export_exe():
    text_engien.print_to_console("📄 Exporting to EXE...")
    # Here could be logic for EXE export

def run():
    text_engien.print_to_console("Hello from Core mod!")
    
    # Register window example function
    text_engien.register_function("on_button_click", on_button_click)
    
    # Create example window
    text_engien.print_to_console("🪟 Creating example window...")
    create_example_window()
    text_engien.print_to_console("✅ Example window created!")
    
    # Register functions
    text_engien.register_function("new_file", new_file)
    text_engien.register_function("open_file", open_file)
    text_engien.register_function("save_file", save_file)
    text_engien.register_function("undo_action", undo_action)
    text_engien.register_function("redo_action", redo_action)
    text_engien.register_function("copy_text", copy_text)
    text_engien.register_function("paste_text", paste_text)
    text_engien.register_function("open_settings", open_settings)
    text_engien.register_function("change_theme", change_theme)
    text_engien.register_function("export_exe", export_exe)

    # Register "File" tab
    text_engien.register_tab("file_tab", "File", "📁")
    
    # First dropdown menu "File"
    text_engien.add_dropdown_menu("file_tab", "File", "📄")
    text_engien.add_menu_item("file_tab", "File", "New File", "new_file", "📄")
    text_engien.add_menu_item("file_tab", "File", "Open File", "open_file", "📂")
    text_engien.add_menu_item("file_tab", "File", "Save File", "save_file", "💾")
    
    # Second dropdown menu "Export"
    text_engien.add_dropdown_menu("file_tab", "Export", "📤")
    text_engien.add_menu_item("file_tab", "Export", "Export to EXE", "export_exe", "📄")

    # Register "Edit" tab
    text_engien.register_tab("edit_tab", "", "")
    text_engien.add_dropdown_menu("edit_tab", "Edit", "🔄")
    text_engien.add_menu_item("edit_tab", "Edit", "Undo", "undo_action", "<-")
    text_engien.add_menu_item("edit_tab", "Edit", "Redo", "redo_action", "->")
    text_engien.add_menu_item("edit_tab", "Edit", "Copy", "copy_text", "📋")
    text_engien.add_menu_item("edit_tab", "Edit", "Paste", "paste_text", "📋")
    
    # Register "Settings" tab
    text_engien.register_tab("settings_tab", "", "")
    text_engien.add_dropdown_menu("settings_tab", "Settings", "🔧")
    text_engien.add_menu_item("settings_tab", "Settings", "General Settings", "open_settings", "🔧")
    text_engien.add_menu_item("settings_tab", "Settings", "Theme", "change_theme", "🎨")

    # # Register buttons
    # text_engien.register_button("save_btn", "Save", "save_file", "💾")
    # text_engien.register_button("open_btn", "Open", "open_file", "📂")
    # text_engien.register_button("new_btn", "New", "new_file", "📄")