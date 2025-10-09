use egui::{Context, Window, Ui};
use crate::python_bridge::window_types::*;
use crate::python_bridge::registry::{REGISTERED_WINDOWS, WINDOW_STATES};


pub fn render_windows(ctx: &Context, action_callback: &dyn Fn(&str)) {
    let windows = REGISTERED_WINDOWS.lock().unwrap().clone();
    
    for (window_id, window) in windows.iter() {
        if window.visible {
            render_window(ctx, window_id, window, action_callback);
        }
    }
}

fn render_window(ctx: &Context, window_id: &str, window: &WindowDefinition, action_callback: &dyn Fn(&str)) {
    let mut window_builder = Window::new(&window.title);
    
    // Применяем настройки окна
    if let Some((x, y)) = window.position {
        window_builder = window_builder.default_pos([x, y]);
    }
    
    if let Some((width, height)) = window.size {
        window_builder = window_builder.default_size([width, height]);
    }
    
    window_builder = window_builder
        .resizable(window.resizable)
        .collapsible(window.collapsible)
        .scroll(window.scroll);
    
    window_builder.show(ctx, |ui| {
        render_elements(ui, window_id, &window.elements, action_callback);
    });
}

fn render_elements(ui: &mut Ui, window_id: &str, elements: &[UiElement], action_callback: &dyn Fn(&str)) {
    for element in elements {
        render_element(ui, window_id, element, action_callback);
    }
}

