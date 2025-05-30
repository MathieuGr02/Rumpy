mod tests {
    use rstest::rstest;
    use rumpy::{linalg::rarray::{Rarray1D, Rarray2D, RarrayCreate, RarrayScalMul}, mat};

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
    #[case(mat![[1., 1.], [1., 1.]], 1., mat![[1., 1.], [1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.]], 0., mat![[0., 0.], [0., 0.]])]
    #[case(mat![[1., 2.], [3., 4.]], 3., mat![[3., 6.], [9., 12.]])]
    fn rarra2d_scalar_mul(#[case] m: Rarray2D<f64>, #[case] scalar: f64, #[case] result: Rarray2D<f64>){
        let rarray_mul_result = &m * scalar;
        for i in 0..(m.get_shape()[0]) {
            for j in 0..(m.get_shape()[1]) {
                assert_eq!(rarray_mul_result[[i, j]], result[[i, j]]); 
            }
        }
    }

    #[rstest]
    #[case(mat![[1., 1.], [1., 1.]], 1., mat![[1., 1.], [1., 1.]])]
    #[case(mat![[1., 1.], [1., 1.]], 0., mat![[0., 0.], [0., 0.]])]
    #[case(mat![[1., 2.], [3., 4.]], 3., mat![[3., 6.], [9., 12.]])]
    fn rarra2d_scalar_mul_assign(#[case] mut m: Rarray2D<f64>, #[case] scalar: f64, #[case] result: Rarray2D<f64>){
        m *= scalar;
        for i in 0..(m.get_shape()[0]){
            assert_eq!(m[i], result[i]); 
        }
    }
}
