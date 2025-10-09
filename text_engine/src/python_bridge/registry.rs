use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::python_bridge::types::{TabDefinition, ButtonDefinition, PythonFunction};

lazy_static! {
    pub static ref REGISTERED_TABS: Mutex<Vec<TabDefinition>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_BUTTONS: Mutex<Vec<ButtonDefinition>> = Mutex::new(Vec::new());
    pub static ref REGISTERED_FUNCTIONS: Mutex<HashMap<String, PythonFunction>> = Mutex::new(HashMap::new());
}

