mod test {
    use rumpy::linalg::rarray::{MatOper, Rarray1D, Rarray2D};
    use rstest::rstest;

    #[test]
    #[should_panic]
    fn rarray2d_mul(){
        let x = Rarray1D::new(&vec![1., 1.]);
        let y = Rarray2D::new(&vec![vec![1., 1.], vec![1., 1.]]);

        let z = Rarray2D::mul(x, y);

        let x = Rarray1D::new(&vec![1., 1.]);
        let y = Rarray2D::new(&vec![vec![1., 1.], vec![1., 1.]]);
        let y1 = Rarray2D::new(&vec![vec![1., 1.], vec![1., 1.]]);
        
        let s = Rarray2D::mul(y, y1);
        println!("{:?}", z);
        println!("{:?}", s);
    }
}
