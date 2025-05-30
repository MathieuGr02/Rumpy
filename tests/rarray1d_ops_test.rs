#[cfg(test)]
mod test {
    use rstest::rstest;
    use rumpy::linalg::rarray::{self, Rarray, Rarray1D, Rarray2D, RarrayMul};
    use rumpy::linalg::rarray::RarrayCreate;
    use rumpy::mat;
    
    #[rstest]
    #[case(mat![1.], mat![1.], 1.)]
    #[case(mat![2.], mat![4.], 8.)]
    #[case(mat![0., 0., 0.], mat![0., 0., 0.], 0.)]
    #[case(mat![1., 1., 1.], mat![1., 1., 1.], 3.)]
    #[case(mat![0., 2., 1.5], mat![1., 0., 2.5], 3.75)]
    fn rarray1d_dot(#[case] a: Rarray1D<f64>, #[case] b: Rarray1D<f64>, #[case] result: f64) {
        let dot_result = Rarray1D::dot(&a, &b);
        assert_eq!(dot_result, result);
    }

    #[rstest]
    #[case(mat![1.], mat![1.], 1.)]
    #[case(mat![2.], mat![4.], 8.)]
    #[case(mat![0., 0., 0.], mat![0., 0., 0.], 0.)]
    #[case(mat![1., 1., 1.], mat![1., 1., 1.], 3.)]
    #[case(mat![0., 2., 1.5], mat![1., 0., 2.5], 3.75)]
    fn rarray1d_dot_mul(#[case] a: Rarray1D<f64>, #[case] b: Rarray1D<f64>, #[case] result: f64) {
        let dot_result = Rarray1D::dot(&a, &b);
        assert_eq!(dot_result, result);
    }

    #[rstest]
    #[case(mat![0., 0., 0.], mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]], mat![0., 0., 0.])]
    #[case(mat![1., 1., 1.], mat![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]], mat![3., 3., 3.])]
    #[case(mat![1., 1., 1.], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![1., 1., 1.])]
    fn rarray1d_mul_rarray2d(#[case] a: Rarray1D<f64>, #[case] b: Rarray2D<f64>, #[case] result: Rarray1D<f64>) {
        let mul_result = Rarray::mul(&a, &b);
        assert_eq!(mul_result, result);
    }

    #[rstest]
    #[case(mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]], mat![0., 0., 0.], mat![0., 0., 0.])]
    #[case(mat![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]], mat![1., 1., 1.], mat![3., 3., 3.])]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![1., 1., 1.], mat![1., 1., 1.])]
    fn rarray2d_mul_rarray1d(#[case] a: Rarray2D<f64>, #[case] b: Rarray1D<f64>, #[case] result: Rarray1D<f64>) {
        let mul_result = Rarray::mul(&b, &a);
        assert_eq!(mul_result, result);
    }

    #[rstest]
    #[case(vec![1., 1.], vec![1.])]
    #[case(vec![1.], vec![1., 1.])]
    #[case(vec![], vec![1.])]
    #[case(vec![1.], vec![])]
    #[should_panic]
    fn rarray1d_dot_wrong_size(#[case] a: Vec<f64>, #[case] b: Vec<f64>){
        let rarray_a = Rarray1D::new(&a);
        let rarray_b = Rarray1D::new(&b);
        let _ = Rarray1D::dot(&rarray_a, &rarray_b);
    }


    #[rstest]
    #[case(vec![1.], 1, 1)]
    #[case(vec![1., 2., 3., 4.], 4, 1)]
    fn rarray1d_transpose_shape(#[case] a: Vec<f64>, #[case] height: usize, #[case] width: usize) {
        let rarray = Rarray1D::new(&a);
        let rarray_t = rarray.transpose();
        assert_eq!(rarray_t.get_shape()[0], height);
        assert_eq!(rarray_t.get_shape()[1], width);
    }

    #[rstest]
    #[case(vec![1.])]
    #[case(vec![1., 2., 3., 4.])]
    fn rarray1d_transpose_index(#[case] a: Vec<f64>){
        let rarray = Rarray1D::new(&a);
        let rarray_t = rarray.transpose();
        for i in 0..rarray_t.get_shape()[1] {
            assert_eq!(rarray_t[i], a[i]);
        }
    }

    #[rstest]
    #[case(vec![1.], 1, 1)]
    #[case(vec![1., 2., 3., 4.], 4, 1)]
    fn rarray1d_mut_transpose_shape(#[case] a: Vec<f64>, #[case] height: usize, #[case] width: usize) {
        let mut rarray = Rarray1D::new(&a);
        rarray.mut_transpose();
        assert_eq!(rarray.get_shape()[0], height);
        assert_eq!(rarray.get_shape()[1], width);
    }

    #[rstest]
    #[case(vec![1.])]
    #[case(vec![1., 2., 3., 4.])]
    fn rarray1d_mut_transpose_index(#[case] a: Vec<f64>){
        let mut rarray = Rarray1D::new(&a);
        rarray.mut_transpose();
        for i in 0..rarray.get_shape()[1] {
            assert_eq!(rarray[i], a[i]);
        }
    }

    #[rstest]
    #[case(mat![1., 1., 1.], mat![1., 1., 1.], mat![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]])]
    #[case(mat![1., 1., 1.], mat![1., 1.], mat![[1., 1.], [1., 1.], [1., 1.]])]
    #[case(mat![1., 1.], mat![1., 1., 1.], mat![[1., 1., 1.], [1., 1., 1.]])]
    fn rarray1d_outer(#[case] a: Rarray1D<f64>, #[case] b: Rarray1D<f64>, #[case] result: Rarray2D<f64>) {
        let outer = Rarray1D::outer(&a, &b);
        for i in 0..(a.get_shape()[0]) {
            for j in 0..(b.get_shape()[1]) {
                assert_eq!(outer[[i, j]], result[[i, j]]);
            }
        }
    }
}
