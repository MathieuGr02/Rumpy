use std::ops::{Mul, MulAssign, Index, IndexMut};

use super::rarray::{Rarray2D, RarrayMul, D2};

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
        //self = Rarray2D::mul(self, rhs);
    }
}
