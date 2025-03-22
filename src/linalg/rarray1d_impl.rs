//! ```
//! use rumpy::linalg::rarray::Rarray1D;
//! use crate::rumpy::linalg::rarray::RarrayCreate;
//! ```

use core::fmt;
use std::ops::{Index, IndexMut, Mul, MulAssign};

use crate::linalg::dimension::D1;
use super::rarray::{Rarray, Rarray1D, RarrayCreate, RarrayScalMul};

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
        Rarray::scal_mul(rhs, self)
    }
}

impl MulAssign<f64> for Rarray1D {
    fn mul_assign(&mut self, rhs: f64) {
        self.data = Rarray::scal_mul(rhs, self).data;
    }
}

impl Mul<&Rarray1D> for &Rarray1D {
    type Output = f64;

    fn mul(self, rhs: &Rarray1D) -> Self::Output {
        assert_eq!(self.shape.width, rhs.shape.height, "Rarray shape mismatch");
        Rarray1D::dot(self, rhs)
    }
}

impl fmt::Display for Rarray1D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_string: String = String::new();
        for i in 0..self.shape.width {
            formatted_string.push_str(&self.data[i].to_string());
            formatted_string.push_str(", ");
        }
        formatted_string.pop();
        formatted_string.pop();
        write!(f, "Rarray1D([{}])", formatted_string)
    }
}