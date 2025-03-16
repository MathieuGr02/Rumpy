use super::rarray::{Rarray1D, Rarray2D, D2};

impl Rarray2D {
    /// Create new rarray from passed array 
    /*
    pub fn new(data: &[&[f64]]) -> Self {
       Rarray2D {
            shape : [data.len(), 1],
            data: data.to_vec()
       }
    }
    */

    /// Create rarray filled with zeros
    pub fn zeros(width: usize, height: usize) -> Self {
        Rarray2D {
            shape: D2 { width, height },
            data: vec![0.; width * height]
        }
    }
    
    /// Create rarray filled with ones
    pub fn ones(size: usize) -> Self {
        Rarray2D {
            shape: D2 { width: size, height: size },
            data: vec![1.; size * size]
        }
    }
}

impl Rarray2D {
    /*
    pub fn mul(self: &Rarray2D, other: &Rarray2D) -> Rarray2D {
        
    }
    */
}
