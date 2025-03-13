#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::linalg::rarray::Rarray1D;
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
    
    #[rstest]
    #[case(&[1.], &[1.], &[2.])]
    #[case(&[1., 1., 1.], &[1., 1., 1.], &[2., 2., 2.])]
    #[case(&[0.], &[0.], &[0.])]
    #[case(&[0., 2., 3.5, 0.75, 1.], &[1., 0.25, 8., 0., 1.25], &[1., 2.25, 11.5, 0.75, 2.25])]
    fn rarray1d_inplace_addition_test(#[case] m: &[f64], #[case] n: &[f64], #[case] result: &[f64]){
        let mut rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        rarray_m += &rarray_n;
        for i in 0..(rarray_m.data.len()) {
            assert_eq!(rarray_m.data[i], result[i]);
        }
    }

    #[rstest]
    #[case(&[1.], &[1., 1.])]
    #[case(&[1., 1.], &[1.])]
    #[should_panic]
    fn rarray1d_inplace_addition_incompatible_test(#[case] m: &[f64], #[case] n: &[f64]){
        let mut rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        rarray_m += &rarray_n;
    }

    #[rstest]
    #[case(&[1.], &[1.], &[0.])]
    #[case(&[1., 1., 1.], &[1., 1., 1.], &[0., 0., 0.])]
    #[case(&[0.], &[0.], &[0.])]
    #[case(&[0., 2., 3.5, 0.75, 1.], &[1., 0.25, 8., 0., 1.25], &[-1., 1.75, -4.5, 0.75, -0.25])]
    fn rarray1d_subtraction_test(#[case] m: &[f64], #[case] n: &[f64], #[case] result: &[f64]){
        let rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        let rarray_result = &rarray_m - &rarray_n;
        for i in 0..(rarray_m.data.len()) {
            assert_eq!(rarray_result.data[i], result[i]);
        }
    }

    #[rstest]
    #[case(&[1.], &[1., 1.])]
    #[case(&[1., 1.], &[1.])]
    #[should_panic]
    fn rarray1d_subtraction_incompatible_test(#[case] m: &[f64], #[case] n: &[f64]){
        let rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        let _ = &rarray_m - &rarray_n;
    }

    #[rstest]
    #[case(&[1.], &[1.], &[0.])]
    #[case(&[1., 1., 1.], &[1., 1., 1.], &[0., 0., 0.])]
    #[case(&[0.], &[0.], &[0.])]
    #[case(&[0., 2., 3.5, 0.75, 1.], &[1., 0.25, 8., 0., 1.25], &[-1., 1.75, -4.5, 0.75, -0.25])]
    fn rarray1d_inplace_subtraction_test(#[case] m: &[f64], #[case] n: &[f64], #[case] result: &[f64]){
        let mut rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        rarray_m -= &rarray_n;
        for i in 0..(rarray_m.data.len()) {
            assert_eq!(rarray_m.data[i], result[i]);
        }
    }

    #[rstest]
    #[case(&[1.], &[1., 1.])]
    #[case(&[1., 1.], &[1.])]
    #[should_panic]
    fn rarray1d_inplace_subtraction_incompatible_test(#[case] m: &[f64], #[case] n: &[f64]){
        let mut rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        rarray_m -= &rarray_n;
    }
}

