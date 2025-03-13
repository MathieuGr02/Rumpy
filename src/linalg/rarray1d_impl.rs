use std::ops::{Index, IndexMut};
use std::process::Output;
use std::slice::SliceIndex;

use super::rarray::{Rarray1D, D1};

// Base functionality for the Rarray1D struct
impl Rarray1D {
    /// Create new rarray from passed array 
    pub fn new(data: &[f64]) -> Self {
       Rarray1D {
            shape : [data.len()],
            data: data.to_vec()
       }
    }

    /// Create rarray filled with zeros
    pub fn zeros(size: usize) -> Self {
        Rarray1D {
            shape: [size],
            data: vec![0.; size]
        }
    }
    
    /// Create rarray filled with ones
    pub fn ones(size: usize) -> Self {
        Rarray1D {
            shape: [size],
            data: vec![1.; size]
        }
    }
}

impl Index<usize> for Rarray1D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Rarray1D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
