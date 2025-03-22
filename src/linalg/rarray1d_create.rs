use std::cmp::max;
use crate::linalg::dimension::{D1, D2};
use crate::linalg::rarray::{Rarray1D, Rarray2D, RarrayCreate};

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

impl Rarray1D {
    /// Create matrix where given vector are the diagonal elements
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let a = Rarray1D::new(&vec![1., 1., 1.]);
    /// let res = Rarray1D::diag(&a);
    /// println!("{}", res);
    /// // >> Rarray2D([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    /// ```
    pub fn diag(&self) -> Rarray2D {
        let major = max(self.shape.height, self.shape.width);

        let mut result = Rarray2D {
            shape: D2 { height: major, width: major},
            data: vec![0.; major * major]
        };

        for i in 0..major {
            result.data[i * major + i] = self.data[i];
        }

        result
    }

    /// Create vector filled with values in given range with step size
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    ///
    /// let a = Rarray1D::range(0, 10, 2);
    /// println!("{}", a);
    /// // >> Rarray1D([0., 2., 4., 6., 8.])
    /// ```
    pub fn range(start: usize, stop: usize, step: usize) -> Rarray1D {
        let shape = (stop - start) / step;
        let mut result = Rarray1D {
            shape: D1 { height: 1, width: shape },
            data: Vec::<f64>::with_capacity(shape)
        };

        for i in (start..stop).step_by(step) {
            result.data.push(i as f64);
        }

        result
    }
}