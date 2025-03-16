use core::panic;
use std::{collections::btree_map::Range, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}, usize};

// Dimensions
#[derive(Debug, Clone, Copy)]
pub struct D1{
    pub width: usize,
    pub height: usize
}

impl Index<usize> for D1 {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.height,
            1 => &self.width,
            _ => panic!("Index out of range")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct D2{
    pub width: usize,
    pub height: usize
}

impl Index<usize> for D2 {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.height,
            1 => &self.width,
            _ => panic!("Index out of range")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct D3{
    pub width: usize,
    pub height: usize,
    pub depth: usize
}

impl Index<usize> for D3 {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.height,
            1 => &self.width,
            2 => &self.depth,
            _ => panic!("Index out of range")
        }
    }
}

// Base array struct 
#[derive(Debug)]
pub struct Rarray<T, D> {
    pub(crate) data: Vec<T>,
    pub(crate) shape: D
}

// Specific implementations 
pub type Rarray1D = Rarray<f64, D1>;
pub type Rarray2D = Rarray<f64, D2>;
pub type Rarray3D = Rarray<f64, D3>;
