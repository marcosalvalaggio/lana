use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Matrix {
    #[pyo3(get, set)]
    data: Vec<Vec<f64>>,
}


#[pymethods]
impl Matrix {

    #[staticmethod]
    pub fn zeros(dimensions: (usize, usize)) -> Self {
        let (rows, cols) = dimensions;
        let data: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];
        Matrix { data }
    }

    #[staticmethod]
    pub fn ones(dimensions: (usize, usize)) -> Self {
        let (rows, cols) = dimensions;
        let data: Vec<Vec<f64>> = vec![vec![1.0; cols]; rows];
        Matrix { data }
    }

    #[staticmethod]
    pub fn eye(size: usize) -> Self {
        let data: Vec<Vec<f64>> = (0..size).map(|i| {
            (0..size).map(|j| if i==j {1.0} else {0.0}).collect()
        }).collect();
        Matrix { data }
    }

    #[staticmethod]
    pub fn matrix(data: Vec<Vec<f64>>) -> Self {
        Matrix { data } 
    }

    #[getter]
    pub fn shape(&self) -> (usize, usize) {
        let rows = self.data.len();
        let cols = self.data[0].len();
        (rows, cols)
    }

    pub fn __getitem__(&self, index: usize) -> PyResult<Vec<f64>> {
        match self.data.get(index) {
            Some(row) => Ok(row.clone()),
            None => Err(pyo3::exceptions::PyIndexError::new_err("Index out of range")),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.data))
    }

}