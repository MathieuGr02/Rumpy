use super::rarray::{self, Rarray};
use crate::linalg;

// TODO: Assert dimensions

pub fn dot(a: Rarray, b: Rarray) -> Rarray {
    let size: usize = 0;

    let mut rarray = linalg::zeros(1, size);
    for i in 0..size {
        rarray.data[i] = a.data[i] * b.data[i]; 
    } 

    rarray
}

pub fn transpose(matrix: Rarray) -> Rarray {
    let mut rarray = linalg::zeros(matrix.width, matrix.height);

    for i in 0..matrix.height {
        for j in 0..matrix.width {
            rarray[[i, j]] = matrix[[j, i]];
            rarray[[j, i]] = matrix[[i, j]];
        }
    }

    rarray
}

pub fn outer(a: Rarray, b: Rarray) -> Rarray {
    let mut rarray = linalg::zeros(a.height, b.width);
    
    let row: usize = 0;
    let col: usize = 0;

    for i in 0..row {
        for j in 0..col {
            //rarray[[i, j]] = a[i] * b[j]; 
        }
    }

    rarray
}
