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

// Creation

impl RarrayCreate<usize, f64> for Rarray1D {
    fn new(data: &Vec<f64>) -> Self {
        Rarray1D {
            shape : D1 { width: data.len() as usize, height: 1 },
            data: data.clone()
        }
    }

    fn zeros(shape: usize) -> Self {
        Rarray1D {
            shape: D1 { width: shape, height: 1},
            data: vec![0.; shape]
        }
    }

    fn ones(shape: usize) -> Self {
        Rarray1D {
            shape: D1 { width: shape, height: 1},
            data: vec![1.; shape]
        }
    }

    fn random(shape: usize) -> Self {
        todo!()
    }

    fn fill(value: f64, shape: usize) -> Self {
        Rarray1D {
            shape: D1 { width: shape, height: 1},
            data: vec![value; shape]
        }
    }
}

impl RarrayCreate<(usize, usize), f64> for Rarray1D {
    fn new(data: &Vec<f64>) -> Self {
        Rarray1D {
            shape : D1 { width: data.len(), height: 1 },
            data: data.clone()
        }
    }

    fn zeros(shape: (usize, usize)) -> Self {
        assert!(shape.0 == 1 || shape.1 == 1, "Cannot create 2D array using 1D array type");
        Rarray1D {
            shape: D1 { width: shape.1, height: shape.0},
            data: vec![0.; shape.0 * shape.1]
        }
    }

    fn ones(shape: (usize, usize)) -> Self {
        assert!(shape.0 == 1 || shape.1 == 1, "Cannot create 2D array using 1D array type");
        Rarray1D {
            shape: D1 { width: shape.1, height: shape.0},
            data: vec![1.; shape.0 * shape.1]
        }
    }

    fn random(shape: (usize, usize)) -> Self {
        todo!()
    }

    fn fill(value: f64, shape: (usize, usize)) -> Self {
        assert!(shape.0 == 1 || shape.1 == 1, "Cannot create 2D array using 1D array type");
        Rarray1D {
            shape: D1 { width: shape.1, height: shape.0},
            data: vec![value; shape.0 * shape.1]
        }
    }
}
