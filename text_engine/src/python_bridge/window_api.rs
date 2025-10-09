use pyo3::prelude::*;
use crate::python_bridge::window_types::*;
use crate::python_bridge::registry::{REGISTERED_WINDOWS, WINDOW_STATES};

// === Управление окнами ===

#[pyfunction]
pub fn create_window(id: String, title: String) -> PyResult<()> {
    let window = WindowDefinition::new(id.clone(), title);
    REGISTERED_WINDOWS.lock().unwrap().insert(id, window);
    Ok(())
}

#[pyfunction]
pub fn show_window(id: String) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&id) {
        window.visible = true;
    }
    Ok(())
}

#[pyfunction]
pub fn hide_window(id: String) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&id) {
        window.visible = false;
    }
    Ok(())
}

#[pyfunction]
pub fn set_window_position(id: String, x: f32, y: f32) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&id) {
        window.position = Some((x, y));
    }
    Ok(())
}

#[pyfunction]
pub fn set_window_size(id: String, width: f32, height: f32) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&id) {
        window.size = Some((width, height));
    }
    Ok(())
}

#[pyfunction]
pub fn set_window_resizable(id: String, resizable: bool) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&id) {
        window.resizable = resizable;
    }
    Ok(())
}

#[pyfunction]
pub fn set_window_collapsible(id: String, collapsible: bool) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&id) {
        window.collapsible = collapsible;
    }
    Ok(())
}

#[pyfunction]
pub fn set_window_scroll(id: String, scroll: bool) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&id) {
        window.scroll = scroll;
    }
    Ok(())
}

// === Добавление элементов в окно ===

#[pyfunction]
pub fn add_label(window_id: String, text: String, wrap: Option<bool>, selectable: Option<bool>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Label(LabelElement {
            text,
            wrap: wrap.unwrap_or(false),
            selectable: selectable.unwrap_or(false),
        }));
    }
    Ok(())
}

#[pyfunction]
pub fn add_button(window_id: String, text: String, function_name: Option<String>, small: Option<bool>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Button(ButtonElement {
            text,
            function_name,
            small: small.unwrap_or(false),
        }));
    }
    Ok(())
}

#[pyfunction]
pub fn add_text_edit(
    window_id: String,
    id: String,
    multiline: Option<bool>,
    hint: Option<String>,
    password: Option<bool>,
) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::TextEdit(TextEditElement {
            id: id.clone(),
            multiline: multiline.unwrap_or(false),
            hint,
            password: password.unwrap_or(false),
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.text_values.entry(id).or_insert_with(String::new);
    }
    Ok(())
}

#[pyfunction]
pub fn add_checkbox(window_id: String, id: String, label: String) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Checkbox(CheckboxElement {
            id: id.clone(),
            label,
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.bool_values.entry(id).or_insert(false);
    }
    Ok(())
}

#[pyfunction]
pub fn add_radio_button(window_id: String, id: String, label: String, group: String) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::RadioButton(RadioButtonElement {
            id: id.clone(),
            label,
            group,
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.bool_values.entry(id).or_insert(false);
    }
    Ok(())
}

#[pyfunction]
pub fn add_slider(
    window_id: String,
    id: String,
    min: f64,
    max: f64,
    label: Option<String>,
    integer: Option<bool>,
) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Slider(SliderElement {
            id: id.clone(),
            label,
            min,
            max,
            integer: integer.unwrap_or(false),
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.float_values.entry(id).or_insert(min);
    }
    Ok(())
}

#[pyfunction]
pub fn add_drag_value(
    window_id: String,
    id: String,
    label: Option<String>,
    speed: Option<f64>,
) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::DragValue(DragValueElement {
            id: id.clone(),
            label,
            speed: speed.unwrap_or(1.0),
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.float_values.entry(id).or_insert(0.0);
    }
    Ok(())
}

#[pyfunction]
pub fn add_combo_box(window_id: String, id: String, label: String, options: Vec<String>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::ComboBox(ComboBoxElement {
            id: id.clone(),
            label,
            options,
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.combo_values.entry(id).or_insert(0);
    }
    Ok(())
}

#[pyfunction]
pub fn add_separator(window_id: String) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Separator);
    }
    Ok(())
}

#[pyfunction]
pub fn add_spacing(window_id: String, amount: f32) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Spacing(amount));
    }
    Ok(())
}

#[pyfunction]
pub fn add_heading(window_id: String, text: String, level: Option<u8>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Heading(HeadingElement {
            text,
            level: level.unwrap_or(1).clamp(1, 3),
        }));
    }
    Ok(())
}

#[pyfunction]
pub fn add_hyperlink(window_id: String, text: String, url: String) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Hyperlink(HyperlinkElement {
            text,
            url,
        }));
    }
    Ok(())
}

#[pyfunction]
pub fn add_image(window_id: String, path: String, width: Option<f32>, height: Option<f32>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        let size = if width.is_some() && height.is_some() {
            Some((width.unwrap(), height.unwrap()))
        } else {
            None
        };
        
        window.elements.push(UiElement::Image(ImageElement {
            path,
            size,
        }));
    }
    Ok(())
}

