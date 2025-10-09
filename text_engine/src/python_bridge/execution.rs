use std::ffi::CString;
use pyo3::prelude::*;
use crate::python_bridge::registry::REGISTERED_FUNCTIONS;

pub fn call_python_function(function_name: &str) -> PyResult<()> {
    if let Ok(functions) = REGISTERED_FUNCTIONS.lock() {
        if let Some(python_function) = functions.get(function_name) {
            Python::attach(|py| {
                python_function.function.call0(py)?;
                Ok(())
            })
        } else {
            println!("⚠️ Function '{}' not found in registered functions", function_name);
            Ok(())
        }
    } else {
        println!("❌ Error accessing registered functions");
        Ok(())
    }
}

pub fn run_python(file_path: &str) -> PyResult<()> {
    let main_content = std::fs::read_to_string(file_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to read file: {}", e)))?;

    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("main.py");

    Python::attach(|py| -> PyResult<()>{

        let main_content_cstr = CString::new(main_content).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to convert code to CString: {}", e)))?;
        let file_name_cstr = CString::new(file_name).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to convert filename to CString: {}", e)))?;
        let empty_cstr = CString::new("").unwrap();

        let run_fn: Py<PyAny> = PyModule::from_code(
            py,
            &main_content_cstr,
            &file_name_cstr,
            &empty_cstr
        )?
        .getattr("run")?
        .into();

        run_fn.call0(py)?;
        Ok(())
    })?;
    Ok(())
}

