#[cfg(test)]
mod test {
    use rstest::rstest;
    use rumpy::linalg::rarray::Rarray1D;

    #[rstest]
    #[case(vec![1.], vec![1.], vec![2.])]
    #[case(vec![1., 1., 1.], vec![1., 1., 1.], vec![2., 2., 2.])]
    #[case(vec![0.], vec![0.], vec![0.])]
    #[case(vec![0., 2., 3.5, 0.75, 1.], vec![1., 0.25, 8., 0., 1.25], vec![1., 2.25, 11.5, 0.75, 2.25])]
    fn rarray1d_addition_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>, #[case] result: Vec<f64>){
        let rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        let rarray_result = &rarray_m + &rarray_n;
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_result[i], result[i]);
        }
    }

    #[rstest]
    #[case(vec![1.], vec![1., 1.])]
    #[case(vec![1., 1.], vec![1.])]
    #[should_panic]
    fn rarray1d_addition_incompatible_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>){
        let rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        let _ = &rarray_m + &rarray_n;
    }

    #[rstest]
    #[case(vec![1.], vec![1.], vec![2.])]
    #[case(vec![1., 1., 1.], vec![1., 1., 1.], vec![2., 2., 2.])]
    #[case(vec![0.], vec![0.], vec![0.])]
    #[case(vec![0., 2., 3.5, 0.75, 1.], vec![1., 0.25, 8., 0., 1.25], vec![1., 2.25, 11.5, 0.75, 2.25])]
    fn rarray1d_inplace_addition_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>, #[case] result: Vec<f64>){
        let mut rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        rarray_m += &rarray_n;
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_m[i], result[i]);
        }
    }

    #[rstest]
    #[case(vec![1.], vec![1., 1.])]
    #[case(vec![1., 1.], vec![1.])]
    #[should_panic]
    fn rarray1d_inplace_addition_incompatible_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>){
        let mut rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        rarray_m += &rarray_n;
    }

    #[rstest]
    #[case(vec![1.], vec![1.], vec![0.])]
    #[case(vec![1., 1., 1.], vec![1., 1., 1.], vec![0., 0., 0.])]
    #[case(vec![0.], vec![0.], vec![0.])]
    #[case(vec![0., 2., 3.5, 0.75, 1.], vec![1., 0.25, 8., 0., 1.25], vec![-1., 1.75, -4.5, 0.75, -0.25])]
    fn rarray1d_subtraction_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>, #[case] result: Vec<f64>){
        let rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        let rarray_result = &rarray_m - &rarray_n;
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_result[i], result[i]);
        }
    }

    #[rstest]
    #[case(vec![1.], vec![1., 1.])]
    #[case(vec![1., 1.], vec![1.])]
    #[should_panic]
    fn rarray1d_subtraction_incompatible_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>){
        let rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        let _ = &rarray_m - &rarray_n;
    }

    #[rstest]
    #[case(vec![1.], vec![1.], vec![0.])]
    #[case(vec![1., 1., 1.], vec![1., 1., 1.], vec![0., 0., 0.])]
    #[case(vec![0.], vec![0.], vec![0.])]
    #[case(vec![0., 2., 3.5, 0.75, 1.], vec![1., 0.25, 8., 0., 1.25], vec![-1., 1.75, -4.5, 0.75, -0.25])]
    fn rarray1d_inplace_subtraction_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>, #[case] result: Vec<f64>){
        let mut rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        rarray_m -= &rarray_n;
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_m[i], result[i]);
        }
    }

    #[rstest]
    #[case(vec![1.], vec![1., 1.])]
    #[case(vec![1., 1.], vec![1.])]
    #[should_panic]
    fn rarray1d_inplace_subtraction_incompatible_test(#[case] m: Vec<f64>, #[case] n: Vec<f64>){
        let mut rarray_m = Rarray1D::new(&m);
        let rarray_n = Rarray1D::new(&n);
        rarray_m -= &rarray_n;
    }
}
