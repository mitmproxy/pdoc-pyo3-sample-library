// Taken from https://pyo3.rs/v0.20.0/module.html?highlight=submodule#python-submodules
use pyo3::prelude::*;

#[pymodule]
fn pdoc_pyo3_sample_library(py: Python<'_>, m: &PyModule) -> PyResult<()> {

    let submodule = PyModule::new(py, "submodule")?;
    submodule.setattr("__doc__", "This is a barebone PyO3 submodule with a child module.")?;
    m.add_submodule(submodule)?;

    let subsubmodule = PyModule::new(py, "subsubmodule")?;
    subsubmodule.setattr("__doc__", "This is a sub-submodule with a function member.")?;
    subsubmodule.add_function(wrap_pyfunction!(func, subsubmodule)?)?;
    submodule.add_submodule(subsubmodule)?;

    let explicit_submodule = PyModule::new(py, "explicit_submodule")?;
    explicit_submodule.setattr("__doc__", "This is a submodule that has been explicitly registered in sys.modules.")?;
    m.add_submodule(explicit_submodule)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("pdoc_pyo3_sample_library.explicit_submodule", explicit_submodule)?;

    let correct_name_submodule = PyModule::new(py, "correct_name_submodule")?;
    correct_name_submodule.setattr("__doc__", "This is a submodule with a patched __name__ to include the parent module name.")?;
    m.add_submodule(correct_name_submodule)?;
    // Needs to happen after .add_submodule()
    correct_name_submodule.setattr("__name__", "pdoc_pyo3_sample_library.correct_name_submodule")?;

    Ok(())
}

#[pyfunction]
fn func() -> String {
    "func".to_string()
}
