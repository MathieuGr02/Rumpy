use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Numeric: Add + AddAssign + Sub + SubAssign + Div + DivAssign + Mul + MulAssign + Copy + Default {}

impl<T> Numeric for T where
    T:
    Add<Output = T> + AddAssign +
    Sub<Output = T> + SubAssign +
    Div<Output = T> + DivAssign +
    Mul<Output = T> + MulAssign +
    Copy + Default {}