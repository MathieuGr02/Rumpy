use crate::linalg::numeric_trait::Numeric;
use super::rarray::{Rarray1D, Rarray2D, D1, D2};

impl<T> Rarray1D<T> where 
    T: Numeric
{
    /// Tranpose 1D matrix
    pub fn transpose(&self) -> Self {
        Rarray1D {
            shape: D1 { width: self.shape.height, height: self.shape.width },
            data: self.data.clone()
        }
    }

    /// Tranpose 1D matrix inplace
    pub fn mut_transpose(&mut self){
        self.shape = D1 { width: self.shape.height, height: self.shape.width };
    } 
}

impl<T> Rarray1D<T> where
    T: Numeric
{
    /// Calculate dot product of two vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let a = Rarray1D::new(&vec![1., 1., 1.]);
    /// let b = Rarray1D::new(&vec![1., 1., 1.]);
    /// let res = Rarray1D::dot(&a, &b);
    /// println!("{}", res);
    /// // >> 3.
    /// ```
    ///
    /// # Panics
    ///
    /// If vectors aren't of same length
    pub fn dot(a: &Self, b: &Self) -> T {
        assert_eq!(a.data.len(), b.data.len(), "Vectors not of same size");
        let mut result: T = T::default();
        for i in 0..a.data.len() {
            result += a.data[i] * b.data[i];
        }
        result
    }

    /// Calculate outer product of two vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use rumpy::linalg::rarray::Rarray1D;
    /// use rumpy::linalg::rarray::RarrayCreate;
    ///
    /// let a = Rarray1D::<f64>::new(&vec![1., 1., 1.]);
    /// let b = Rarray1D::<f64>::new(&vec![1., 1., 1.]);
    /// let res = Rarray1D::outer(&a, &b);
    /// println!("{}", res);
    /// // >> Rarray2D([[1., 1., 1.], [1., 1., 1.], [1., 1., 1.]])
    /// ```
    pub fn outer(a: &Self, b: &Self) -> Rarray2D<T> {
        let mut row = a.shape.height;
        if a.shape.width > row {
            row = a.shape.width;
        }

        let mut col = b.shape.height;
        if b.shape.width > col {
            col = b.shape.width;
        }

        let mut result: Rarray2D<T> = Rarray2D {
            shape: D2 { height: row, width: col },
            data: vec![T::default(); row * col]
        }; 

        for i in 0..row {
            for j in 0..col {
                result.data[i * row + j] = a[i] * b[j];
            }
        }

        result
    }

    /*
    /// Sum values of array
    pub fn sum(&self) -> T {
        self.data.iter().sum()
    }
   
    // TODO: yet to implement functionality
    pub fn unique(&self) -> Rarray1D {
        
    }

    pub fn unique_index(&self) -> Rarray1D {

    }

    pub fn diag(&self) -> Rarray2D {
    
    }
    */
}

