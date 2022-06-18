use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn measure(a: &str, b: &str) -> f64 {
    let a = a.to_lowercase();
    let b = b.to_lowercase();

    // initialize matrix
    let mut matrix = gen_matrix(a.len(), b.len());

    // loop over letters
    for (i, aa) in (1..a.len()+1).zip(a.chars()) {
        for (j, bb) in (1..b.len()+1).zip(b.chars()) {
            let sub_cost = if aa == bb { 0. } else { 1. };

            matrix[i][j] = [
                matrix[i-1][j] + 1.,
                matrix[i][j-1] + 1.,
                matrix[i-1][j-1] + sub_cost,
            ].into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        }
    }

    matrix[a.len()-1][b.len()-1]
}

fn gen_matrix(a: usize, b: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![];
    for i in 0..a+1 {
        matrix.push({
            let mut row = vec![];
            for i in 0..b+1 {
                row.push(0.);
            }
            row
        });
    }
    
    for i in 1..a+1 {
        matrix[i][0] = (i) as f64;
    }
    for i in 1..b+1 {
        matrix[0][i] = (i) as f64;
    }

    matrix
}