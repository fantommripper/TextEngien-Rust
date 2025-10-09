# Text-Engine: TUI Game Engine powered by Rust + Python

Welcome to **Text-Engine** — an ambitious project to create a powerful and flexible game engine for building Text User Interface (TUI) games. The engine is designed with a modular architecture, combining the performance of Rust with the simplicity of Python for game scripting.

## 🌟 Key Features

* **Rust Core**: The engine's core is built in Rust, ensuring high performance, memory safety, and reliability. This solid foundation handles all the heavy lifting — from rendering to event management.
* **Python Scripting**: Game logic is written in Python, offering a simple and intuitive development experience. We've created a custom Rust-based library that bridges the gap between the Rust core and your Python code.
* **TUI-Focused**: Our primary goal is to provide a first-class experience for creating TUI games. We're building a rich set of tools and components specifically designed for text-based interfaces.
* **`egui` Interface**: The engine uses the powerful and easy-to-use `egui` library for its own interface, providing a modern environment for managing projects and debugging.
* **Flexible Mod System**: At the heart of our design is an incredibly flexible mod system, allowing developers to easily extend the engine's functionality.

## 🏗️ Current Architecture

### Modular System
- ✅ **Mod Loader** ([`mod_system/loader.rs`](text_engine/src/mod_system/loader.rs)) — automatic loading of mods from the `mods/` directory
- ✅ **Dependency System** ([`mod_system/dependencies.rs`](text_engine/src/mod_system/dependencies.rs)) — resolution and validation of mod dependencies
- ✅ **Mod Manifests** ([`mod_system/manifest.rs`](text_engine/src/mod_system/manifest.rs)) — mod configuration via TOML files

### Python Bridge
- ✅ **Mod API** ([`python_bridge/api.rs`](text_engine/src/python_bridge/api.rs)) — Python API for engine interaction
- ✅ **Function Registration** — system for registering Python functions callable from Rust
- ✅ **Tab and Menu System** — create UI elements from Python code
- ✅ **Console Output** — [`print_to_console()`](text_engine/mods/Core/main.py:5) function for debugging

### UI Components
- ✅ **Main Application Window** ([`app/app.rs`](text_engine/src/app/app.rs))
- ✅ **Tab System** — support for multiple tabs with dropdown menus
- ✅ **Output Console** — built-in console for logs
- ⏳ **Text Editor** (in development)

## 🚀 Quick Start

### Requirements
- Rust 2024 edition (or newer)
- Python 3.x
- Cargo

### Installation and Running

```bash
# Clone the repository
git clone https://github.com/yourusername/TextEngien-Rust.git
cd TextEngien-Rust/text_engine

# Build and run the project
cargo run
```

## 📁 Project Structure

```
TextEngien-Rust/
├── text_engine/              # Main Rust project
│   ├── src/
│   │   ├── main.rs          # Application entry point
│   │   ├── mod_system/      # Mod loading system
│   │   │   ├── loader.rs    # Mod loader
│   │   │   ├── dependencies.rs  # Dependency resolution
│   │   │   └── manifest.rs  # Manifest parsing
│   │   ├── python_bridge/   # Python integration
│   │   │   ├── api.rs       # Python API
│   │   │   ├── execution.rs # Python code execution
│   │   │   └── registry.rs  # Function registry
│   │   ├── app/             # Application and UI
│   │   │   ├── app.rs       # Main application
│   │   │   └── state.rs     # Application state
│   │   └── ui/              # UI components
│   ├── mods/                # Mods directory
│   │   └── Core/            # Base mod
│   │       ├── manifest.toml  # Mod configuration
│   │       └── main.py      # Mod code in Python
│   └── Cargo.toml           # Project dependencies
```

## 🔌 Creating Mods

### Mod Structure

Each mod consists of:
1. **manifest.toml** — configuration file
2. **main.py** — main Python script

### Example manifest.toml

```toml
[mod_info]
id = "my_mod"
name = "My Awesome Mod"
version = "0.1.0"
authors = ["Your Name"]
main = "main.py"
load_order = 100

[dependencies]
# id = "version"
# core = "0.1.0"
```

### Example main.py

```python
import text_engien

def my_function():
    text_engien.print_to_console("Hello from my mod!")

def run():
    # Register functions
    text_engien.register_function("my_func", my_function)
    
    # Create a tab
    text_engien.register_tab("my_tab", "My Tab", "🎮")
    
    # Add dropdown menu
    text_engien.add_dropdown_menu("my_tab", "Actions", "⚡")
    
    # Add menu items
    text_engien.add_menu_item("my_tab", "Actions", "Do Something", "my_func", "✨")
```

## 📚 Python API

### Available Functions

- [`print_to_console(message)`](text_engine/mods/Core/main.py:5) — print message to console
- [`register_function(name, func)`](text_engine/mods/Core/main.py:48) — register Python function
- [`register_tab(id, title, icon)`](text_engine/mods/Core/main.py:60) — create new tab
- [`add_dropdown_menu(tab_id, name, icon)`](text_engine/mods/Core/main.py:63) — add dropdown menu
- [`add_menu_item(tab_id, menu, label, function, icon)`](text_engine/mods/Core/main.py:64) — add menu item

## 🛠️ Technology Stack

- **Rust** — systems programming
- **PyO3** — Python ↔ Rust integration
- **egui** — graphical interface
- **eframe** — application framework
- **Legion** — Entity Component System (ECS)
- **TOML** — configuration files
- **Serde** — serialization/deserialization

## 📊 Current Status

### ✅ Implemented
- Basic mod loading system
- Dependency resolution between mods
- Python API for creating UI elements
- Tab and menu system
- Output console
- Core mod with usage examples

### 🚧 In Development
- Text/code editor
- Save/load system
- Advanced UI components
- API documentation
- Plugin system

### 📋 Planned
- Full-featured TUI rendering system
- Event system
- Audio subsystem
- Networking capabilities
- Export to executable files

## 🙏 Call for Help

You've made it this far, which means you see the potential of this project. But let's be honest — right now, it's still very raw. There's a mountain of work to be done, and I can't do it alone.

If you have any experience with Rust, Python, game development, or just a passion for TUI games — **I personally ask you to consider helping**. Whether it's writing code, fixing bugs, improving documentation, or just sharing ideas — every bit of help makes a world of difference.

This project is a labor of love, and with your support, we can turn this dream into reality. Please, help me make this engine something special.

## 🤝 How to Help

1. ⭐ Star the project on GitHub
2. 🐛 Report bugs via Issues
3. 💡 Suggest new features
4. 🔧 Submit Pull Requests
5. 📖 Improve documentation
6. 🎮 Create mods and share them

## 📄 License

This project is in active development. License information will be added later.

---

Thank you for your time and attention. I hope to see you in the list of contributors! ❤️