fn render_element(ui: &mut Ui, window_id: &str, element: &UiElement, action_callback: &dyn Fn(&str)) {
    match element {
        UiElement::Label(label) => {
            if label.selectable {
                let _ = ui.selectable_label(false, &label.text);
            } else if label.wrap {
                ui.label(&label.text);
            } else {
                ui.label(&label.text);
            }
        }
        
        UiElement::Button(button) => {
            let btn = if button.small {
                ui.small_button(&button.text)
            } else {
                ui.button(&button.text)
            };
            
            if btn.clicked() {
                if let Some(func_name) = &button.function_name {
                    action_callback(func_name);
                }
            }
        }
        
        UiElement::TextEdit(text_edit) => {
            let mut states = WINDOW_STATES.lock().unwrap();
            let state = states.entry(window_id.to_string()).or_insert_with(ElementState::new);
            let text = state.text_values.entry(text_edit.id.clone()).or_insert_with(String::new);
            
            let mut text_edit_widget = egui::TextEdit::singleline(text);
            
            if text_edit.multiline {
                text_edit_widget = egui::TextEdit::multiline(text);
            }
            
            if text_edit.password {
                text_edit_widget = text_edit_widget.password(true);
            }
            
            if let Some(hint) = &text_edit.hint {
                text_edit_widget = text_edit_widget.hint_text(hint);
            }
            
            ui.add(text_edit_widget);
        }
        
        UiElement::Checkbox(checkbox) => {
            let mut states = WINDOW_STATES.lock().unwrap();
            let state = states.entry(window_id.to_string()).or_insert_with(ElementState::new);
            let value = state.bool_values.entry(checkbox.id.clone()).or_insert(false);
            
            ui.checkbox(value, &checkbox.label);
        }
        
        UiElement::RadioButton(radio) => {
            let mut states = WINDOW_STATES.lock().unwrap();
            let state = states.entry(window_id.to_string()).or_insert_with(ElementState::new);
            let value = state.bool_values.entry(radio.id.clone()).or_insert(false);
            
            if ui.radio(*value, &radio.label).clicked() {
                *value = !*value;
            }
        }
        
        UiElement::Slider(slider) => {
            let mut states = WINDOW_STATES.lock().unwrap();
            let state = states.entry(window_id.to_string()).or_insert_with(ElementState::new);
            let value = state.float_values.entry(slider.id.clone()).or_insert(slider.min);
            
            if slider.integer {
                let mut int_value = *value as i64;
                let slider_widget = egui::Slider::new(&mut int_value, slider.min as i64..=slider.max as i64);
                
                if let Some(label) = &slider.label {
                    ui.add(slider_widget.text(label));
                } else {
                    ui.add(slider_widget);
                }
                
                *value = int_value as f64;
            } else {
                let slider_widget = egui::Slider::new(value, slider.min..=slider.max);
                
                if let Some(label) = &slider.label {
                    ui.add(slider_widget.text(label));
                } else {
                    ui.add(slider_widget);
                }
            }
        }
        
        UiElement::DragValue(drag) => {
            let mut states = WINDOW_STATES.lock().unwrap();
            let state = states.entry(window_id.to_string()).or_insert_with(ElementState::new);
            let value = state.float_values.entry(drag.id.clone()).or_insert(0.0);
            
            let drag_widget = egui::DragValue::new(value).speed(drag.speed);
            
            if let Some(label) = &drag.label {
                ui.label(label);
                ui.add(drag_widget);
            } else {
                ui.add(drag_widget);
            }
        }
        
        UiElement::ComboBox(combo) => {
            let mut states = WINDOW_STATES.lock().unwrap();
            let state = states.entry(window_id.to_string()).or_insert_with(ElementState::new);
            let selected = state.combo_values.entry(combo.id.clone()).or_insert(0);
            
            let selected_text = combo.options.get(*selected)
                .map(|s| s.as_str())
                .unwrap_or("None");
            
            egui::ComboBox::from_label(&combo.label)
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    for (i, option) in combo.options.iter().enumerate() {
                        ui.selectable_value(selected, i, option);
                    }
                });
        }
        
        UiElement::Separator => {
            ui.separator();
        }
        
        UiElement::Spacing(amount) => {
            ui.add_space(*amount);
        }
        
        UiElement::Heading(heading) => {
            match heading.level {
                1 => ui.heading(&heading.text),
                2 => ui.label(egui::RichText::new(&heading.text).heading().strong()),
                3 => ui.label(egui::RichText::new(&heading.text).strong()),
                _ => ui.label(&heading.text),
            };
        }
        
        UiElement::Hyperlink(link) => {
            ui.hyperlink_to(&link.text, &link.url);
        }
        
        UiElement::Image(_image) => {
            // Для изображений потребуется дополнительная логика загрузки
            ui.label("[Image placeholder]");
        }
        
        UiElement::ProgressBar(progress) => {
            let states = WINDOW_STATES.lock().unwrap();
            let state = states.get(window_id);
            let value = state
                .and_then(|s| s.float_values.get(&progress.id))
                .copied()
                .unwrap_or(0.0);
            
            let progress_bar = egui::ProgressBar::new(value as f32);
            
            if progress.show_percentage {
                ui.add(progress_bar.show_percentage());
            } else {
                ui.add(progress_bar);
            }
        }
        
        UiElement::Spinner(spinner) => {
            ui.add(egui::Spinner::new().size(spinner.size));
        }
        
        UiElement::ColorPicker(picker) => {
            let mut states = WINDOW_STATES.lock().unwrap();
            let state = states.entry(window_id.to_string()).or_insert_with(ElementState::new);
            let color = state.color_values.entry(picker.id.clone()).or_insert([1.0, 1.0, 1.0, 1.0]);
            
            if picker.alpha {
                ui.color_edit_button_rgba_unmultiplied(color);
            } else {
                let mut rgb = [color[0], color[1], color[2]];
                ui.color_edit_button_rgb(&mut rgb);
                color[0] = rgb[0];
                color[1] = rgb[1];
                color[2] = rgb[2];
            }
            
            ui.label(&picker.label);
        }
        
        UiElement::HorizontalLayout(elements) => {
            ui.horizontal(|ui| {
                render_elements(ui, window_id, elements, action_callback);
            });
        }
        
        UiElement::VerticalLayout(elements) => {
            ui.vertical(|ui| {
                render_elements(ui, window_id, elements, action_callback);
            });
        }
        
        UiElement::Group(group) => {
            ui.group(|ui| {
                render_elements(ui, window_id, &group.elements, action_callback);
            });
        }
        
        UiElement::CollapsingHeader(header) => {
            egui::CollapsingHeader::new(&header.text)
                .default_open(header.default_open)
                .show(ui, |ui| {
                    render_elements(ui, window_id, &header.elements, action_callback);
                });
        }
    }
}