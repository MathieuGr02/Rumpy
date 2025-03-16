mod test{
    use rstest::rstest;
    use rumpy::linalg::rarray::Rarray2D;

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
}
