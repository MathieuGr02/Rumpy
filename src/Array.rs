use std::fmt;
use std::fmt::Debug;
use std::ops::Sub;
use std::ops::Add;

trait RArrayTrait<T>{
    fn size(&self) -> ();
}

#[derive(Debug, Clone)]
pub struct RArray2<T>{
    pub array: Vec<Vec<T>>,
    size: (usize, usize),
}

#[derive(Debug, Clone)]
struct RArray1<T>{
    pub array: Vec<T>,
    size: (usize),
}

impl<T: Default + Clone + Debug> RArray2<T>{
    pub fn zeros(row: usize, col: usize) -> RArray2<T> {
        let mut arr: Vec<Vec<T>> = vec![vec![T::default(); col]; row];
        RArray2 {
            array: arr,
            size: (row, col)
        }
    }

    pub fn default(row: usize, col: usize) -> RArray2<T>{
        let mut arr: Vec<Vec<T>> = vec![vec![T::default(); col]; row];
        RArray2 {
            array: arr,
            size: (row, col)
        }
    }

    pub fn new(arr: &[T]){
        println!("{:?}", arr);
    }

     pub fn shape(&self) -> (usize, usize){ self.size }
}
impl<T> Add for RArray2<T>{
    type Output = RArray2<T>;

    fn add(self, other: Self) -> Self::Output {
        let self_shape = self.size;
        let other_shape = self.size;

        assert_eq!(self_shape.0, other_shape.0);
        assert_eq!(self_shape.1, other_shape.1);
        /*
        let rarray2_sum = RArray2::<T>::default(self_shape.0, self_shape.1);

        for i in 0..self_shape.0{
            for j in 0..self_shape.1{
                rarray2_sum[i][j] = *self.array[i][j] + *other.array[i][j];
            }
        }
        rarray2_sum
         */
    }
}

impl<T> Sub for RArray2<T>{
    type Output = RArray2<T>;

    fn sub(self, other: Self) -> Self::Output {
        let self_shape = self.shape();
        let other_shape = self.shape();

        assert_eq!(self_shape[0], other_shape[0]);
        assert_eq!(self_shape[1], other_shape[1]);

        let rarray2_sum = RArray2::<T>::zeros(self_shape.0, self_shape.1);

        for i in 0..self_shape[0]{
            for j in 0..self_shape[1]{
                rarray2_sum[i][j] = *self.array[i][j] + *other.array[i][j];
            }
        }
        rarray2_sum
    }
}

impl<T: fmt::Debug> fmt::Display for RArray2<T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RArray2({:?})", self.array)
    }
}