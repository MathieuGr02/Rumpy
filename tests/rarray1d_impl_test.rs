mod test {
    use rstest::rstest;
    use rumpy::linalg::rarray::{Rarray1D, RarrayCreate};

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
    #[case(vec![1., 2., 3.], 3)]
    #[should_panic]
    fn rarray1d_mut_out_of_bounds_indexing(#[case] a: Vec<f64>, #[case] index: usize){
        let mut rarray = Rarray1D::new(&a);
        rarray[index] = 2.;
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

    #[rstest]
    #[case(vec![1., 1., 1.], vec![1., 1., 1.], 3.)]
    fn rarray1d_dot(#[case] a: Vec<f64>, #[case] b: Vec<f64>, #[case] result: f64){
        let rarray_a = Rarray1D::new(&a).transpose();
        let rarray_b = Rarray1D::new(&b);
        let rarray_dot = &rarray_a * &rarray_b;
        println!("Result : {:?}", result);
        assert_eq!(rarray_dot, result);
    }

    #[rstest]
    #[case(vec![1., 1., 1.], vec![1., 1., 1., 1.])]
    #[case(vec![1., 1., 1., 1.], vec![1., 1., 1.])]
    #[should_panic]
    fn rarray1d_dot_shape_mismatch(#[case] a: Vec<f64>, #[case] b: Vec<f64>){
        let rarray_a = Rarray1D::new(&a).transpose();
        let rarray_b = Rarray1D::new(&b);
        let rarray_dot = &rarray_a * &rarray_b;
    }

    #[rstest]
    #[case(vec![1., 1., 1.], "Rarray1D([1, 1, 1])")]
    #[case(vec![],  "Rarray1D([])")]
    #[case(vec![0.5, 2., 1.25, 20.],  "Rarray1D([0.5, 2, 1.25, 20])")]
    fn rarray1d_fmt(#[case] a: Vec<f64>, #[case] result: String){
        let rarray = Rarray1D::new(&a);
        assert_eq!(format!("{}", rarray), result);
    }
}
