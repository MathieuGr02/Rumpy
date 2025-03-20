//! ```
//! use rumpy::linalg::rarray::Rarray1D;
//! use crate::rumpy::linalg::rarray::RarrayCreate;
//! ```

use core::fmt;
use std::ops::{Index, IndexMut, Mul, MulAssign};

use crate::linalg::dimension::D1;
use super::rarray::{Rarray, Rarray1D, RarrayCreate, RarrayScalMul};

impl Index<usize> for Rarray1D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if self.shape[0] < self.shape[1] {
            assert!(index < self.shape[1], "Index out of bounds");
        } 
        else {
            assert!(index < self.shape[0], "Index out of bounds");
        }

        &self.data[index]
    }
}

impl IndexMut<usize> for Rarray1D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.shape[0] < self.shape[1] {
            assert!(index < self.shape[1], "Index out of bounds");
        } 
        else {
            assert!(index < self.shape[0], "Index out of bounds");
        }

        &mut self.data[index]
    }
}

impl Mul<f64> for &Rarray1D {
    type Output = Rarray1D;

    fn mul(self, rhs: f64) -> Self::Output {
        Rarray::scal_mul(rhs, self)
    }
}

impl MulAssign<f64> for Rarray1D {
    fn mul_assign(&mut self, rhs: f64) {
        self.data = Rarray::scal_mul(rhs, self).data;
    }
}

impl Mul<&Rarray1D> for &Rarray1D {
    type Output = f64;

    fn mul(self, rhs: &Rarray1D) -> Self::Output {
        assert_eq!(self.shape.width, rhs.shape.height, "Rarray shape mismatch");
        Rarray1D::dot(self, rhs)
    }
}

impl fmt::Display for Rarray1D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_string: String = String::new();
        for i in 0..self.shape.height {
            formatted_string.push_str(&self.data[i].to_string());
            formatted_string.push_str(",");
        }
        formatted_string.pop();
        write!(f, "Rarray1D([{}])", formatted_string)
    }
}

// Creation
impl RarrayCreate<usize, Vec<f64>, f64> for Rarray1D {
    /// Create rarray1D vector using Vec 
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::new(&vec![1., 1., 1.]);
    /// println!("{}", v);
    /// ```
    fn new(data: &Vec<f64>) -> Self {
        Rarray1D {
            shape : D1 { width: data.len() as usize, height: 1 },
            data: data.clone()
        }
    }

    /// Create rarra1d vector of length `shape` filled with zeros
    ///
    /// # Examples
    ///
    /// ```
    ///use rumpy::linalg::rarray::Rarray1D;
    ///use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::zeros(3);
    /// println!("{}", v);
    /// ```
    fn zeros(shape: usize) -> Self {
        Rarray1D {
            shape: D1 { width: shape, height: 1},
            data: vec![0.; shape]
        }
    }
    
    /// Create rarray1D vector of length `shape` filled with random numbers
    ///
    /// # Examples
    /// 
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::random(3);
    /// println!("{}", v);
    /// ```
    fn random(shape: usize) -> Self {
        let mut data = Vec::<f64>::with_capacity(shape);
        for _ in 0..shape {
            data.push(rand::random::<f64>());
        }
        

        Rarray1D {
            shape: D1 { width: shape, height: 1 },
            data
        }
    }

    /// Create rarray1D vector of length `shape` filled with `value`
    ///
    /// # Examples 
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::fill(4., 3);
    /// println!("{}", v);
    /// ```
    fn fill(value: f64, shape: usize) -> Self {
        Rarray1D {
            shape: D1 { width: shape, height: 1},
            data: vec![value; shape]
        }
    }
}

impl RarrayCreate<(usize, usize), Vec<Vec<f64>>, f64> for Rarray1D {
    /// Create vector using Vec<Vec> 
    ///
    /// # Examples 
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::new(&vec![vec![0.], vec![0.], vec![0.]]);
    /// println!("{}", v);
    /// ```
    ///
    /// # Panics
    ///
    /// If rows aren't of same length
    fn new(data: &Vec<Vec<f64>>) -> Self {
        let row = data.len();
        let col = data[0].len();
        for i in 0..row {
            assert_eq!(col, data[i].len(), "All rows must be of same length");
        }

        Rarray1D {
            shape : D1 { width: 1, height: data.len() },
            data: data.clone().into_iter().flatten().collect()
        }
    }
    
    /// Create vector of shape `m x 1` filled with zeros
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    ///let v = Rarray1D::zeros((3, 1));
    /// println!("{}", v);
    /// ```
    ///
    /// # Panics 
    ///
    /// If (x, y) with x > 1 and y > 1 
    fn zeros(shape: (usize, usize)) -> Self {
        assert!(shape.0 == 1 || shape.1 == 1, "Cannot create 2D array using 1D array type");
        Rarray1D {
            shape: D1 { width: shape.1, height: shape.0},
            data: vec![0.; shape.0 * shape.1]
        }
    }
    
    /// Create vector of shape` filled with random numbers
    ///
    /// # Examples
    /// 
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::random((3, 1));
    /// println!("{}", v);
    /// ```
    /// # Panics 
    ///
    /// If (x, y) with x > 1 and y > 1 
    fn random(shape: (usize, usize)) -> Self {
        assert!(shape.0 == 1 || shape.1 == 1, "Cannot create 2D array using 1D array type");
        let mut data = Vec::<f64>::with_capacity(shape.0 * shape.1);
        for _ in 0..(shape.0 * shape.1) {
            data.push(rand::random::<f64>());
        }
        

        Rarray1D {
            shape: D1 { width: shape.1, height: shape.0 },
            data
        }
    }
    
    /// Create vector of shape `m x 1` filled with `value`
    ///
    /// # Examples 
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::fill(4., (3, 1));
    /// println!("{}", v);
    /// ```
    /// # Panics 
    ///
    /// If (x, y) with x > 1 and y > 1 
    fn fill(value: f64, shape: (usize, usize)) -> Self {
        assert!(shape.0 == 1 || shape.1 == 1, "Cannot create 2D array using 1D array type");
        Rarray1D {
            shape: D1 { width: shape.1, height: shape.0},
            data: vec![value; shape.0 * shape.1]
        }
    }
}
