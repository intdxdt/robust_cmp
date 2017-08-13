extern crate robust_subtract;
//Robust Compare
pub fn compare(a: &[f64], b: &[f64]) -> f64 {
    *(robust_subtract::robust_subtract(a, b).last().unwrap())
}


#[cfg(test)]
mod test_robust_compare {
    use super::compare as cmp;

    #[test]
    fn test_robust_cmp() {
        assert!(cmp(&vec!(5.), &vec!(1., 4.)) == 0.);
        assert!(cmp(&vec!(1e64), &vec!(-1e-100, 1e64)) > 0.);
        assert!(cmp(&vec!(1e64), &vec!(1e-100, 1e64)) < 0.);
    }
}