#[cfg(test)]
mod test {
    use rstest::rstest;
    use rumpy::linalg::rarray::{self, Rarray1D};
    
    #[rstest]
    #[case(&[1.], &[1.], 1.)]
    #[case(&[2.], &[4.], 8.)]
    #[case(&[0., 0., 0.], &[0., 0., 0.], 0.)]
    #[case(&[1., 1., 1.], &[1., 1., 1.], 3.)]
    #[case(&[0., 2., 1.5], &[1., 0., 2.5], 3.75)]
    fn rarray1d_dot(#[case] a: &[f64], #[case] b: &[f64], #[case] result: f64) {
        let rarray_a = Rarray1D::new(a);
        let rarray_b = Rarray1D::new(b);
        let dot_result = Rarray1D::dot(&rarray_a, &rarray_b);
        assert_eq!(dot_result, result);
    }

    #[rstest]
    #[case(&[1., 1.], &[1.])]
    #[case(&[1.], &[1., 1.])]
    #[case(&[], &[1.])]
    #[case(&[1.], &[])]
    #[should_panic]
    fn rarray1d_dot_wrong_size(#[case] a: &[f64], #[case] b: &[f64]){
        let rarray_a = Rarray1D::new(a);
        let rarray_b = Rarray1D::new(b);
        let _ = Rarray1D::dot(&rarray_a, &rarray_b);
    }
}
