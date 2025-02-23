
#[cfg(test)]
mod tests {
    use crate::linalg;

    #[test]
    fn rarray_zeros_creation_test() {
        let matrix = linalg::zeros(3, 3);
        
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix[[i, j]], 0.);
            } 
        }
    }

    #[test]
    fn rarray_ones_creation_test() {
        let matrix = linalg::ones(3);
        println!("{}", matrix);
        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    assert_eq!(matrix[[i, j]], 0.);
                } else {
                    assert_eq!(matrix[[i, j]], 1.);
                }
            }
        }
    }
}
