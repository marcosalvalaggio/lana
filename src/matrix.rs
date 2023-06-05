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
    pub fn full(dimensions: (usize, usize), fill_value: f64) -> Self {
        let (rows, cols) = dimensions;
        let data: Vec<Vec<f64>> = vec![vec![fill_value; cols]; rows];
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

    pub fn to_list(&self) -> Vec<Vec<f64>> {
        self.data.clone()
    }

    pub fn __repr__(&self) -> String {
        let mut result = String::new();
        for (i, row) in self.data.iter().enumerate() {
            if i == 0 {
                result.push_str("Matrix([");
            }
            result.push_str("[");
            for (j, val) in row.iter().enumerate() {
                if j != 0 {
                    result.push_str(", ");
                }
                result.push_str(&val.to_string());
            }
            result.push_str("]");
            if i != self.data.len() - 1 {
                result.push_str(",\n       ");
            } else {
                result.push_str("])");
            }
        }
        result
    }    

}