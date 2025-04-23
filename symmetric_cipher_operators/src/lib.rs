use pyo3::PyResult;
use pyo3::Python;
use pyo3::pymodule;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;

#[pymodule]
fn crypto_ops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(operator::py_sbox_component, m)?)?;
    m.add_function(wrap_pyfunction!(operator::py_xor_component, m)?)?;
    m.add_function(wrap_pyfunction!(operator::py_rotate_component, m)?)?;
    m.add_function(wrap_pyfunction!(operator::py_constant_component, m)?)?;
    m.add_function(wrap_pyfunction!(operator::py_linear_layer_component, m)?)?;
    Ok(())
}

mod operator;