#[pyfunction]
pub fn add_progress_bar(window_id: String, id: String, show_percentage: Option<bool>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::ProgressBar(ProgressBarElement {
            id: id.clone(),
            show_percentage: show_percentage.unwrap_or(true),
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.float_values.entry(id).or_insert(0.0);
    }
    Ok(())
}

#[pyfunction]
pub fn add_spinner(window_id: String, size: Option<f32>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::Spinner(SpinnerElement {
            size: size.unwrap_or(20.0),
        }));
    }
    Ok(())
}

#[pyfunction]
pub fn add_color_picker(window_id: String, id: String, label: String, alpha: Option<bool>) -> PyResult<()> {
    if let Some(window) = REGISTERED_WINDOWS.lock().unwrap().get_mut(&window_id) {
        window.elements.push(UiElement::ColorPicker(ColorPickerElement {
            id: id.clone(),
            label,
            alpha: alpha.unwrap_or(false),
        }));
        
        // Инициализируем состояние
        let mut states = WINDOW_STATES.lock().unwrap();
        let state = states.entry(window_id.clone()).or_insert_with(ElementState::new);
        state.color_values.entry(id).or_insert([1.0, 1.0, 1.0, 1.0]);
    }
    Ok(())
}

// === Получение значений элементов ===

#[pyfunction]
pub fn get_text_value(window_id: String, element_id: String) -> PyResult<String> {
    let states = WINDOW_STATES.lock().unwrap();
    if let Some(state) = states.get(&window_id) {
        if let Some(value) = state.text_values.get(&element_id) {
            return Ok(value.clone());
        }
    }
    Ok(String::new())
}

#[pyfunction]
pub fn get_bool_value(window_id: String, element_id: String) -> PyResult<bool> {
    let states = WINDOW_STATES.lock().unwrap();
    if let Some(state) = states.get(&window_id) {
        if let Some(value) = state.bool_values.get(&element_id) {
            return Ok(*value);
        }
    }
    Ok(false)
}

#[pyfunction]
pub fn get_float_value(window_id: String, element_id: String) -> PyResult<f64> {
    let states = WINDOW_STATES.lock().unwrap();
    if let Some(state) = states.get(&window_id) {
        if let Some(value) = state.float_values.get(&element_id) {
            return Ok(*value);
        }
    }
    Ok(0.0)
}

#[pyfunction]
pub fn get_combo_value(window_id: String, element_id: String) -> PyResult<usize> {
    let states = WINDOW_STATES.lock().unwrap();
    if let Some(state) = states.get(&window_id) {
        if let Some(value) = state.combo_values.get(&element_id) {
            return Ok(*value);
        }
    }
    Ok(0)
}

#[pyfunction]
pub fn get_color_value(window_id: String, element_id: String) -> PyResult<Vec<f32>> {
    let states = WINDOW_STATES.lock().unwrap();
    if let Some(state) = states.get(&window_id) {
        if let Some(value) = state.color_values.get(&element_id) {
            return Ok(value.to_vec());
        }
    }
    Ok(vec![1.0, 1.0, 1.0, 1.0])
}

// === Установка значений элементов ===

#[pyfunction]
pub fn set_text_value(window_id: String, element_id: String, value: String) -> PyResult<()> {
    let mut states = WINDOW_STATES.lock().unwrap();
    let state = states.entry(window_id).or_insert_with(ElementState::new);
    state.text_values.insert(element_id, value);
    Ok(())
}

#[pyfunction]
pub fn set_bool_value(window_id: String, element_id: String, value: bool) -> PyResult<()> {
    let mut states = WINDOW_STATES.lock().unwrap();
    let state = states.entry(window_id).or_insert_with(ElementState::new);
    state.bool_values.insert(element_id, value);
    Ok(())
}

#[pyfunction]
pub fn set_float_value(window_id: String, element_id: String, value: f64) -> PyResult<()> {
    let mut states = WINDOW_STATES.lock().unwrap();
    let state = states.entry(window_id).or_insert_with(ElementState::new);
    state.float_values.insert(element_id, value);
    Ok(())
}

#[pyfunction]
pub fn set_combo_value(window_id: String, element_id: String, value: usize) -> PyResult<()> {
    let mut states = WINDOW_STATES.lock().unwrap();
    let state = states.entry(window_id).or_insert_with(ElementState::new);
    state.combo_values.insert(element_id, value);
    Ok(())
}

#[pyfunction]
pub fn set_color_value(window_id: String, element_id: String, value: Vec<f32>) -> PyResult<()> {
    let mut states = WINDOW_STATES.lock().unwrap();
    let state = states.entry(window_id).or_insert_with(ElementState::new);
    if value.len() >= 4 {
        state.color_values.insert(element_id, [value[0], value[1], value[2], value[3]]);
    }
    Ok(())
}