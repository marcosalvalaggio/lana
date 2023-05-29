use pyo3::prelude::*;

#[pyclass]
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

    pub fn print(&self) {
        for row in &self.data {
            for i in row {
                print!("{} ", i);
            }
            println!();
        }
    }

}


#[pymodule]
fn lana(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Matrix>()?;
    Ok(())
}