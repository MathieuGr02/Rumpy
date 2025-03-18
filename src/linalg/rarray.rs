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
    pub(crate) data: Tec<T>,
    pub(crate) shape: D
}

// Specific implementations 
pub type Rarray1D = Rarray<f64, D1>;
pub type Rarray2D = Rarray<f64, D2>;
pub type Rarray3D = Rarray<f64, D3>;

pub trait RarrayCreate {
    fn new<T, V>(data: T) -> V;
    fn zeros<T, V>(shape: T) -> V;
    fn ones<T, V>(shape: T) -> V;
    fn random<T, V>(shape: T) -> V;
}

pub trait RarrayMul<T, V, S> {
    fn mul(one: T, other: V) -> S;
}

pub trait RarrayAdd<T, V, S> {
    fn add(one: T, other: V) -> S;
}

pub trait RarraySub<T, V, S> {
    fn sub(one: T, other: V) -> S;
}

impl RarrayMul<Rarray1D, Rarray2D, Rarray1D> for Rarray2D {
    fn mul(one: Rarray1D, other: Rarray2D) -> Rarray1D {
        let mut major = 1;
        if one.shape.width > one.shape.height {
            assert_eq!(one.shape.width, other.shape.height, "Rarray shape mismatch");
            major = one.shape.width;
        } else {
            assert_eq!(one.shape.height, other.shape.height, "Rarray shape mismatch");
            major = one.shape.height;
        }

        let result = Rarray1D {
            shape: D1 { width: major, height: 1 },
            data: vec![0.; major]
        };

        result
    }
}

impl RarrayMul<Rarray2D, Rarray1D, Rarray1D> for Rarray2D {
    fn mul(one: Rarray2D, other: Rarray1D) -> Rarray1D {
       other
    }
}

impl RarrayMul<Rarray2D, Rarray2D, Rarray2D> for Rarray2D {
    fn mul(one: Rarray2D, other: Rarray2D) -> Rarray2D {
        one 
    }
}

impl RarrayAdd<Rarray2D, Rarray2D, Rarray2D> for Rarray2D {
    fn add(one: Rarray2D, other: Rarray2D) -> Rarray2D {
        assert_eq!(one.shape.width, other.shape.width, "Rarray shape mismatch");
        assert_eq!(one.shape.height, other.shape.height, "Rarray shape mismatch");

        let mut result = Rarray2D {
            shape: one.shape,
            data: vec![0; one.shape.width * one.shape.height]
        };

        for i in 0..one.shape.width {
            for j in 0..one.shape.height {
                result[[i, j]] = one[[i, j]] + other[[i, j]];
            }
        }

        result
    }
}

impl RarraySub<Rarray2D, Rarray2D, Rarray2D> for Rarray2D {
    fn sub(one: Rarray2D, other: Rarray2D) -> Rarray2D {
        assert_eq!(one.shape.width, other.shape.width, "Rarray shape mismatch");
        assert_eq!(one.shape.height, other.shape.height, "Rarray shape mismatch");

        let mut result = Rarray2D {
            shape: one.shape,
            data: vec![0; one.shape.width * one.shape.height]
        };

        for i in 0..one.shape.width {
            for j in 0..one.shape.height {
                result[[i, j]] = one[[i, j]] - other[[i, j]];
            }
        }

        result
    }
}
