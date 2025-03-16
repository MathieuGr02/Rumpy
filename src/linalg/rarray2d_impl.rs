use std::ops::{Mul, MulAssign};

use super::rarray::{Rarray2D, D2};

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
                //rarray_mut[i * self.shape[1] + j * self.shape[0]] = sum;
            }
        }

        rarray_mut
    }
}
