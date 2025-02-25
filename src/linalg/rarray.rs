use std::{collections::btree_map::Range, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}, usize};

use rand::seq::IndexedRandom;

// Dimensions
pub type D1 = [usize; 1];
pub type D2 = [usize; 2];
pub type D3 = [usize; 3];

pub trait Dimension {
    const SIZE: usize;

    fn count_items(&self) -> usize;
}

impl Dimension for D1 {
    const SIZE: usize = 1;

    fn count_items(&self) -> usize {
        let mut items: usize = 1; 
        for i in 0..self.len() {
            items *= self[i]; 
        }

        items
    }
}

impl Dimension for D2 {
    const SIZE: usize = 2;
    
    fn count_items(&self) -> usize {
        let mut items: usize = 1; 
        for i in 0..self.len() {
            items *= self[i]; 
        }

        items
    }
}

impl Dimension for D3 {
    const SIZE: usize = 3;

    fn count_items(&self) -> usize {
        let mut items: usize = 1; 
        for i in 0..self.len() {
            items *= self[i]; 
        }

        items
    }
}


// Base array struct 
#[derive(Debug)]
struct Rarray<T, D> where D: Dimension {
    pub(crate) data: Vec<T>,
    pub(crate) shape: D
}

// Specific implementations 
pub type Rarray1D = Rarray<f64, D1>;
pub type Rarray2D = Rarray<f64, D2>;
pub type Rarray3D = Rarray<f64, D3>;

impl<T, D> Add for Rarray<T, D> where 
    T : Add + Copy,
    D: Dimension + Iterator + Mul
{
    type Output = Rarray<T, D>;

    fn add(self, rhs: Self) -> Self::Output {
        let items: usize = self.shape.count_items();
        let mut add_rarray = Rarray::<T, D> {
            shape: self.shape,
            data: Vec::<T>::with_capacity(items) 
        };
        
        for i in 0..self.data.len() {
            add_rarray.data[i] = self.data[i] + rhs.data[i];
        }

        add_rarray
    }
}

mod tests {
    fn test_rarray1d(){
    }
}

/*
#[derive(Debug)]
pub struct Rarray {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) data: Vec<f64>
}
*/
