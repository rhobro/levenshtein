use wasm_bindgen::prelude::*;
use ndarray::prelude::*;

#[wasm_bindgen]
pub fn measure(a: &str, b: &str) -> f64 {
    let a = a.to_lowercase();
    let b = b.to_lowercase();

    // initialize matrix
    let mut matrix: Array<f64, _> = Array::zeros((a.len()+1, b.len()+1));
    matrix.column_mut(0).assign(&Array::range(0., (a.len() + 1) as f64, 1.));
    matrix.row_mut(0).assign(&Array::range(0., (b.len() + 1) as f64, 1.));

    // loop over letters
    for (i, aa) in (1..a.len()+1).zip(a.chars()) {
        for (j, bb) in (1..b.len()+1).zip(b.chars()) {
            let sub_cost = if aa == bb { 0. } else { 1. };

            matrix[[i, j]] = [
                matrix[[i-1, j]] + 1.,
                matrix[[i, j-1]] + 1.,
                matrix[[i-1, j-1]] + sub_cost,
            ].into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        }
    }

    matrix[[a.len()-1, b.len()-1]]
}