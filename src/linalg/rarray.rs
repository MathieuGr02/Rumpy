use std::fmt::Debug;
use std::ops::Neg;
use std::usize;
use crate::linalg::numeric_trait::Numeric;
pub(crate) use super::dimension::{Dim, D1, D2, D3};

/// Base array struct
/// Consists of data field which is a 1 dimensional `Vec<T>` and of a shape field which is of type `Dim`
#[derive(Debug)]
pub struct Rarray<T, D: Dim> {
    pub(crate) data: Vec<T>,
    pub(crate) shape: D
}

// Specific implementations 
pub type Rarray1D<T> = Rarray<T, D1>;
pub type Rarray2D<T> = Rarray<T, D2>;
pub type Rarray3D<T> = Rarray<T, D3>;

pub trait RarrayCreate<T, V, S> {
    fn new(data: &V) -> Self;
    fn zeros(shape: T) -> Self;
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

impl<T> RarrayMul<Rarray1D<T>, Rarray2D<T>, Rarray1D<T>> for Rarray2D<T> where
    T: Numeric
{
    /// Performs (1 x n) x (n x m) matrix multiplication
    fn mul(one: &Rarray1D<T>, other: &Rarray2D<T>) -> Rarray1D<T> {
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
            data: vec![T::default(); major]
        };

        for i in 0..one.shape.width {
            for j in 0..major {
                result[i] += one[j] * other[[i, j]];
            }
        }

        result
    }
}

impl<T> RarrayMul<Rarray2D<T>, Rarray1D<T>, Rarray1D<T>> for Rarray2D<T> where
    T: Numeric
{
    /// Performs (n x m) x (m x 1) matrix multiplication
    fn mul(one: &Rarray2D<T>, other: &Rarray1D<T>) -> Rarray1D<T> {
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
            data: vec![T::default(); major]
        };

        for i in 0..one.shape.height {
            for j in 0..major {
                result[i] += one[[i, j]] * other[j];
            }
        }

        result
    }
}

impl<T> RarrayMul<Rarray2D<T>, Rarray2D<T>, Rarray2D<T>> for Rarray2D<T> where
    T: Numeric
{
    /// Performs (n x m) x (m x l) matrix multiplication
    fn mul(one: &Rarray2D<T>, other: &Rarray2D<T>) -> Rarray2D<T> {
        assert_eq!(one.shape.height, other.shape.width, "Rarray shape mismatch");

        let mut result = Rarray2D {
            shape: D2 { height: one.shape.height, width: other.shape.width },
            data: vec![T::default(); one.shape.height * other.shape.width]
        };

        for i in 0..one.shape.height {
            for j in 0..other.shape.width {
                for k in 0..one.shape.width {
                    result.data[i * result.shape.width + j * result.shape.height] += one.data[i * one.shape.width + k] * other.data[j * other.shape.height + k];
                }
            }
        }

        result
    }
}

impl<T, D> RarrayScalMul<T, Rarray<T, D>> for Rarray<T, D> where
    T: Numeric,
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
    T : Numeric,
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
    T : Numeric,
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