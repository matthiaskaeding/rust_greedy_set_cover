use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::{HashMap, HashSet}; // Make sure this use statement is present

mod greedy_set_cover;

#[pymodule]
fn setcover_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greedy_set_cover_py, m)?)?;
    Ok(())
}

#[pyfunction]
fn greedy_set_cover_py(sets: HashMap<String, Vec<i32>>, algo: i16) -> PyResult<HashSet<String>> {
    Ok(greedy_set_cover::greedy_set_cover(&sets, algo))
}
