

#[cfg(test)]
mod tests {
    use std::any::Any;
    use crate::linalg;

    #[test]
    fn rarray_print_test() {
        let matrix = linalg::zeros(3, 3);
        assert_eq!("[[0, 0, 0],\n[0, 0, 0],\n[0, 0, 0]]", format!("{matrix}"));
    }

    #[test]
    fn rarray_indexing() {
        let matrix = linalg::zeros(3, 3);
        assert_eq!(matrix[[0, 0]], 0.);
    }

    #[test]
    fn rarray_mut_indexing() {
        let mut matrix = linalg::zeros(3, 3);
        assert_eq!(matrix[[0, 0]], 0.);

        matrix[[0, 0]] = 1.;

        assert_eq!(matrix[[0, 0]], 1.);
    }
    
    #[test]
    #[should_panic]
    fn rarray_row_outofbounds_indexing() {
        let matrix = linalg::zeros(3, 3);
        let _ = matrix[[3, 0]];
    }

    #[test]
    #[should_panic]
    fn rarray_row_outofbounds_mut_indexing() {
        let mut matrix = linalg::zeros(3, 3);
        let _ = matrix[[3, 0]];
    }

    #[test]
    #[should_panic]
    fn array_column_outofbounds_indexing() {
        let matrix = linalg::zeros(3, 3);
        let _ = matrix[[0, 3]];
    }

    #[test]
    #[should_panic]
    fn array_column_outofbounds_mut_indexing() {
        let mut matrix = linalg::zeros(3, 3);
        let _ = matrix[[0, 3]];
    }
    
    #[test]
    fn create_zeros_rarray() {
        let matrix = linalg::zeros(3, 3);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix[[i, j]], 0.);
            }
        }
    }
}
