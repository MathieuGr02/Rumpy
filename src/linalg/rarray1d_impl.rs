use std::ops::{Index, IndexMut, Mul, MulAssign};
use std::process::Output;
use std::slice::SliceIndex;

use super::rarray::{Rarray1D, D2};

// Base functionality for the Rarray1D struct
impl Rarray1D {
    /// Create new rarray from passed array 
    pub fn new(data: &[f64]) -> Self {
       Rarray1D {
            shape : [data.len(), 1],
            data: data.to_vec()
       }
    }

    /// Create rarray filled with zeros
    pub fn zeros(size: usize) -> Self {
        Rarray1D {
            shape: [size, 1],
            data: vec![0.; size]
        }
    }
    
    /// Create rarray filled with ones
    pub fn ones(size: usize) -> Self {
        Rarray1D {
            shape: [size, 1],
            data: vec![1.; size]
        }
    }
}

impl Rarray1D {
    fn transpose(&self) -> Self {
        Rarray1D {
            shape: [self.shape[1], self.shape[0]],
            data: self.data.clone()
        }
    }
}

impl Index<usize> for Rarray1D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if self.shape[0] < self.shape[1] {
            assert!(index < self.shape[1], "Index out of bounds");
        } 
        else {
            assert!(index < self.shape[0], "Index out of bounds");
        }

        &self.data[index]
    }
}

impl IndexMut<usize> for Rarray1D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.shape[0] < self.shape[1] {
            assert!(index < self.shape[1], "Index out of bounds");
        } 
        else {
            assert!(index < self.shape[0], "Index out of bounds");
        }

        &mut self.data[index]
    }
}

impl Mul<f64> for &Rarray1D {
    type Output = Rarray1D;

    fn mul(self, rhs: f64) -> Self::Output {
        Rarray1D {
            shape: self.shape,
            data: self.data.iter().clone().map(|x| rhs * x).collect()
        }
    }
}
