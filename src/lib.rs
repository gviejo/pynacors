use numpy::PyReadonlyArrayDyn;
use ndarray;
use numpy::{IntoPyArray, PyArray1, PyArray2, PyArrayDyn, PyReadonlyArray1};
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn is_prime(num: u32) -> bool {
    match num {
        0 | 1 => false,
        _ => {
            let limit = (num as f32).sqrt() as u32;
            (2..=limit).any(|i| num % i == 0) == false
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn pynacors(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn restrict<'py>(_py: Python<'py>, 
            t: PyReadonlyArray1<f64>,
            d: PyReadonlyArray1<f64>,
            s: PyReadonlyArray1<f64>,
            e: PyReadonlyArray1<f64>)
            // -> (&'py PyArray1<f64>, &'py PyArray1<f64>)
    {
        let time_array = t.as_array();
        let data_array = d.as_array();
        let starts = s.as_array();
        let ends = e.as_array();

        let n = time_array.len() as usize;
        let m = starts.len() as usize;

        let mut ix = ndarray::Array::<u8,_>::zeros(n);
        let mut k = 0 as usize;
        let mut t = 0 as usize;
        let mut tokeep = 0 as usize;

        while ends[k] < time_array[t] {
            k += 1;
        }

        while k < m {
            // Outside
            while t < n {
                if time_array[t] >= starts[k] {
                    break
                }
                t += 1;
            }
            // Inside
            while t < n {
                if time_array[t] > ends[k] {
                    k += 1;
                    break
                } else {
                    ix[t] = 1;
                    tokeep += 1;
                }
                t += 1;
            }
            if k == m { break }
            if t == n { break }
        }
        
        let mut new_time_array = ndarray::Array::<f64,_>::zeros(tokeep);
        let mut new_data_array = ndarray::Array::<f64,_>::zeros(tokeep);
        k = 0;
        for i in 0..n {
            if ix[i] == 1 {                
                new_time_array[k] = time_array[i];
                new_data_array[k] = data_array[i];
                k += 1;
            }
        }

        // (new_time_array.into_pyarray(py), new_data_array.into_pyarray(py))

    }

    #[pyfn(m)]
    fn max_min<'py>(py: Python<'py>, x: PyReadonlyArrayDyn<f64>) -> &'py PyArray1<f64> {
        let array = x.as_array();
        let result_array = rust_fn::max_min(&array);
        result_array.into_pyarray(py)   
    }

    #[pyfn(m)]
    fn double_and_random_perturbation(_py: Python<'_>, x: &PyArrayDyn<f64>, perturbation_scaling: f64) {
        let mut array = unsafe {x.as_array_mut()};
        rust_fn::double_and_random_perturbation(&mut array, perturbation_scaling);
    }

    #[pyfn(m)]
    fn eye<'py>(py: Python<'py>, size: usize) -> &PyArray2<f64> {
        let array = ndarray::Array::eye(size);
        array.into_pyarray(py)
    }

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    Ok(())
}



// Rust funtions
mod rust_fn {
    use ndarray::{arr1, Array1};
    use numpy::ndarray::{ArrayViewD, ArrayViewMutD};
    use ordered_float::OrderedFloat;
    use rand::Rng;

    pub fn double_and_random_perturbation(x: &mut ArrayViewMutD<'_, f64>, scaling: f64) {
        let mut rng = rand::thread_rng();
        x.iter_mut()
            .for_each(|x| *x = *x * 2. + (rng.gen::<f64>() - 0.5) * scaling);
    }

    pub fn max_min(x: &ArrayViewD<'_, f64>) -> Array1<f64> {
        if x.len() == 0 {
            return arr1(&[]);
        }
        let max_val = x
            .iter()
            .map(|a| OrderedFloat(*a))
            .max()
            .expect("Error calculating max value")
            .0;
        let min_val = x
            .iter()
            .map(|a| OrderedFloat(*a))
            .min()
            .expect("Error calculating max value")
            .0;
        let result_array = arr1(&[max_val, min_val]);
        result_array
    } 
}