use super::rarray::{Rarray, RarrayAdd, RarraySub};
use std::iter::zip;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt::Debug;
use crate::linalg::dimension::Dim;
use crate::linalg::numeric_trait::Numeric;

impl<T, D> Rarray<T, D> where
    D: Dim
{
    /// Return shape of array
    pub fn get_shape(&self) -> &D {
        &self.shape
    }
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


impl<T, D> PartialEq for Rarray<T, D> where
    T: Numeric + PartialEq,
    D: Dim + Eq 
{
    fn eq(&self, other: &Self) -> bool {
        if self.shape != other.shape {
            return false;
        }

        for (x, y) in zip(&self.data, &other.data) {
            if x != y {
                return false;
            }
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
