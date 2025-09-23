use std::ffi::CString;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn print_to_console(msg: &str) -> PyResult<()> {
    println!("[Python Mod] {}", msg);
    Ok(())
}

#[pymodule(name = "text_engien")]
fn text_engien(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_to_console, m)?)?;
    Ok(())
}

pub fn inite_module(){
    pyo3::append_to_inittab!(text_engien);
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