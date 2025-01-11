use std::ops::{Add, Div};

pub fn mean<T>(object: &[T]) -> f64
where
    T: Add<Output = T> + Div<Output = T> + Iterator<Item = f64>,
{
    let sum: f64 = object.iter().copied().reduce(|a, b| a + b);
    0.
}
