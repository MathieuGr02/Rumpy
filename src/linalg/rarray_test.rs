use rstest::rstest;



#[cfg(test)]
mod tests {
    use rand::seq::IndexedRandom;

    use crate::linalg::{self, rarray::{Rarray, Rarray1D}};

    use crate::linalg::rarray_test::rstest;
    /*
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
        matrix[[0, 2]] = 1.;
        matrix[[2, 0]] = 1.;
        matrix[[2, 2]] = 1.;

        assert_eq!(matrix[[0, 0]], 1.);
        assert_eq!(matrix[[2, 0]], 1.);
        assert_eq!(matrix[[0, 2]], 1.);
        assert_eq!(matrix[[2, 2]], 1.);
        assert_eq!(matrix[[1, 1]], 0.);
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
    #[should_panic]
    fn rarray_incorrect_column_size_addition_test() {
        let matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 4);

        let _ = matrix_1 + matrix_2;
    }
    
    #[test]
    #[should_panic]
    fn rarray_incorrect_row_size_addition_test() {
        let matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(4, 3);

        let _ = matrix_1 + matrix_2;
    }
    
    #[test]
    #[should_panic]
    fn rarray_incorrect_column_size_subtraction_test() {
        let matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 4);

        let _ = matrix_1 - matrix_2;
    }
    
    #[test]
    #[should_panic]
    fn rarray_incorrect_row_size_subtraction_test() {
        let matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(4, 3);

        let _ = matrix_1 - matrix_2;
    }

    #[test]
    #[should_panic]
    fn rarray_incorrect_column_size_inplace_addition_test() {
        let mut matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 4);

        matrix_1 += matrix_2;
    }
    
    #[test]
    #[should_panic]
    fn rarray_incorrect_row_size_inplace_addition_test() {
        let mut matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(4, 3);

        matrix_1 += matrix_2;
    }
    
    #[test]
    #[should_panic]
    fn rarray_incorrect_column_size_inplace_subtraction_test() {
        let mut matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 4);

        matrix_1 -= matrix_2;
    }
    
    #[test]
    #[should_panic]
    fn rarray_incorrect_row_size_inplace_subtraction_test() {
        let mut matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(4, 3);

        matrix_1 -= matrix_2;
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

    #[test]
    fn rarray_multiplication_test() {
        let matrix_1 = linalg::zeros(3, 3);
        let matrix_2 = linalg::zeros(3, 3);

        let matrix_result = matrix_1 * matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix_result[[i, j]], 0.);
            }
        }

        let matrix_1 = linalg::ones(3);
        let matrix_2 = linalg::ones(3);

        let matrix_result = matrix_1 * matrix_2;

        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    assert_eq!(matrix_result[[i, j]], 1.);
                } else {
                    assert_eq!(matrix_result[[i, j]], 0.);
                }
            }
        }
    }
    */

    #[rstest]
    #[case(&[1.], &[1.], &[2.])]
    #[case(&[1., 1., 1.], &[1., 1., 1.], &[2., 2., 2.])]
    #[case(&[0.], &[0.], &[0.])]
    #[case(&[0., 2., 3.5, 0.75, 1.], &[1., 0.25, 8., 0., 1.25], &[1., 2.25, 11.5, 0.75, 2.25])]
    fn rarray1d_addition_test(#[case] m: &[f64], #[case] n: &[f64], #[case] result: &[f64]){
        let rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        let rarray_result = &rarray_m + &rarray_n;
        for i in 0..(rarray_m.data.len()) {
            assert_eq!(rarray_result.data[i], result[i]);
        }
    }

    #[rstest]
    #[case(&[1.], &[1., 1.])]
    #[case(&[1., 1.], &[1.])]
    #[should_panic]
    fn rarray1d_addition_incompatible_test(#[case] m: &[f64], #[case] n: &[f64]){
        let rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        let _ = &rarray_m + &rarray_n;
    }
}

