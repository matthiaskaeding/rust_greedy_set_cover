use pyo3::prelude::*;
use std::collections::HashMap;

use setcover_core::greedy_set_cover;

#[pyfunction]
fn greedy_set_cover_string_i64(
    sets: HashMap<String, Vec<i64>>,
    algo: i16,
) -> PyResult<Vec<String>> {
    Ok(greedy_set_cover(&sets, algo))
}

#[pyfunction]
fn greedy_set_cover_string_string(
    sets: HashMap<String, Vec<String>>,
    algo: i16,
) -> PyResult<Vec<String>> {
    Ok(greedy_set_cover(&sets, algo))
}

#[pyfunction]
fn greedy_set_cover_i64_i64(sets: HashMap<i64, Vec<i64>>, algo: i16) -> PyResult<Vec<i64>> {
    Ok(greedy_set_cover(&sets, algo))
}

#[pyfunction]
fn greedy_set_cover_i64_string(sets: HashMap<i64, Vec<String>>, algo: i16) -> PyResult<Vec<i64>> {
    Ok(greedy_set_cover(&sets, algo))
}

#[pymodule]
fn _setcover_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greedy_set_cover_string_i64, m)?)?;
    m.add_function(wrap_pyfunction!(greedy_set_cover_string_string, m)?)?;
    m.add_function(wrap_pyfunction!(greedy_set_cover_i64_i64, m)?)?;
    m.add_function(wrap_pyfunction!(greedy_set_cover_i64_string, m)?)?;
    Ok(())
}
