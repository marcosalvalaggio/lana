mod matrix;
use matrix::Matrix; 

use pyo3::prelude::*;

#[pymodule]
fn lana(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Matrix>()?;
    Ok(())
}