use std::cmp::max;
use crate::linalg::dimension::{D1, D2};
use crate::linalg::numeric_trait::Numeric;
use crate::linalg::rarray::{Rarray1D, Rarray2D, RarrayCreate};


// Creation
impl<T> RarrayCreate<usize, Vec<T>, T> for Rarray1D<T> where
    T: Numeric
{
    /// Create rarray1D vector using Vec
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::<f64>::new(&vec![1., 1., 1.]);
    /// println!("{}", v);
    /// ```
    fn new(data: &Vec<T>) -> Self {
        Rarray1D {
            shape : D1 { width: data.len(), height: 1 },
            data: data.clone()
        }
    }

    //TODO: Constraint to numeric values
    /// Create rarra1d vector of length `shape` filled with zeros
    ///
    /// # Examples
    ///
    /// ```
    ///use rumpy::linalg::rarray::Rarray1D;
    ///use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::<f64>::zeros(3);
    /// println!("{}", v);
    /// ```
    fn zeros(shape: usize) -> Self {
        Rarray1D {
            shape: D1 { width: shape, height: 1},
            data: vec![T::default(); shape]
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
    fn fill(value: T, shape: usize) -> Self {
        Rarray1D {
            shape: D1 { width: shape, height: 1},
            data: vec![value; shape]
        }
    }
}

impl<T> RarrayCreate<(usize, usize), Vec<Vec<T>>, T> for Rarray1D<T> where
    T: Numeric
{
    /// Create vector using Vec<Vec>
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use crate::rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let v = Rarray1D::<f64>::new(&vec![vec![0.], vec![0.], vec![0.]]);
    /// println!("{}", v);
    /// ```
    ///
    /// # Panics
    ///
    /// If rows aren't of same length
    fn new(data: &Vec<Vec<T>>) -> Self {
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
    ///let v = Rarray1D::<f64>::zeros((3, 1));
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
            data: vec![T::default(); shape.0 * shape.1]
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
    /// let v = Rarray1D::<f64>::fill(4., (3, 1));
    /// println!("{}", v);
    /// ```
    /// # Panics
    ///
    /// If (x, y) with x > 1 and y > 1
    fn fill(value: T, shape: (usize, usize)) -> Self {
        assert!(shape.0 == 1 || shape.1 == 1, "Cannot create 2D array using 1D array type");
        Rarray1D {
            shape: D1 { width: shape.1, height: shape.0},
            data: vec![value; shape.0 * shape.1]
        }
    }
}

impl<T> Rarray1D<T> where
    T: Numeric
{
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
    pub fn diag(&self) -> Rarray2D<T> {
        let major = max(self.shape.height, self.shape.width);

        let mut result = Rarray2D {
            shape: D2 { height: major, width: major},
            data: vec![T::default(); major * major]
        };

        for i in 0..major {
            result.data[i * major + i] = self.data[i];
        }

        result
    }


    /// Create vector filled with values in given exclusive range `[start, stop)` with step size.
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    ///
    /// let a = Rarray1D::<f64>::range(0, 10, 2);
    /// println!("{}", a);
    /// // >> Rarray1D([0., 2., 4., 6., 8.])
    /// ```
    pub fn range(start: i32, stop: i32, step: i32) -> Rarray1D<T> {
        assert!(step != 0, "Invalid step. Step cannot be zero");

        if start < stop {
            assert!(step > 0, "Invalid step for start < stop");
        }
        else if start > stop {
            assert!(step < 0, "Invalid step for start > stop");
        }

        let shape = ((stop - start) / step).abs() as usize;
        
        let mut result = Rarray1D {
            shape: D1 { height: 1, width: shape as usize },
            data: Vec::<T>::with_capacity(shape as usize)
        };

        for i in 0..shape {
            let val = start + (i as i32) * step;
            result.data.push(T::from(val));
        }

        result
    }
}
