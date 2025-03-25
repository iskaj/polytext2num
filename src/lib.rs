use pyo3::prelude::*;
use text2num::{Language, text2digits};

#[pyfunction]
fn dutch_to_number(input: &str) -> PyResult<String> {
    let lang = Language::dutch();

    match text2digits(input, &lang) {
        Ok(number) => Ok(number),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))),
    }
}

#[pymodule]
fn dutchtext2num(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dutch_to_number, m)?)?;
    Ok(())
}
