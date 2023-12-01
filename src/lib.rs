// Taken from https://pyo3.rs/v0.20.0/module.html?highlight=submodule#python-submodules
use pyo3::prelude::*;

#[pymodule]
fn pdoc_pyo3_sample_library(py: Python<'_>, m: &PyModule) -> PyResult<()> {

    let child_module = PyModule::new(py, "child_module")?;
    child_module.add_function(wrap_pyfunction!(func, child_module)?)?;
    m.add_submodule(child_module)?;

    Ok(())
}

#[pyfunction]
fn func() -> String {
    "func".to_string()
}
