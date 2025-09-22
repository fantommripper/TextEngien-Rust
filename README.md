# Text-Engine: A TUI Game Engine

Welcome to **Text-Engine**, an ambitious project to create a powerful and flexible game engine for building Text User Interface (TUI) games. This engine is designed with a modular architecture, combining the performance of Rust with the simplicity of Python for game scripting.

## 🌟 Key Features

* **Rust Core**: The engine's core is being built in Rust, ensuring high performance, memory safety, and reliability. This solid foundation is designed to handle all the heavy lifting, from rendering to event management, so you can focus on creating your game.
* **Python Scripting**: Game logic is written in Python, offering a simple and intuitive scripting experience. We provide a custom Rust-based library that bridges the gap between the Rust core and your Python code, allowing for seamless integration.
* **TUI-Focused**: Our primary goal is to provide a top-notch experience for creating TUI games. We're building a rich set of tools and components specifically designed for text-based interfaces, making it easy to create beautiful and responsive UIs.
* **`egui` Interface**: The engine will use the powerful and easy-to-use `egui` library for its own interface, providing a modern and developer-friendly environment for managing your projects, assets, and debugging.
* **Flexible Plugin System**: At the heart of our design is an incredibly flexible plugin system. This will allow developers to extend the engine's functionality with ease, creating a rich ecosystem of tools and features. Whether you want to add a new rendering backend or integrate a third-party service, the plugin system will make it possible.

---

## 🚀 Getting Started

Currently, the project is in its early stages of development. The basic mod-loading and dependency resolution systems are in place, but there is still a long way to go. We are actively working on building out the core features and welcome any and all contributions.

### Project Structure

* `main.rs`: The entry point of the application, responsible for initializing the ECS and loading the core mods.
* `mod_loader.rs`: Handles the loading and management of mods, including parsing `manifest.toml` files.
* `dependency_resolver.rs`: A crucial component that resolves the loading order of mods based on their dependencies.
* `python_api.rs`: The bridge between Rust and Python, allowing Python mods to interact with the engine's core.

---

## 🙏 A Humble Plea for Help

You've read this far, which means you've seen the vision and the potential of this project. But let's be honest—right now, it's still very raw. There's a mountain of work to be done, and I can't do it alone.

If you have any experience with Rust, Python, game development, or even just a passion for TUI games, I am personally begging you to consider contributing. Whether it's writing code, fixing bugs, improving documentation, or just sharing your ideas, every little bit of help will make a world of difference.

This project is a labor of love, and with your support, we can turn this dream into a reality. Please, help me make this engine something special.

Thank you for your time and consideration. I hope to see you on our list of contributors soon. ❤️
