use std::iter::zip;
use std::ops::Index;

/// Dimension structs
/// Each dimension struct has its side lengths and an index trait.

// 1D struct. Has height and width, because of vector transposition.
#[derive(Debug, Clone, Copy)]
pub struct D1 {
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

impl PartialEq<Self> for D1 {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }

    fn ne(&self, other: &Self) -> bool {
        self.width == other.width || self.height == other.height
    }
}

impl Eq for D1 {
}

#[derive(Debug, Clone, Copy)]
pub struct D2 {
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

impl PartialEq<Self> for D2 {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }

    fn ne(&self, other: &Self) -> bool {
        self.width == other.width || self.height == other.height
    }
}

impl Eq for D2 {
}

#[derive(Debug, Clone, Copy)]
pub struct D3 {
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

impl PartialEq<Self> for D3 {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.depth == other.depth
    }

    fn ne(&self, other: &Self) -> bool {
        self.width == other.width || self.height == other.height || self.depth == other.depth
    }
}

impl Eq for D3 {
}

pub trait Dim {}

impl Dim for D1 {}
impl Dim for D2 {}
impl Dim for D3 {}
