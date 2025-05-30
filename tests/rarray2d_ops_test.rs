mod test{
    use rstest::rstest;
    use rumpy::linalg::rarray::{Rarray, Rarray1D, Rarray2D, RarrayMul};
    use rumpy::linalg::rarray::RarrayCreate;
    use rumpy::mat;

    #[rstest]
    #[case(vec![vec![1., 1.], vec![1., 1.,]])]
    #[case(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]])]
    fn rarray2d_new(#[case] a: Vec<Vec<f64>>) {
        let rarray = Rarray2D::new(&a);
        let shape = rarray.get_shape();
        for i in 0..shape[0] {
            for j in 0..shape[1] {
                assert_eq!(rarray[[i, j]], a[i][j]);
            }
        }
    }

    #[rstest]
    #[case(vec![vec![], vec![1.], vec![1.]])]
    #[case(vec![vec![1.], vec![], vec![1.]])]
    #[case(vec![vec![1.], vec![1.], vec![]])]
    #[should_panic]
    fn rarray2d_mismatching_row_sizes(#[case] a: Vec<Vec<f64>>){
        let _ = Rarray2D::new(&a);
    }

    #[rstest]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]], mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]])]
    #[case(mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]])]
    #[case(mat![[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]], mat![[1., 4., 7.], [2., 5., 8.], [3., 6., 9.]], mat![[14., 32., 50.], [32., 77., 122.], [50., 122., 194.]])]
    fn rarray2d_mul_rarray2d(#[case] a: Rarray2D<f64>, #[case] b: Rarray2D<f64>, #[case] result: Rarray2D<f64>) {
        let mul_result = Rarray::mul(&a, &b);
        assert_eq!(mul_result, result);
    }

    #[rstest]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]], mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]])]
    #[case(mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]])]
    #[case(mat![[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]], mat![[1., 4., 7.], [2., 5., 8.], [3., 6., 9.]], mat![[14., 32., 50.], [32., 77., 122.], [50., 122., 194.]])]
    fn rarray2d_mul_rarray2d_op(#[case] a: Rarray2D<f64>, #[case] b: Rarray2D<f64>, #[case] result: Rarray2D<f64>) {
        let mul_result = &a * &b;
        assert_eq!(mul_result, result);
    }

    #[rstest]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0., 0., 0.]])]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.]])]
    #[case(mat![[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.]], mat![[1., 0., 0.], [0., 1., 0.]])]
    #[case(mat![[1., 0.], [0., 1.], [0., 0.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])]
    #[should_panic]
    fn rarray2d_mul_rarray2d_op_incompatible(#[case] a: Rarray2D<f64>, #[case] b: Rarray2D<f64>) {
        let mul_result = &a * &b;
    }
}
