use rand::seq::IndexedRandom;

use super::rarray::{Rarray, Rarray1D};
use core::panic;
use std::{default, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}, usize};

// Base operations for the Rarray abstract struct
impl<T, D> Add<&Rarray<T, D>> for &Rarray<T, D> where 
    T : Add<Output = T> + Copy + Default,
    D : Copy
{
    type Output = Rarray<T, D>;

    fn add(self, rhs: &Rarray<T, D>) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len(), "Incompatible dimensions");

        let mut add_rarray = Rarray::<T, D> {
            shape: self.shape,
            data: vec![T::default(); self.data.len()] 
        };
        
        for i in 0..self.data.len() {
            add_rarray.data[i] = self.data[i] + rhs.data[i];
        }

        add_rarray
    }
}

impl<T, D> AddAssign<&Rarray<T, D>> for Rarray<T, D> where 
    T: AddAssign + Copy + Default
{
    fn add_assign(&mut self, rhs: &Rarray<T, D>) {
        assert_eq!(self.data.len(), rhs.data.len(), "Incompatible dimensions");

        for i in 0..self.data.len() {
            self.data[i] += rhs.data[i]
        }
    }
}


impl<T, D> Sub<&Rarray<T, D>> for &Rarray<T, D> where 
    T : Sub<Output = T> + Copy + Default,
    D : Copy
{
    type Output = Rarray<T, D>;

    fn sub(self, rhs: &Rarray<T, D>) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len(), "Incompatible dimensions");

        let mut sub_rarray = Rarray::<T, D> {
            shape: self.shape,
            data: vec![T::default(); self.data.len()] 
        };
        
        for i in 0..self.data.len() {
            sub_rarray.data[i] = self.data[i] - rhs.data[i];
        }

        sub_rarray
    }
}


impl<T, D> SubAssign<&Rarray<T, D>> for Rarray<T, D> where 
    T: SubAssign + Copy + Default
{
    fn sub_assign(&mut self, rhs: &Rarray<T, D>) {
        assert_eq!(self.data.len(), rhs.data.len(), "Incompatible dimensions");

        for i in 0..self.data.len() {
            self.data[i] -= rhs.data[i]
        }
    }
}


