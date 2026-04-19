use pyo3::prelude::*;
use pyo3::types::PyModule;

#[derive(Debug, serde::Deserialize)]
struct Data {
        name: String,
        value: i32,
}


fn sum_core(input: &str) -> i32 {
        let parsed: Data = serde_json::from_str(input).unwrap();
        parsed.name.len() as i32 + parsed.value 
}

#[pyfunction]
fn sum(input: &str) -> PyResult<i32> {
//      let parsed: Data = serde_json::from_str(input).unwrap();
//              .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

//      Ok(parsed.name.len() as i32 + parsed.value)

        Ok(sum_core(input))
}

#[pymodule]
fn rust_json(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(sum, m)?)?;

        Ok(())
}

#[cfg(test)]
mod tests {
        use super::sum_core;

        #[test]
        fn test_stokes_baker() {
                assert_eq!(
                        sum_core("{ \"name\": \"Stokes Baker\", \"value\": 954832 }"),
                        954844
                );

        } 

}
