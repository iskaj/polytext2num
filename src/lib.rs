use pyo3::prelude::*;
use text2num::{Language, text2digits};

#[pyfunction]
fn text_to_number(input: &str, language: &str) -> PyResult<String> {
    let lang = match language.to_lowercase().as_str() {
        "en" | "english" => Language::english(),
        "nl" | "dutch"   => Language::dutch(),
        "fr" | "french"  => Language::french(),
        "es" | "spanish" => Language::spanish(),
        "de" | "german"  => Language::german(),
        "it" | "italian" => Language::italian(),
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Unsupported language")),
    };

    match text2digits(input, &lang) {
        Ok(number) => Ok(number),
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))),
    }
}

#[pymodule]
fn polytext2num(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(text_to_number, m)?)?;
    Ok(())
}