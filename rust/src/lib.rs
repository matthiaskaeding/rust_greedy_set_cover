// Add the cfg attribute to the pyo3 imports
#[cfg(feature = "extension-module")]
use pyo3::prelude::*;
#[cfg(feature = "extension-module")]
use pyo3::types::PyModule;


// This module contains your core logic and tests, it should always be included.
mod greedy_set_cover;

// Add the cfg attribute to the pymodule block
#[cfg(feature = "extension-module")]
#[pymodule]
fn setcover_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greedy_set_cover_py, m)?)?;
    Ok(())
}

// Add the cfg attribute to the pyfunction wrapper
#[cfg(feature = "extension-module")]
#[pyfunction]
fn greedy_set_cover_py(sets: HashMap<String, Vec<i32>>, algo: i16) -> PyResult<HashSet<String>> {
    Ok(greedy_set_cover::greedy_set_cover(&sets, algo))
}
