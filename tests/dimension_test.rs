mod test {
    use rstest::rstest;
    use rumpy::linalg::dimension::{D1, D2, D3};

    #[rstest]
    #[case(1, 1)]
    #[case(1, 5)]
    #[case(5, 1)]
    fn dimension1_new(#[case] height: usize, #[case] width: usize) {
        let dim = D1::new(height, width);
        assert_eq!(dim.height, height);
        assert_eq!(dim.width, width);
    }

    #[rstest]
    #[case(2, 5)]
    #[case(5, 2)]
    #[should_panic]
    fn dimension1_new_invalid_shape(#[case] height: usize, #[case] width: usize) {
        let _ = D1::new(height, width);
    }

    #[rstest]
    fn dimension1_index() {
        let dim = D1::new(1, 5);
        assert_eq!(dim[0], 1);
        assert_eq!(dim[1], 5);
    }

    #[rstest]
    #[should_panic]
    fn dimension1_index_invalid() {
        let dim = D1::new(1, 5);
        dim[2];
    }

    #[rstest]
    #[case(D1::new(1, 5), D1::new(1, 5))]
    #[case(D1::new(5, 1), D1::new(5, 1))]
    fn dimension1_eq(#[case] a: D1, #[case] b: D1) {
        assert_eq!(a, b);
    }

    #[rstest]
    #[case(D1::new(1, 5), D1::new(5, 1))]
    #[case(D1::new(5, 1), D1::new(1, 5))]
    fn dimension1_ne(#[case] a: D1, #[case] b: D1) {
        assert_ne!(a, b);
    }

    #[rstest]
    #[case(1, 1)]
    #[case(5, 5)]
    fn dimension2_new(#[case] height: usize, #[case] width: usize) {
        let dim = D2::new(height, width);
        assert_eq!(dim.height, height);
        assert_eq!(dim.width, width);
    }

    #[rstest]
    fn dimension2_index() {
        let dim = D2::new(5, 5);
        assert_eq!(dim[0], 5);
        assert_eq!(dim[1], 5);
    }

    #[rstest]
    #[should_panic]
    fn dimension2_index_invalid() {
        let dim = D2::new(1, 5);
        dim[2];
    }
}
