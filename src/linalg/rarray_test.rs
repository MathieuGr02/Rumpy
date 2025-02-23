

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
    fn rarray_addition_test() {
        let matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 3);

        let matrix_result = matrix_1 + matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_result[[i, j]], 0.);
            }
        }

        let matrix_1 = linalg::ones(3);
        let matrix_2 = linalg::ones(3);

        let matrix_result = matrix_1 + matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    assert_eq!(matrix_result[[i, j]], 2.);
                } else {
                    assert_eq!(matrix_result[[i, j]], 0.);
                }
            }
        }

        let mut matrix_1 = linalg::zeros(3, 3);
        let mut matrix_2 = linalg::zeros(3, 3);

        for i in 0..3 {
            for j in 0..3 {
                matrix_1[[i, j]] = 1.;
                matrix_2[[i, j]] = 1.;
            }
        }

        let matrix_result = matrix_1 + matrix_2;
    
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_result[[i, j]], 2.);
            }
        }
    }

    #[test]
    fn rarray_inplace_addition_test() {
        let mut matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 3);

        matrix_1 += matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_1[[i, j]], 0.);
            }
        }

        let mut matrix_1 = linalg::ones(3);
        let matrix_2 = linalg::ones(3);

        matrix_1 += matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    assert_eq!(matrix_1[[i, j]], 2.);
                } else {
                    assert_eq!(matrix_1[[i, j]], 0.);
                }
            }
        }

        let mut matrix_1 = linalg::zeros(3, 3);
        let mut matrix_2 = linalg::zeros(3, 3);

        for i in 0..3 {
            for j in 0..3 {
                matrix_1[[i, j]] = 1.;
                matrix_2[[i, j]] = 1.;
            }
        }

        matrix_1 += matrix_2;
    
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_1[[i, j]], 2.);
            }
        }
    }


    #[test]
    fn rarray_subtraction_test() {
        let matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 3);

        let matrix_result = matrix_1 - matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_result[[i, j]], 0.);
            }
        }

        let matrix_1 = linalg::ones(3);
        let matrix_2 = linalg::ones(3);

        let matrix_result = matrix_1 - matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_result[[i, j]], 0.);
            }
        }

        let mut matrix_1 = linalg::zeros(3, 3);
        let mut matrix_2 = linalg::zeros(3, 3);

        for i in 0..3 {
            for j in 0..3 {
                matrix_1[[i, j]] = 1.;
                matrix_2[[i, j]] = 1.;
            }
        }

        let matrix_result = matrix_1 - matrix_2;
    
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_result[[i, j]], 0.);
            }
        }
    }

    #[test]
    fn rarray_inplace_subtraction_test() {
        let mut matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 3);

        matrix_1 -= matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_1[[i, j]], 0.);
            }
        }

        let mut matrix_1 = linalg::ones(3);
        let matrix_2 = linalg::ones(3);

        matrix_1 -= matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_1[[i, j]], 0.);
            }
        }

        let mut matrix_1 = linalg::zeros(3, 3);
        let mut matrix_2 = linalg::zeros(3, 3);

        for i in 0..3 {
            for j in 0..3 {
                matrix_1[[i, j]] = 1.;
                matrix_2[[i, j]] = 1.;
            }
        }

        matrix_1 -= matrix_2;
    
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_1[[i, j]], 0.);
            }
        }
    }
}
