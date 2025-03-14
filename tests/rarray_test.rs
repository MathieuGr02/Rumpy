#[cfg(test)]
mod test {
    use rstest::rstest;
    use rumpy::linalg::rarray::Rarray1D;

    #[rstest]
    #[case(&[1.], &[1.], &[2.])]
    #[case(&[1., 1., 1.], &[1., 1., 1.], &[2., 2., 2.])]
    #[case(&[0.], &[0.], &[0.])]
    #[case(&[0., 2., 3.5, 0.75, 1.], &[1., 0.25, 8., 0., 1.25], &[1., 2.25, 11.5, 0.75, 2.25])]
    fn rarray1d_addition_test(#[case] m: &[f64], #[case] n: &[f64], #[case] result: &[f64]){
        let rarray_m = Rarray1D::new(m);
        let rarray_n = Rarray1D::new(n);
        let rarray_result = &rarray_m + &rarray_n;
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_result[i], result[i]);
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
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_m[i], result[i]);
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
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_result[i], result[i]);
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
        for i in 0..(rarray_m.get_shape()[0]) {
            assert_eq!(rarray_m[i], result[i]);
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

    #[rstest]
    #[case(&[1., 2., 3.], 0, 1.)]
    #[case(&[1., 2., 3.], 1, 2.)]
    #[case(&[1., 2., 3.], 2, 3.)]
    fn rarray1d_indexing(#[case] a: &[f64], #[case] index: usize, #[case] result: f64){
        let rarray = Rarray1D::new(a);
        assert_eq!(rarray[index], result);
    }

    #[rstest]
    #[case(&[1., 2., 3.], 0, 0.)]
    #[case(&[1., 2., 3.], 1, 0.)]
    #[case(&[1., 2., 3.], 2, 0.)]
    fn rarray1d_mut_indexing(#[case] a: &[f64], #[case] index: usize, #[case] value: f64){
        let mut rarray = Rarray1D::new(a);
        rarray[index] = value;
        assert_eq!(rarray[index], value);
    }

    #[rstest]
    #[case(&[1., 2., 3.], 3)]
    #[should_panic]
    fn rarray1d_out_of_bounds_indexing(#[case] a: &[f64], #[case] index: usize){
        let rarray = Rarray1D::new(a);
        let _ = rarray[index];
    }

    #[rstest]
    #[case(&[1., 1., 1.], 1., &[1., 1., 1.])]
    #[case(&[1., 1., 2.], 0., &[0., 0., 0.])]
    #[case(&[1., 2., 3.], 3., &[3., 6., 9.])]
    fn rarra1d_scalar_mul(#[case] a: &[f64], #[case] scalar: f64, #[case] result: &[f64]){
        let rarray = Rarray1D::new(a);
        let rarray_mul_result = &rarray * scalar;
        for i in 0..(rarray.get_shape()[0]){
            assert_eq!(rarray_mul_result[i], result[i]); 
        }
    }

    #[rstest]
    #[case(&[1., 1., 1.], 1., &[1., 1., 1.])]
    #[case(&[1., 1., 2.], 0., &[0., 0., 0.])]
    #[case(&[1., 2., 3.], 3., &[3., 6., 9.])]
    fn rarra1d_scalar_mul_assign(#[case] a: &[f64], #[case] scalar: f64, #[case] result: &[f64]){
        let mut rarray = Rarray1D::new(a);
        rarray *= scalar;
        for i in 0..(rarray.get_shape()[0]){
            assert_eq!(rarray[i], result[i]); 
        }
    }
}
