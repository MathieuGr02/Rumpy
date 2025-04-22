//! ```
//! use rumpy::linalg::rarray::Rarray1D;
//! use crate::rumpy::linalg::rarray::RarrayCreate;
//! ```

use core::fmt;
use std::ops::{Index, IndexMut, Mul, MulAssign};
use crate::linalg::numeric_trait::Numeric;
use super::rarray::{Rarray, Rarray1D, RarrayScalMul};

impl<T> Index<usize> for Rarray1D<T> {
    type Output = T;

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

impl<T> IndexMut<usize> for Rarray1D<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if self.shape[0] < self.shape[1] {
            assert!(index < self.shape[1], "Index out of bounds");
        } 
        else {
            assert!(index < self.shape[0], "Index out of bounds");
        }

        &mut self.data[index]
    }
}

impl<T> Mul<T> for &Rarray1D<T> where
    T: Numeric
{
    type Output = Rarray1D<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Rarray::scal_mul(rhs, self)
    }
}

impl<T> MulAssign<T> for Rarray1D<T> where
    T: Numeric
{
    fn mul_assign(&mut self, rhs: T) {
        self.data = Rarray::scal_mul(rhs, self).data;
    }
}

impl<T> Mul<&Rarray1D<T>> for &Rarray1D<T>
    where T: Numeric
{
    type Output = T;

    fn mul(self, rhs: &Rarray1D<T>) -> Self::Output {
        assert_eq!(self.shape.width, rhs.shape.height, "Rarray shape mismatch");
        Rarray1D::dot(self, rhs)
    }
}

impl<T> fmt::Display for Rarray1D<T> where
    T: Numeric
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
