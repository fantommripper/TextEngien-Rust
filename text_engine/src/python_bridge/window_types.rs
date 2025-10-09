use serde::{Deserialize, Serialize};

/// Основная структура окна
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowDefinition {
    pub id: String,
    pub title: String,
    pub visible: bool,
    pub position: Option<(f32, f32)>,
    pub size: Option<(f32, f32)>,
    pub resizable: bool,
    pub collapsible: bool,
    pub scroll: bool,
    pub elements: Vec<UiElement>,
}

impl WindowDefinition {
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            visible: true,
            position: None,
            size: None,
            resizable: true,
            collapsible: true,
            scroll: true,
            elements: Vec::new(),
        }
    }
}

/// Перечисление всех возможных UI элементов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiElement {
    Label(LabelElement),
    Button(ButtonElement),
    TextEdit(TextEditElement),
    Checkbox(CheckboxElement),
    RadioButton(RadioButtonElement),
    Slider(SliderElement),
    DragValue(DragValueElement),
    ComboBox(ComboBoxElement),
    Separator,
    Spacing(f32),
    Heading(HeadingElement),
    Hyperlink(HyperlinkElement),
    Image(ImageElement),
    ProgressBar(ProgressBarElement),
    Spinner(SpinnerElement),
    ColorPicker(ColorPickerElement),
    HorizontalLayout(Vec<UiElement>),
    VerticalLayout(Vec<UiElement>),
    Group(GroupElement),
    CollapsingHeader(CollapsingHeaderElement),
}

// === Элементы UI ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelElement {
    pub text: String,
    pub wrap: bool,
    pub selectable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonElement {
    pub text: String,
    pub function_name: Option<String>,
    pub small: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEditElement {
    pub id: String,
    pub multiline: bool,
    pub hint: Option<String>,
    pub password: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckboxElement {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioButtonElement {
    pub id: String,
    pub label: String,
    pub group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderElement {
    pub id: String,
    pub label: Option<String>,
    pub min: f64,
    pub max: f64,
    pub integer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragValueElement {
    pub id: String,
    pub label: Option<String>,
    pub speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboBoxElement {
    pub id: String,
    pub label: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingElement {
    pub text: String,
    pub level: u8, // 1, 2, or 3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperlinkElement {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
    pub path: String,
    pub size: Option<(f32, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBarElement {
    pub id: String,
    pub show_percentage: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinnerElement {
    pub size: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPickerElement {
    pub id: String,
    pub label: String,
    pub alpha: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupElement {
    pub elements: Vec<UiElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapsingHeaderElement {
    pub text: String,
    pub default_open: bool,
    pub elements: Vec<UiElement>,
}

// === Состояния элементов ===

#[derive(Debug, Clone)]
pub struct ElementState {
    pub text_values: std::collections::HashMap<String, String>,
    pub bool_values: std::collections::HashMap<String, bool>,
    pub float_values: std::collections::HashMap<String, f64>,
    pub combo_values: std::collections::HashMap<String, usize>,
    pub color_values: std::collections::HashMap<String, [f32; 4]>,
}

impl ElementState {
    pub fn new() -> Self {
        Self {
            text_values: std::collections::HashMap::new(),
            bool_values: std::collections::HashMap::new(),
            float_values: std::collections::HashMap::new(),
            combo_values: std::collections::HashMap::new(),
            color_values: std::collections::HashMap::new(),
        }
    }
}