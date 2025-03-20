use std::iter::zip;

use super::rarray::{Rarray1D, Rarray2D, RarrayCreate, D1};

impl Rarray1D {
    /// Tranpose 1D matrix
    pub fn transpose(&self) -> Self {
        Rarray1D {
            shape: D1 { width: self.shape.height, height: self.shape.width },
            data: self.data.clone()
        }
    }

    /// Tranpose 1D matrix inplace
    pub fn mut_transpose(&mut self){
        self.shape = D1 { width: self.shape.height, height: self.shape.width };
    } 
}

impl Rarray1D {
    /// Calculate dot product of two vectors
    pub fn dot(a: &Self, b: &Self) -> f64 {
        assert_eq!(a.data.len(), b.data.len(), "Vectors not of same size");
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
    /// Sum values of array
    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }
   
    // TODO: yet to implement functionality
    /*
    pub fn unique(&self) -> Rarray1D {
        
    }

    pub fn unique_index(&self) -> Rarray1D {

    }

    pub fn diag(&self) -> Rarray2D {
    
    }
    */
}

