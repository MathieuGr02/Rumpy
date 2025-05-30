mod tests {
    use rstest::rstest;
    use rumpy::{linalg::rarray::{Rarray1D, Rarray2D, RarrayCreate}, mat};

    #[rstest]
    #[case(mat![1.], mat![1.], mat![2.])]
    #[case(mat![1., 1., 1.], mat![1., 1., 1.], mat![2., 2., 2.])]
    #[case(mat![0.], mat![0.], mat![0.])]
    #[case(mat![0., 2., 3.5, 0.75, 1.], mat![1., 0.25, 8., 0., 1.25], mat![1., 2.25, 11.5, 0.75, 2.25])]
    fn rarray1d_addition(#[case] m: Rarray1D<f64>, #[case] n: Rarray1D<f64>, #[case] result: Rarray1D<f64>){
        let rarray_result = &m + &n;
        for i in 0..(m.get_shape()[0]) {
            assert_eq!(rarray_result[i], result[i]);
        }
    }

    #[rstest]
    #[case(mat![1.], mat![1., 1.])]
    #[case(mat![1., 1.], mat![1.])]
    #[should_panic]
    fn rarray1d_addition_incompatible(#[case] m: Rarray1D<f64>, #[case] n: Rarray1D<f64>){
        let _ = &m + &n;
    }

    #[rstest]
    #[case(mat![1.], mat![1.], mat![2.])]
    #[case(mat![1., 1., 1.], mat![1., 1., 1.], mat![2., 2., 2.])]
    #[case(mat![0.], mat![0.], mat![0.])]
    #[case(mat![0., 2., 3.5, 0.75, 1.], mat![1., 0.25, 8., 0., 1.25], mat![1., 2.25, 11.5, 0.75, 2.25])]
    fn rarray1d_inplace_addition(#[case] mut m: Rarray1D<f64>, #[case] n: Rarray1D<f64>, #[case] result: Rarray1D<f64>){
        m += &n;
        for i in 0..(m.get_shape()[0]) {
            assert_eq!(m[i], result[i]);
        }
    }

    #[rstest]
    #[case(mat![1.], mat![1., 1.])]
    #[case(mat![1., 1.], mat![1.])]
    #[should_panic]
    fn rarray1d_inplace_addition_incompatible(#[case] mut m: Rarray1D<f64>, #[case] n: Rarray1D<f64>){
        m += &n;
    }

    #[rstest]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]])]
    #[case(mat![[0., 0., 1.], [0., 1., 0.], [1., 0., 0.]], mat![[0., 0., 1.], [0., 1., 0.], [1., 0., 0.]], mat![[0., 0., 2.], [0., 2., 0.], [2., 0., 0.]])]
    #[case(mat![[0., 0., 1.], [0., 1., 0.], [1., 0., 0.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 1.], [0., 2., 0.], [1., 0., 1.]])]
    #[case(mat![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]], mat![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]], mat![[2., 2., 2.], [2., 2., 2.], [2., 2., 2.]])]
    fn rarray2d_addition(#[case] m: Rarray2D<f64>, #[case] n: Rarray2D<f64>, #[case] result: Rarray2D<f64>) {
        let rarray_result = &m + &n;
        for i in 0..(m.get_shape()[0]) {
            for j in 0..(m.get_shape()[1]) {
                assert_eq!(rarray_result[[i, j]], result[[i, j]]);
            }
        }
    }

    #[rstest]
    #[case(mat![[1., 1., 1.], [1., 1., 1.]], mat![[1., 1.], [1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.]], mat![[1., 1., 1.], [1., 1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.], [1., 1.]], mat![[1., 1.], [1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.]], mat![[1., 1.], [1., 1.], [1., 1.]])]
    #[should_panic]
    fn rarray2d_addition_incompatible(#[case] m: Rarray2D<f64>, #[case] n: Rarray2D<f64>) { 
        let _ = &m + &n;
    }

    #[rstest]
    #[case(mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]])]
    #[case(mat![[0., 0., 1.], [0., 1., 0.], [1., 0., 0.]], mat![[0., 0., 1.], [0., 1., 0.], [1., 0., 0.]], mat![[0., 0., 2.], [0., 2., 0.], [2., 0., 0.]])]
    #[case(mat![[0., 0., 1.], [0., 1., 0.], [1., 0., 0.]], mat![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]], mat![[1., 0., 1.], [0., 2., 0.], [1., 0., 1.]])]
    #[case(mat![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]], mat![[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]], mat![[2., 2., 2.], [2., 2., 2.], [2., 2., 2.]])]
    fn rarray2d_inplace_addition(#[case] mut m: Rarray2D<f64>, #[case] n: Rarray2D<f64>, #[case] result: Rarray2D<f64>) {
        m += &n;
        for i in 0..(m.get_shape()[0]) {
            for j in 0..(m.get_shape()[1]) {
                assert_eq!(m[[i, j]], result[[i, j]]);
            }
        }
    }

    #[rstest]
    #[case(mat![[1., 1., 1.], [1., 1., 1.]], mat![[1., 1.], [1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.]], mat![[1., 1., 1.], [1., 1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.], [1., 1.]], mat![[1., 1.], [1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.]], mat![[1., 1.], [1., 1.], [1., 1.]])]
    #[should_panic]
    fn rarray2d_inplace_addition_incompatible(#[case] mut m: Rarray2D<f64>, #[case] n: Rarray2D<f64>) { 
        m += &n;
    }
}
