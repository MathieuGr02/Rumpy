use rand::seq::IndexedRandom;

use super::rarray::{Rarray1D, Rarray2D, D2};

impl Rarray2D {
    /// Create new rarray from passed array 
    pub fn new(data: &Vec<Vec<f64>>) -> Self {
        let width = data[0].len();
        for i in 1..data.len() {
            assert_eq!(width, data[i].len(), "All rows need to be of the same size");
        }
        Rarray2D {
            shape : D2 { width, height: data.len() },
            data: data.clone().into_iter().flatten().collect() 
        }
    }

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
