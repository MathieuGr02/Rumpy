use std::iter::zip;

use super::rarray::{Rarray1D, Rarray2D, D1};

// Base functionality for the Rarray1D struct
impl Rarray1D {
    /// Create new rarray from passed array 
    pub fn new(data: &Vec<f64>) -> Self {
       Rarray1D {
            shape : D1 { width: data.len() as usize, height: 1 },
            data: data.clone()
       }
    }

    /// Create rarray filled with zeros
    pub fn zeros(size: usize) -> Self {
        Rarray1D {
            shape: D1 { width: size, height: 1},
            data: vec![0.; size]
        }
    }
    
    /// Create rarray filled with ones
    pub fn ones(size: usize) -> Self {
        Rarray1D {
            shape: D1 { width: size, height: 1},
            data: vec![1.; size]
        }
    }
}

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

