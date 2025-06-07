use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

// This is the key line. It imports the public `greedy_set_cover` function
// from your other local crate, `setcover-core`. This works because your
// `py-setcover/Cargo.toml` has the dependency:
// `setcover-core = { path = "../crates/setcover-core" }`
use setcover_core::greedy_set_cover;

/// A Python wrapper for the Rust greedy_set_cover function.
/// It takes a Python dictionary and an integer, and returns a Python set.
/// The types are automatically converted by PyO3.
#[pyfunction]
fn greedy_set_cover_py(sets: HashMap<String, Vec<i32>>, algo: i16) -> PyResult<HashSet<String>> {
    // The only thing this function does is call the real implementation
    // from the core library and wrap the result in PyResult::Ok.
    Ok(greedy_set_cover(&sets, algo))
}

/// The Python module definition.
/// This is the entry point that Python uses when it imports your module.
/// It registers the `greedy_set_cover_py` function so it can be called from Python.
#[pymodule]
fn setcover(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greedy_set_cover_py, m)?)?;
    Ok(())
}
