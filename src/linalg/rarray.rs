use std::{collections::btree_map::Range, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}, usize};

use rand::seq::IndexedRandom;

// Dimensions
pub type D2 = [usize; 2];
pub type D3 = [usize; 3];

// Base array struct 
#[derive(Debug)]
pub(crate) struct Rarray<T, D> {
    pub(crate) data: Vec<T>,
    pub(crate) shape: D
}

// Specific implementations 
pub type Rarray1D = Rarray<f64, D2>;
pub type Rarray2D = Rarray<f64, D2>;
pub type Rarray3D = Rarray<f64, D3>;
