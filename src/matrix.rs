use pyo3::prelude::*;
use rayon::prelude::*;

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

    pub fn __add__(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.data.len(), other.data.len());
        assert_eq!(self.data[0].len(), other.data[0].len());
        // VANILLA 
        // let mut result: Vec<Vec<f64>> = vec![vec![0.0; self.data[0].len()]; self.data.len()];
        // for i in 0..self.data.len() {
        //     for j in 0..self.data[0].len() {
        //         result[i][j] = self.data[i][j] + other.data[i][j];
        //     }
        // } 
        // Perform matrix addition in parallel
        let result: Vec<Vec<f64>> = self
            .data
            .par_iter()
            .zip(other.data.par_iter())
            .map(|(row1, row2)| {
                row1.par_iter()
                    .zip(row2.par_iter())
                    .map(|(elem1, elem2)| elem1 + elem2)
                    .collect()
            }).collect();
        Matrix { data: result }
    }

    pub fn __neg__(&self) -> Matrix {
        // VANILLA
        // let mut result: Vec<Vec<f64>> = vec![vec![0.0; self.data[0].len()]; self.data.len()];
        // for i in 0..self.data.len() {
        //     for j in 0..self.data[0].len() {
        //         result[i][j] = self.data[i][j] * -1.;
        //     }
        // } 
        let result: Vec<Vec<f64>> = self
            .data
            .par_iter()
            .map(|row| row.iter().map(|elem| -*elem).collect())
            .collect();
        Matrix { data: result }
    }

    pub fn __sub__(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.data.len(), other.data.len());
        assert_eq!(self.data[0].len(), other.data[0].len());
        // VANILLA
        // let other_neg: Matrix = other.__neg__();
        // let result: Matrix = self.__add__(&other_neg);
        // result
        let result: Vec<Vec<f64>> = self
        .data
        .par_iter()
        .zip(other.data.par_iter())
        .map(|(row1, row2)| {
            row1.par_iter()
                .zip(row2.par_iter())
                .map(|(elem1, elem2)| elem1 - elem2)
                .collect()
        }).collect();
        Matrix { data: result }
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