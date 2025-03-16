use std::ops::{Index, IndexMut, Mul, MulAssign};

use super::rarray::Rarray1D;

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
        Rarray1D {
            shape: self.shape,
            data: self.data.iter().clone().map(|x| rhs * x).collect()
        }
    }
}

impl MulAssign<f64> for Rarray1D {
    fn mul_assign(&mut self, rhs: f64) {
        self.data = self.data.iter().map(|x| rhs * x).collect();
    }
}
