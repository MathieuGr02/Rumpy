mod test {
    use rstest::rstest;
    use rumpy::linalg::rarray::{Rarray1D, RarrayCreate};

    #[rstest]
    #[case(vec![1., 1., 1.])]
    #[case(vec![0.])]
    #[case(vec![2.4, 2., 1., 0.4, 6.,])]
    fn rarray1d_new_hor(#[case] a: Vec<f64>){
        let rarray_a = Rarray1D::<f64>::new(&a);
        for i in 0..a.len() {
            assert_eq!(a[i], rarray_a[i]);
        }
    }

    #[rstest]
    #[case(vec![vec![1.], vec![1.], vec![1.]])]
    #[case(vec![vec![0.]])]
    #[case(vec![vec![2.4], vec![2.], vec![1.], vec![0.4], vec![6.]])]
    fn rarray1d_new_ver(#[case] a: Vec<Vec<f64>>){
        let rarray_a = Rarray1D::<f64>::new(&a);
        for i in 0..a[0].len() {
            assert_eq!(a[0][i], rarray_a[i]);
        }
    }

    #[rstest]
    #[case(3)]
    #[case(1)]
    #[case(10)]
    fn rarray1d_zeros_hor(#[case] shape: usize){
        let rarray_zeros = Rarray1D::<f64>::zeros(shape);
        for i in 0..shape {
            assert_eq!(rarray_zeros[i], 0.);
        }
    }

    #[rstest]
    #[case(3, 1)]
    #[case(1, 1)]
    #[case(10, 1)]
    fn rarray1d_zeros_ver(#[case] height: usize, #[case] width: usize){
        let rarray_zeros = Rarray1D::<f64>::zeros((height, width));
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

    #[rstest]
    #[case(vec![1., 1., 1.])]
    #[case(vec![0., 0., 0.])]
    #[case(vec![1., 0.25, 3.])]
    fn rarray1d_diag(#[case] a: Vec<f64>) {
        let vec = Rarray1D::<f64>::new(&a); 
        let m = Rarray1D::diag(&vec);
        let shape = m.get_shape();
        assert_eq!(shape.width, a.len());
        assert_eq!(shape.height, a.len());

        for i in 0..shape.width {
            for j in 0..shape.height {
                if i == j {
                    assert_eq!(m[[i, j]], a[i]);
                }
                else {
                    assert_eq!(m[[i, j]], 0.);
                }
            }
        }
    }

    #[rstest]
    #[case(0, 10, 1, vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9.])]
    #[case(0, 10, 2, vec![0., 2., 4., 6., 8.])]
    #[case(10, 0, -1, vec![10., 9., 8., 7., 6., 5., 4., 3., 2., 1.])]
    #[case(-4, 4, 4, vec![-4., 0.])]
    #[case(4, -4, -4, vec![4., 0.])]
    #[case(1, 1, 1, vec![1.])]
    #[case(1, 1, -1, vec![1.])]
    fn rarray1d_range(#[case] start: i32, #[case] stop: i32, #[case] step: i32, #[case] result: Vec<f64>) {
        let v = Rarray1D::<f64>::range(start, stop, step);
        for i in 0..v.get_shape()[1] {
            assert_eq!(v[i], result[i]);
        }
    }

    #[rstest]
    #[case(0, 10, -1)]
    #[case(10, 0, 1)]
    #[should_panic]
    fn rarray1d_range_invalid_step(#[case] start: i32, #[case] stop: i32, #[case] step: i32) {
        let _ = Rarray1D::<f64>::range(start, stop, step);
    }
}
