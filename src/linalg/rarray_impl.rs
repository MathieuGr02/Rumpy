use super::rarray::{Rarray, Rarray1D, RarrayAdd, RarraySub};
use std::{ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}, usize};
use std::fmt::Debug;
use std::ops::Neg;
use crate::linalg::dimension::Dim;
use crate::linalg::numeric_trait::Numeric;

impl<T, D> Rarray<T, D> {
    /// Return shape of array
    pub fn get_shape(&self) -> &D {
        &self.shape
    }
    // TODO: Implement following functions

    /// Move ownership of array
    pub fn to_owned(self) {}
}

// Base operations for the Rarray abstract struct
impl<T, D> Add<&Rarray<T, D>> for &Rarray<T, D> where
    T : Numeric,
    D : Copy + Dim + Debug + Eq
{
    type Output = Rarray<T, D>;

    fn add(self, rhs: &Rarray<T, D>) -> Self::Output {
        Rarray::add(self, rhs)
    }
}

impl<T, D> AddAssign<&Rarray<T, D>> for Rarray<T, D> where 
    T: Numeric,
    D: Copy + Dim + Debug + Eq
{
    fn add_assign(&mut self, rhs: &Rarray<T, D>) {
        self.data = Rarray::add(self, rhs).data;
    }
}


impl<T, D> Sub<&Rarray<T, D>> for &Rarray<T, D> where 
    T : Numeric,
    D : Copy + Dim + Debug + Eq
{
    type Output = Rarray<T, D>;

    fn sub(self, rhs: &Rarray<T, D>) -> Self::Output {
        Rarray::sub(self, rhs)
    }
}


impl<T, D> SubAssign<&Rarray<T, D>> for Rarray<T, D> where
    T : Numeric,
    D : Copy + Dim + Debug + Eq
{
    fn sub_assign(&mut self, rhs: &Rarray<T, D>) {
        self.data = Rarray::sub(self, rhs).data;
    }
}


