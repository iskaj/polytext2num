use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use text2num::{Language, text2digits, replace_numbers_in_text};

#[pyfunction]
fn text2num(input: &str, language: &str, threshold: Option<f64>) -> PyResult<String> {
    let lang = get_language(language)?;
    let threshold = threshold.unwrap_or(20.0);
    Ok(replace_numbers_in_text(input, &lang, threshold))
}

// Internal helper to map string to Language
fn get_language(lang: &str) -> PyResult<Language> {
    match lang.to_lowercase().as_str() {
        "en" | "english" => Ok(Language::english()),
        "nl" | "dutch" => Ok(Language::dutch()),
        "fr" | "french" => Ok(Language::french()),
        "es" | "spanish" => Ok(Language::spanish()),
        "de" | "german" => Ok(Language::german()),
        "it" | "italian" => Ok(Language::italian()),
        _ => Err(PyValueError::new_err("Unsupported language")),
    }
}

#[pymodule]
fn polytext2num(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(text2num, m)?)?;
    Ok(())
}