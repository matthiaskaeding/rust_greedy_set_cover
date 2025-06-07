use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

use setcover_core::greedy_set_cover;

#[pyfunction]
fn greedy_set_cover_py(sets: HashMap<String, Vec<i32>>, algo: i16) -> PyResult<HashSet<String>> {
    Ok(greedy_set_cover(&sets, algo))
}

#[pymodule]
fn _setcover_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // <-- RENAMED THIS FUNCTION
    m.add_function(wrap_pyfunction!(greedy_set_cover_py, m)?)?;
    Ok(())
}
