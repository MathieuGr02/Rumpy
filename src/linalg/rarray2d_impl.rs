//! ```
//! use rumpy::linalg::rarray::Rarray2D;
//! use crate::rumpy::linalg::rarray::RarrayCreate;
//! ```

use core::fmt;
use std::ops::{Mul, MulAssign, Index, IndexMut};

use super::rarray::{Rarray2D, RarrayCreate, RarrayMul, D2};

impl Index<[usize; 2]> for Rarray2D {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        assert!((index[0] < self.shape[0]) && index[1] < self.shape[1], "Index out of bounds");
        &self.data[index[0] * self.shape[0] + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Rarray2D {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        assert!((index[0] < self.shape[0]) && index[1] < self.shape[1], "Index out of bounds");
        &mut self.data[index[0] * self.shape[0] + index[1]]
    }
}

impl Mul<&Rarray2D> for &Rarray2D {
    type Output = Rarray2D;

    fn mul(self, rhs: &Rarray2D) -> Self::Output {
        Rarray2D::mul(self, rhs)
    }
}

impl MulAssign<&Rarray2D> for Rarray2D {
    fn mul_assign(&mut self, rhs: &Rarray2D) {
        self.data = Rarray2D::mul(self, rhs).data;
    }
}

impl fmt::Display for Rarray2D {
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

impl RarrayCreate<(usize, usize), Vec<Vec<f64>>, f64> for Rarray2D {
    /// Create 2D matrix using Vec<Vec>
    ///
    /// # Examples
    ///
    /// ```
    ///use rumpy::linalg::rarray::Rarray2D;
    ///use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    ///let m = Rarray2D::new(&vec![
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
    fn new(data: &Vec<Vec<f64>>) -> Self {
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
    /// let m = Rarray2D::zeros((3, 3));
    /// println!("{}", m);
    /// // >> Rarray2D([0., 0., 0.], [0., 0., 0.], [0., 0., 0.])
    /// ```
    fn zeros(shape: (usize, usize)) -> Self {
        Rarray2D {
            shape: D2 { height: shape.0, width: shape.1 },
            data: vec![0.; shape.0 * shape.1]
        } 
    }

    /// Create 2D matrix of shape `m x n` filled with random numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray2D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let m = Rarray2D::random((3, 3));
    /// println!("{}", m);
    /// // >> Rarray2D([])
    /// ```
    fn random(shape: (usize, usize)) -> Self {
        let mut data = Vec::<f64>::with_capacity(shape.0 * shape.1);
        for _ in 0..(shape.0 * shape.1) {
            data.push(rand::random::<f64>());
        }

        Rarray2D {
            shape: D2 { height: shape.0, width: shape.1 },
            data
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
    fn fill(value: f64, shape: (usize, usize)) -> Self {
        Rarray2D {
            shape: D2 { height: shape.0, width: shape.1 },
            data: vec![value; shape.0 * shape.1]
        } 
    }
}
