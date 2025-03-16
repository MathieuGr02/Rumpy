use std::ops::{Mul, MulAssign, Index, IndexMut};

use super::rarray::{Rarray2D, D2};

impl Index<[usize; 2]> for Rarray2D {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        assert!((index[0] < self.shape[0]) && index[1] < self.shape[1], "Index out of bounds");
        &self.data[index[0] * self.shape[0] + index[1]]
    }
}

impl IndexMut<[usize; 2]> for Rarray2D {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        assert!(self.shape[0] < index[0] && self.shape[1] < index[1], "Index out of bounds");
        &mut self.data[index[0] * self.shape[0] + index[1]]
    }
}

impl Mul<&Rarray2D> for &Rarray2D {
    type Output = Rarray2D;

    fn mul(self, rhs: &Rarray2D) -> Self::Output {
        assert_eq!(self.shape[1], rhs.shape[0], "Incompatible array sizes");

        let mut rarray_mut: Rarray2D = Rarray2D {
            shape: D2 { height: self.shape[0], width: rhs.shape[1]},
            data: vec![0.; self.shape[0] * rhs.shape[1]]
        };

        for i in 0..self.shape[0] {
            for j in 0..rhs.shape[1] {
                let mut sum: f64 = 0.;
                for k in 0..self.shape[1] {
                    sum += self.data[i * self.shape[1] + k] * rhs.data[j * self.shape[0] + k];
                }
                rarray_mut.data[i * self.shape[1] + j * self.shape[0]] = sum;
            }
        }

        rarray_mut
    }
}

impl MulAssign<&Rarray2D> for Rarray2D {
    fn mul_assign(&mut self, rhs: &Rarray2D) {
        assert_eq!(self.shape[1], rhs.shape[0], "Incompatible array sizes");

        let mut data = vec![0.; self.shape[0] * rhs.shape[1]];

        for i in 0..self.shape[0] {
            for j in 0..rhs.shape[1] {
                let mut sum: f64 = 0.;
                for k in 0..self.shape[1] {
                    sum += self.data[i * self.shape[1] + k] * rhs.data[j * self.shape[0] + k];
                }
                data[i * self.shape[1] + j * self.shape[0]] = sum;
            }
        }
        
        self.data = data;
    }
}
