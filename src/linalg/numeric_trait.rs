use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Specifies that a generic type is a numerical type
pub trait Numeric: 
    Add<Output = Self> 
    + AddAssign 
    + Sub<Output = Self>
    + SubAssign 
    + Div<Output = Self>
    + DivAssign 
    + Mul<Output = Self>
    + MulAssign 
    + Copy 
    + Default 
    + ToString
    + From<i32>
    {}

impl<T> Numeric for T where
    T:
    Add<Output = T> + AddAssign +
    Sub<Output = T> + SubAssign +
    Div<Output = T> + DivAssign +
    Mul<Output = T> + MulAssign + 
    ToString + Copy + Default + From<i32> {}