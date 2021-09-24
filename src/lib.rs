mod utils;

use pyo3::{
    prelude::{
        Python,
        PyResult,
        PyModule,
        pyfunction,
        pymodule
    },
    wrap_pyfunction
};

use numpy::{
    PyArrayDyn,
    PyArray1,
    IntoPyArray,
    PyReadonlyArray1
};

use ndarray::{
    arr1,
    Array1
};

#[pyfunction]
fn covariance<'py>(py: Python<'py>, a: PyReadonlyArray1<f64>) -> PyResult<&'py PyArray1<f64>> {
    let d = a.to_owned_array();
    Ok(d.into_pyarray(py))
}

#[pymodule]
fn fast_correlation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(covariance, m)?)?;
    Ok(())
}