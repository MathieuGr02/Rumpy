//! ```
//! use rumpy::linalg::rarray::Rarray2D;
//! use crate::rumpy::linalg::rarray::RarrayCreate;
//! ```

use core::fmt;
use std::ops::{Mul, MulAssign, Index, IndexMut};
use crate::linalg::numeric_trait::Numeric;
use super::rarray::{Rarray2D, RarrayCreate, RarrayMul, D2};

impl<T> Index<[usize; 2]> for Rarray2D<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        assert!((index[0] < self.shape[0]) && index[1] < self.shape[1], "Index out of bounds");
        &self.data[index[0] * self.shape[0] + index[1]]
    }
}

impl<T> IndexMut<[usize; 2]> for Rarray2D<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        assert!((index[0] < self.shape[0]) && index[1] < self.shape[1], "Index out of bounds");
        &mut self.data[index[0] * self.shape[0] + index[1]]
    }
}

impl<T> Mul<&Rarray2D<T>> for &Rarray2D<T> where 
    T: Numeric
{
    type Output = Rarray2D<T>;

    fn mul(self, rhs: &Rarray2D<T>) -> Self::Output {
        Rarray2D::mul(self, rhs)
    }
}

impl<T> MulAssign<&Rarray2D<T>> for Rarray2D<T> where
    T: Numeric
{
    fn mul_assign(&mut self, rhs: &Rarray2D<T>) {
        self.data = Rarray2D::mul(self, rhs).data;
    }
}

impl<T> fmt::Display for Rarray2D<T> where 
    T: Numeric
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_string: String = String::new();
        for i in 0..self.shape.height {
            for j in 0..self.shape.width {
                formatted_string.push_str(&self.data[self.shape.height * i + j].to_string());
                formatted_string.push_str(", ");
            }
            formatted_string.pop();
            formatted_string.push_str("]");
            formatted_string.push_str(",");
        }
        formatted_string.pop();
        write!(f, "[{}]", formatted_string)
    }
}

impl<T> RarrayCreate<(usize, usize), Vec<Vec<T>>, T> for Rarray2D<T> where
    T: Numeric
{
    /// Create 2D matrix using Vec<Vec>
    ///
    /// # Examples
    ///
    /// ```
    ///use rumpy::linalg::rarray::Rarray2D;
    ///use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    ///let m = Rarray2D::<f64>::new(&vec![
    ///     vec![1., 1., 1.],
    ///     vec![1., 1., 1.],
    ///     vec![1., 1., 1.]
    /// ]);
    /// println!("{}", m);
    /// // >> Rarray2D([1., 1., 1.], [1., 1., 1.], [1., 1., 1.])
    /// ```
    ///
    /// # Panics
    ///
    /// If not all rows are of same length
    fn new(data: &Vec<Vec<T>>) -> Self {
        let row = data.len();
        let col = data[0].len();
        for i in 0..row {
            assert_eq!(col, data[i].len(), "All rows must be of same length");
        }

        Rarray2D {
            shape: D2 { height: row, width: col },
            data: data.clone().into_iter().flatten().collect()
        } 
    }

    /// Create 2D matrix of shape `m x n` filled with zeros
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray2D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let m = Rarray2D::<f64>::zeros((3, 3));
    /// println!("{}", m);
    /// // >> Rarray2D([0., 0., 0.], [0., 0., 0.], [0., 0., 0.])
    /// ```
    fn zeros(shape: (usize, usize)) -> Self {
        Rarray2D {
            shape: D2 { height: shape.0, width: shape.1 },
            data: vec![T::default(); shape.0 * shape.1]
        } 
    }
    
    /// Create 2D matrix of shape `m x n` filled with `value`
    ///
    /// # Examples 
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray2D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    /// 
    /// let m = Rarray2D::fill(4., (3, 3));
    /// println!("{}", m);
    /// // >> Rarray2D([[4., 4., 4.], [4., 4., 4.], [4., 4., 4.]])
    /// ```
    fn fill(value: T, shape: (usize, usize)) -> Self {
        Rarray2D {
            shape: D2 { height: shape.0, width: shape.1 },
            data: vec![value; shape.0 * shape.1]
        } 
    }
}

impl<T> Rarray2D<T> where
    T: Numeric
{
    /// Create 2D matrix of shape `m x n` filled with `value`
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray2D;
    ///
    /// let m = Rarray2D::<f64>::ones(3);
    /// println!("{}", m);
    /// // >> Rarray2D([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    /// ```
    pub fn ones(shape: usize) -> Self {
        let mut result = Rarray2D {
            shape: D2 { height: shape, width: shape },
            data: vec![T::default(); shape * shape]
        };

        for i in 0..shape {
            result.data[shape * i + i] = T::from(1);
        }

        result
    }
}
