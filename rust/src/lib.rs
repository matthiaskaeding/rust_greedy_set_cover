use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

mod greedy_set_cover;

#[pymodule]
fn set_cover(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greedy_set_cover_py, m)?)?;
    Ok(())
}

#[pyfunction]
fn greedy_set_cover_py(sets: HashMap<String, Vec<i32>>, algo: i16) -> PyResult<HashSet<String>> {
    Ok(greedy_set_cover::greedy_set_cover(&sets, algo))
}
