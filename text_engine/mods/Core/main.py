import text_engien

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