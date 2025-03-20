use std::fmt::Debug;
use std::ops::{Add, Mul, MulAssign, Sub};
use std::usize;
use rand::seq::IndexedRandom;

pub(crate) use super::dimension::{Dim, D1, D2, D3};

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

pub trait RarrayCreate<T, V, S> {
    fn new(data: &V) -> Self;
    fn zeros(shape: T) -> Self;
    fn random(shape: T) -> Self;
    fn fill(value: S, shape: T) -> Self;
}

pub trait RarrayMul<T, V, S> {
    fn mul(one: &T, other: &V) -> S;
}

pub trait RarrayScalMul<T, V> {
    fn scal_mul(scal: T, rarray: &V) -> V;
}

pub trait RarrayAdd<T, V, S> {
    fn add(one: &T, other: &V) -> S;
}

pub trait RarraySub<T, V, S> {
    fn sub(one: &T, other: &V) -> S;
}

impl RarrayMul<Rarray1D, Rarray2D, Rarray1D> for Rarray2D {
    /// Performs (1 x n) x (n x m) matrix multiplication
    fn mul(one: &Rarray1D, other: &Rarray2D) -> Rarray1D {
        let mut major: usize = 1;
        if one.shape.width > one.shape.height {
            assert_eq!(one.shape.width, other.shape.height, "Rarray shape mismatch");
            major = one.shape.width;
        } else {
            assert_eq!(one.shape.height, other.shape.height, "Rarray shape mismatch");
            major = one.shape.height;
        }

        let mut result = Rarray1D {
            shape: D1 { width: one.shape.width, height: 1 },
            data: vec![0.; major]
        };

        for i in 0..one.shape.width {
            let mut sum: f64 = 0.;
            for j in 0..major {
                sum += one[j] * other[[i, j]];
            }
            result[i] = sum;
        }

        result
    }
}

impl RarrayMul<Rarray2D, Rarray1D, Rarray1D> for Rarray2D {
    /// Performs (n x m) x (m x 1) matrix multiplication
    fn mul(one: &Rarray2D, other: &Rarray1D) -> Rarray1D {
        let mut major: usize = 1;
        if one.shape.width > one.shape.height {
            assert_eq!(one.shape.width, other.shape.height, "Rarray shape mismatch");
            major = one.shape.width;
        } else {
            assert_eq!(one.shape.height, other.shape.height, "Rarray shape mismatch");
            major = one.shape.height;
        }

        let mut result = Rarray1D {
            shape: D1 { width: one.shape.height, height: 1 },
            data: vec![0.; major]
        };

        for i in 0..one.shape.height {
            let mut sum: f64 = 0.;
            for j in 0..major {
                sum += one[[i, j]] * other[j];
            }
            result[i] = sum;
        }

        result
    }
}

impl RarrayMul<Rarray2D, Rarray2D, Rarray2D> for Rarray2D {
    /// Performs (n x m) x (m x l) matrix multiplication
    fn mul(one: &Rarray2D, other: &Rarray2D) -> Rarray2D {
        assert_eq!(one.shape.height, other.shape.width, "Rarray shape mismatch");

        let mut result = Rarray2D {
            shape: D2 { height: one.shape.height, width: other.shape.width },
            data: vec![0.; one.shape.height * other.shape.width]
        };

        for i in 0..one.shape.height {
            for j in 0..other.shape.width {
                let mut sum: f64 = 0.;
                for k in 0..one.shape.width {
                    sum += one.data[i * one.shape.width + k] * other.data[j * other.shape.height + k];
                }
                result.data[i * result.shape.width + j * result.shape.height] = sum;
            }
        }

        result
    }
}

impl<T, D> RarrayScalMul<T, Rarray<T, D>> for Rarray<T, D> where
    T: Copy + MulAssign,
    D : Copy + Dim + Debug,
{
    fn scal_mul(scal: T, rarray: &Rarray<T, D>) -> Rarray<T, D> {
        let mut result = Rarray {
            shape: rarray.shape.clone(),
            data: rarray.data.clone()
        };
        
        for i in 0..rarray.data.len() {
            result.data[i] *= scal;
        }

        result
    }
}

impl<T, D> RarrayAdd<Rarray<T, D>, Rarray<T, D>, Rarray<T, D>> for Rarray<T, D> where
    T : Copy + Add<Output = T> + Default,
    D : Copy + Dim + Debug + Eq
{
    fn add(one: &Rarray<T, D>, other: &Rarray<T, D>) -> Rarray<T, D> {
        assert_eq!(one.shape, other.shape, "Rarray shape mismatch");

        let mut result = Rarray {
            shape: one.shape,
            data: vec![T::default(); one.data.len()]
        };

        for i in 0..one.data.len() {
            result.data[i] = one.data[i] + other.data[i];
        }

        result
    }
}

impl<T, D> RarraySub<Rarray<T, D>, Rarray<T, D>, Rarray<T, D>> for Rarray<T, D> where
    T : Copy + Sub<Output = T> + Default,
    D : Copy + Dim + Debug + Eq
{
    fn sub(one: &Rarray<T, D>, other: &Rarray<T, D>) -> Rarray<T, D> {
        assert_eq!(one.shape, other.shape, "Rarray shape mismatch");

        let mut result = Rarray {
            shape: one.shape,
            data: vec![T::default(); one.data.len()]
        };

        for i in 0..one.data.len() {
            result.data[i] = one.data[i] - other.data[i];
        }

        result
    }
}
