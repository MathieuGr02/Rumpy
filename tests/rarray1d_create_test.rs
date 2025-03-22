mod test {
    use std::alloc::handle_alloc_error;
    use std::env::var;
    use rstest::rstest;
    use rumpy::linalg::rarray;
    use rumpy::linalg::rarray::{Rarray1D, Rarray2D, RarrayCreate};

    #[rstest]
    #[case(vec![1., 1., 1.])]
    #[case(vec![0.])]
    #[case(vec![2.4, 2., 1., 0.4, 6.,])]
    fn rarray1d_new_hor(#[case] a: Vec<f64>){
        let rarray_a = Rarray1D::new(&a);
        for i in 0..a.len() {
            assert_eq!(a[i], rarray_a[i]);
        }
    }

    #[rstest]
    #[case(vec![vec![1.], vec![1.], vec![1.]])]
    #[case(vec![vec![0.]])]
    #[case(vec![vec![2.4], vec![2.], vec![1.], vec![0.4], vec![6.]])]
    fn rarray1d_new_ver(#[case] a: Vec<Vec<f64>>){
        let rarray_a = Rarray1D::new(&a);
        for i in 0..a[0].len() {
            assert_eq!(a[0][i], rarray_a[i]);
        }
    }

    #[rstest]
    #[case(3)]
    #[case(1)]
    #[case(10)]
    fn rarray1d_zeros_hor(#[case] shape: usize){
        let rarray_zeros = Rarray1D::zeros(shape);
        for i in 0..shape {
            assert_eq!(rarray_zeros[i], 0.);
        }
    }

    #[rstest]
    #[case(3, 1)]
    #[case(1, 1)]
    #[case(10, 1)]
    fn rarray1d_zeros_ver(#[case] height: usize, #[case] width: usize){
        let rarray_zeros = Rarray1D::zeros((height, width));
        for i in 0..height {
            assert_eq!(rarray_zeros[i], 0.);
        }
    }

    #[rstest]
    #[case(3, 5.)]
    #[case(1, 2.)]
    #[case(10, 0.5)]
    fn rarray1d_fill_hor(#[case] shape: usize, #[case] value: f64){
        let rarray_fill = Rarray1D::fill(value, shape);
        for i in 0..shape {
            assert_eq!(rarray_fill[i], value);
        }
    }

    #[rstest]
    #[case(3, 1, 5.)]
    #[case(1, 1, 2.)]
    #[case(10, 1, 0.5)]
    fn rarray1d_fill_ver(#[case] height: usize, #[case] width: usize, #[case] value: f64){
        let rarray_fill = Rarray1D::fill(value, (height, width));
        for i in 0..height {
            assert_eq!(rarray_fill[i], value);
        }
    }
}