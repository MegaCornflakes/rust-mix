use wasm_bindgen::prelude::*;

mod tables;

#[wasm_bindgen]
pub fn number() -> f64 {
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let c = dot(&a, &b);
    c[0][0]
}

fn dot(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut c = vec![vec![0.0; b[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..b.len() {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let c = dot(&a, &b);
        assert_eq!(c, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
    }
}
