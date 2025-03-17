use core::panic;
use std::{ops::Index, usize};

// Dimensions
#[derive(Debug, Clone, Copy)]
pub struct D1{
    pub height: usize,
    pub width: usize
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
    pub height: usize,
    pub width: usize
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
    pub height: usize,
    pub width: usize,
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

pub trait Dimension {}

impl Dimension for D1 {}
impl Dimension for D2 {}
impl Dimension for D3 {}

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

