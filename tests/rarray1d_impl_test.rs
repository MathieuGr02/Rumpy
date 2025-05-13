mod test {
    use rstest::rstest;
    use rumpy::{linalg::rarray::{Rarray1D, RarrayCreate}, mat};

    #[rstest]
    #[case(mat![1., 2., 3.], 0, 1.)]
    #[case(mat![1., 2., 3.], 1, 2.)]
    #[case(mat![1., 2., 3.], 2, 3.)]
    fn rarray1d_hor_indexing(#[case] m: Rarray1D<f64>, #[case] index: usize, #[case] result: f64){
        assert_eq!(m[index], result);
    }

    #[rstest]
    #[case(mat![1., 2., 3.], 0, 0.)]
    #[case(mat![1., 2., 3.], 1, 0.)]
    #[case(mat![1., 2., 3.], 2, 0.)]
    fn rarray1d_hor_mut_indexing(#[case] mut m: Rarray1D<f64>, #[case] index: usize, #[case] value: f64){
        m[index] = value;
        assert_eq!(m[index], value);
    }

    #[rstest]
    #[case(mat![[1.], [2.], [3.]], 0, 1.)]
    #[case(mat![[1.], [2.], [3.]], 1, 2.)]
    #[case(mat![[1.], [2.], [3.]], 2, 3.)]
    fn rarray1d_ver_indexing(#[case] m: Rarray1D<f64>, #[case] index: usize, #[case] result: f64){
        assert_eq!(m[index], result);
    }

    #[rstest]
    #[case(mat![[1.], [2.], [3.]], 0, 0.)]
    #[case(mat![[1.], [2.], [3.]], 1, 0.)]
    #[case(mat![[1.], [2.], [3.]], 2, 0.)]
    fn rarray1d_ver_mut_indexing(#[case] mut m: Rarray1D<f64>, #[case] index: usize, #[case] value: f64){
        m[index] = value;
        assert_eq!(m[index], value);
    }

    #[rstest]
    #[case(mat![1., 2., 3.], 3)]
    #[should_panic]
    fn rarray1d_hor_out_of_bounds_indexing(#[case] m: Rarray1D<f64>, #[case] index: usize){
        let _ = m[index];
    }

    #[rstest]
    #[case(mat![[1.], [2.], [3.]], 3)]
    #[should_panic]
    fn rarray1d_ver_out_of_bounds_indexing(#[case] m: Rarray1D<f64>, #[case] index: usize){
        let _ = m[index];
    }

    #[rstest]
    #[case(mat![1., 2., 3.], 3)]
    #[should_panic]
    fn rarray1d_hor_mut_out_of_bounds_indexing(#[case] mut m: Rarray1D<f64>, #[case] index: usize){
        m[index] = 2.;
    }

    #[rstest]
    #[case(mat![[1.], [2.], [3.]], 3)]
    #[should_panic]
    fn rarray1d_ver_mut_out_of_bounds_indexing(#[case] mut m: Rarray1D<f64>, #[case] index: usize){
        m[index] = 2.;
    }


    #[rstest]
    #[case(mat![1., 1., 1.], 1., mat![1., 1., 1.])]
    #[case(mat![1., 1., 2.], 0., mat![0., 0., 0.])]
    #[case(mat![1., 2., 3.], 3., mat![3., 6., 9.])]
    fn rarra1d_scalar_mul(#[case] m: Rarray1D<f64>, #[case] scalar: f64, #[case] result: Rarray1D<f64>){
        let rarray_mul_result = &m * scalar;
        for i in 0..(m.get_shape()[0]){
            assert_eq!(rarray_mul_result[i], result[i]); 
        }
    }

    #[rstest]
    #[case(mat![1., 1., 1.], 1., mat![1., 1., 1.])]
    #[case(mat![1., 1., 2.], 0., mat![0., 0., 0.])]
    #[case(mat![1., 2., 3.], 3., mat![3., 6., 9.])]
    fn rarra1d_scalar_mul_assign(#[case] mut m: Rarray1D<f64>, #[case] scalar: f64, #[case] result: Rarray1D<f64>){
        m *= scalar;
        for i in 0..(m.get_shape()[0]){
            assert_eq!(m[i], result[i]); 
        }
    }

    #[rstest]
    #[case(mat![1., 1., 1.], mat![[1.], [1.], [1.]], 3.)]
    fn rarray1d_dot(#[case] a: Rarray1D<f64>, #[case] b: Rarray1D<f64>, #[case] result: f64){
        let rarray_dot = &a * &b;
        println!("Result : {:?}", result);
        assert_eq!(rarray_dot, result);
    }

    #[rstest]
    #[case(mat![1., 1., 1.], mat![1., 1., 1., 1.])]
    #[case(mat![1., 1., 1., 1.], mat![1., 1., 1.])]
    #[should_panic]
    fn rarray1d_dot_shape_mismatch(#[case] a: Rarray1D<f64>, #[case] b: Rarray1D<f64>){
        let _ = &a * &b;
    }

    #[rstest]
    #[case(mat![1., 1., 1.], "Rarray1D([1, 1, 1])")]
    //#[case(mat![],  "Rarray1D([])")]
    #[case(mat![0.5, 2., 1.25, 20.],  "Rarray1D([0.5, 2, 1.25, 20])")]
    fn rarray1d_fmt(#[case] m: Rarray1D<f64>, #[case] result: String){
        assert_eq!(format!("{}", m), result);
    }

    #[rstest]
    #[case(mat![1., 1., 1.], mat![1., 1., 1.])]
    #[case(mat![2.5], mat![2.5])]
    fn rarray1d_eq(#[case] a: Rarray1D<f64>, #[case] b: Rarray1D<f64>) {
        assert_eq!(a, b);
    }

    #[rstest]
    #[case(mat![1., 1., 1.], mat![0., 1., 1.])]
    #[case(mat![1., 1., 1.], mat![1., 0., 1.])]
    #[case(mat![1., 1., 1.], mat![1., 0., 0.])]
    #[case(mat![2.5], mat![0.])]
    #[case(mat![1., 1.], mat![1.])]
    #[case(mat![1.], mat![1., 1.])]
    fn rarray1d_nq(#[case] a: Rarray1D<f64>, #[case] b: Rarray1D<f64>) {
        assert_ne!(a, b);
    }
}
