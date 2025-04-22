mod tests {
    use rstest::rstest;
    use rumpy::linalg::rarray::{Rarray1D, Rarray2D, RarrayCreate};
    use rumpy::mat;

    #[rstest]
    #[case(mat![0., 0., 0.], vec![0., 0., 0.])]
    #[case(mat![1.], vec![1.])]
    #[case(mat![0., 1., 2., 3., 4., 5.], vec![0., 1., 2., 3., 4., 5.])]
    fn rarray1d_ver_macro(#[case] one: Rarray1D<f64>, #[case] other: Vec<f64>) {
        let other = Rarray1D::new(&other);
        assert_eq!(one, other);
    }

    #[rstest]
    #[case(mat![[0.], [0.], [0.]], vec![vec![0.], vec![0.], vec![0.]])]
    #[case(mat![1.], vec![vec![1.]])]
    #[case(mat![[0.], [1.], [2.], [3.], [4.], [5.]], vec![vec![0.], vec![1.], vec![2.], vec![3.], vec![4.], vec![5.]])]
    fn rarray1d_hor_macro(#[case] one: Rarray1D<f64>, #[case] other: Vec<Vec<f64>>) {
        let other = Rarray1D::new(&other);
        assert_eq!(one, other);
    }
}
