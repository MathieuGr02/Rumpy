use std::iter::zip;

use rand::seq::IndexedRandom;

use super::rarray::{Rarray1D, Rarray2D};

impl Rarray1D {
    /// Calculate dot product of two vectors
    pub fn dot(a: &Self, b: &Self) -> f64 {
        assert_eq!(a.data.len(), b.data.len(), "Vectors of not same size");
        let mut result: f64 = 0.;
        for (x, y) in zip(&a.data, &b.data) {
            result += x * y;
        }
        result
    }

    /*
    pub fn outer(a: &Self, b: &Self) -> Rarray2D {
        let mut outer_matrix: Rarray2D = Rarray2D { 
            shape: 
        }; 
        for i in &a.data.iter() {
            for j in &b.data.iter() {
                
            }
        }
    }
    */
}

