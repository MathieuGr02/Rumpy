use crate::linalg;
use core::fmt;
use std::{ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}, usize};

/*
impl fmt::Display for Rarray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted_string: String = Default::default();

        for i in 0..self.height {
            let mut row_string: String = Default::default();
           
            for j in 0..self.width {
                row_string.push_str(&self.data[self.height * i + j].to_string());
                row_string.push_str(", ");
            }          
            
            row_string.pop();
            row_string.pop();
   
            formatted_string.push_str("[");
            formatted_string.push_str(&row_string);
            formatted_string.push_str("],\n");
        }

        formatted_string.pop();
        formatted_string.pop();
        
        write!(f, "[{}]", &formatted_string)
    }
}

impl Rarray {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let height: usize = matrix.len();
        let width: usize = matrix[0].len();

        let mut data = vec![];  
        for row_matrix in matrix.iter() {
            let col_len = row_matrix.len(); 
            assert_eq!(col_len, width, "All columns must be of same size");
            
            for element in row_matrix.iter() {
                data.push(*element);
            }
        }
        
        Rarray {
            height,
            width,
            data
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.height, self.width)
    }
}

impl Mul for Rarray {
    type Output = Self;

    fn mul(self, _other: Self) -> Self::Output {
        assert_eq!(self.width, _other.height, "Matrix of type [{}, {}] incompatible with [{}, {}]", self.height, self.width, _other.height, _other.width);
        
        let mut mul_rarray = linalg::zeros(self.height, _other.width);

        for i in 0..self.height {
            for j in 0.._other.width {
                let mut row_sum: f64 = 0.;
                for k in 0..self.width {
                   row_sum += self.data[i * self.height + k] * _other.data[k * _other.width + j]; 
                }
                mul_rarray.data[i * self.height + j] = row_sum;
            } 
        }

        mul_rarray
    }
}

impl Mul<f64> for Rarray {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        let mut mul_rarray = base::zeros(self.height, self.width);

        for i in 0..(self.width * self.height) {
            mul_rarray.data[i] = factor * self.data[i];
        }

        mul_rarray
    }
}

impl MulAssign for Rarray {
    fn mul_assign(&mut self, _other: Self) {
        assert_eq!(self.width, _other.height, "Matrix of type [{}, {}] incompatible with [{}, {}]", self.height, self.width, _other.height, _other.width);
        
        let mut mul_rarray = linalg::zeros(self.height, _other.width);

        for i in 0..self.height {
            for j in 0.._other.width {
                let mut row_sum: f64 = 0.;
                for k in 0..self.width {
                   row_sum += self.data[i * self.height + k] * _other.data[k * _other.width + j]; 
                }
                mul_rarray.data[i * self.height + j] = row_sum;
            } 
        }
    }
}

impl MulAssign<f64> for Rarray {
    fn mul_assign(&mut self, factor: f64) {
        let mut mul_rarray = linalg::zeros(self.height, self.width);

        for i in 0..(self.width * self.height) {
            mul_rarray.data[i] = factor * self.data[i]; 
        }
    }
}

*/
