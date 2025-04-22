pub mod rarray;
pub mod rarray_impl;

mod rarray1d_impl;
pub mod rarray1d_ops;
pub mod rarray1d_create;

mod rarray2d_impl;
pub mod rarray2d_ops;
mod dimension;
mod numeric_trait;


#[macro_export]
macro_rules! mat {
    // For n x 1 vector
    ( $( [ $x:expr ] ),* ) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push(vec![$x]);
            )*
            Rarray1D::new(&temp)
        }
    };

    // For n x n matrix
    ( $( [ $( $x:expr ),+ ] ),* ) => {
        {
            let mut temp = Vec::new();
            $(
                let mut inner = Vec::new();
                $(
                    inner.push($x);
                )*
                temp.push(inner);
            )*
            Rarray2D::new(&temp)
        }
    };
    
    // For 1 x n vector
    ( $( $x:expr ),* ) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            Rarray1D::new(&temp)
        }
    };
}
