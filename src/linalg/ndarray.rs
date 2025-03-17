use std::usize;

use super::rarray::Rarray;

pub type NRarray<'a> = Rarray<f64, &'a[usize]>;
