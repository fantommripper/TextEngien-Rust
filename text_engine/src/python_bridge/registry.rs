use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::python_bridge::types::{TabDefinition, ButtonDefinition, PythonFunction};
use crate::python_bridge::window_types::{WindowDefinition, ElementState};

lazy_static! {
    pub static ref REGISTERED_TABS: Mutex<Vec<TabDefinition>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_BUTTONS: Mutex<Vec<ButtonDefinition>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_FUNCTIONS: Mutex<HashMap<String, PythonFunction>> = Mutex::new(HashMap::new());
    pub static ref REGISTERED_WINDOWS: Mutex<HashMap<String, WindowDefinition>> = Mutex::new(HashMap::new());
    pub static ref WINDOW_STATES: Mutex<HashMap<String, ElementState>> = Mutex::new(HashMap::new());
}

