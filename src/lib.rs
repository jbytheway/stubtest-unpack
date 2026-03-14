use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_rust")]
mod stubtest_unpack {
    use pyo3::prelude::*;

    #[pyfunction]
    #[pyo3(signature = (*, a, b))]
    fn f1(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    #[pyo3(signature = (*, a, b))]
    fn f2(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
