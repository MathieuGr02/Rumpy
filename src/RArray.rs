use std::fmt;

#[derive(Debug)]
pub struct RArray2f64{
    pub array: Vec<Vec<f64>>,
    size: (usize, usize),
}

impl fmt::Display for RArray2f64{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RArray2f64({:?})", self.array)
    }
}

/*
impl Index<Vec<f64>> for Vec<f64>{
    type Output = usize;

    fn index(&self, ind: usize) -> &Self::Output{
        self.array.get(ind)
    }
}
 */

impl RArray2f64{
    pub fn new(array) -> RArray2f64 {
        let shape: usize = array.len();
        let mut new_array = Self::zeros(shape, shape);
        new_array
    }

    pub fn zeros(col: usize, row: usize) -> RArray2f64{
        RArray2f64{
            array: vec![vec![0.0; col]; row],
            size: (col, row)
        }
    }

    pub fn ones(shape: usize) -> RArray2f64{
        let mut array: RArray2f64 = Self::zeros(shape, shape);
        for i in 0..shape{
            //array[i][i] += 1;
        }
        array
    }
}

