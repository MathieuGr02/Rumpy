use std::ops::{Add, Div};

pub fn mean<T: Copy>(object: &[T]) -> f64
where
    T: Add<Output = T> + Div<Output = T> + Iterator<Item = T>,
{
    let sum: T = object.iter().copied().reduce(|a, b| a + b).unwrap();
    let length = object.iter();
}

pub fn std<T>(object: &[T]) -> f64
where
    T:,
{
}
