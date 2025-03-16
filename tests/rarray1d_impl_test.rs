mod test {
    use rstest::rstest;
    use rumpy::linalg::rarray::Rarray1D;

    #[rstest]
    #[case(vec![1., 2., 3.], 0, 1.)]
    #[case(vec![1., 2., 3.], 1, 2.)]
    #[case(vec![1., 2., 3.], 2, 3.)]
    fn rarray1d_indexing(#[case] a: Vec<f64>, #[case] index: usize, #[case] result: f64){
        let rarray = Rarray1D::new(&a);
        assert_eq!(rarray[index], result);
    }

    #[rstest]
    #[case(vec![1., 2., 3.], 0, 0.)]
    #[case(vec![1., 2., 3.], 1, 0.)]
    #[case(vec![1., 2., 3.], 2, 0.)]
    fn rarray1d_mut_indexing(#[case] a: Vec<f64>, #[case] index: usize, #[case] value: f64){
        let mut rarray = Rarray1D::new(&a);
        rarray[index] = value;
        assert_eq!(rarray[index], value);
    }

    #[rstest]
    #[case(vec![1., 2., 3.], 3)]
    #[should_panic]
    fn rarray1d_out_of_bounds_indexing(#[case] a: Vec<f64>, #[case] index: usize){
        let rarray = Rarray1D::new(&a);
        let _ = rarray[index];
    }

    #[rstest]
    #[case(vec![1., 1., 1.], 1., vec![1., 1., 1.])]
    #[case(vec![1., 1., 2.], 0., vec![0., 0., 0.])]
    #[case(vec![1., 2., 3.], 3., vec![3., 6., 9.])]
    fn rarra1d_scalar_mul(#[case] a: Vec<f64>, #[case] scalar: f64, #[case] result: Vec<f64>){
        let rarray = Rarray1D::new(&a);
        let rarray_mul_result = &rarray * scalar;
        for i in 0..(rarray.get_shape()[0]){
            assert_eq!(rarray_mul_result[i], result[i]); 
        }
    }

    #[rstest]
    #[case(vec![1., 1., 1.], 1., vec![1., 1., 1.])]
    #[case(vec![1., 1., 2.], 0., vec![0., 0., 0.])]
    #[case(vec![1., 2., 3.], 3., vec![3., 6., 9.])]
    fn rarra1d_scalar_mul_assign(#[case] a: Vec<f64>, #[case] scalar: f64, #[case] result: Vec<f64>){
        let mut rarray = Rarray1D::new(&a);
        rarray *= scalar;
        for i in 0..(rarray.get_shape()[0]){
            assert_eq!(rarray[i], result[i]); 
        }
    }
}
