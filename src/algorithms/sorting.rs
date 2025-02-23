pub enum SortType{
    MERGESORT,
    QUICKSORT
}
/*
use crate::algorithms::sorting::SortType::{MERGESORT, QUICKSORT};
use crate::RArray::RArray2f64;

pub fn sort(array: RArray2f64, mut sort: Option<SortType>){
    let sorting_type = sort.unwrap_or(QUICKSORT);

    match sorting_type {
        QUICKSORT => mergesort(array),
        MERGESORT => quicksort(array)
    }
}

fn mergesort(array: RArray2f64){

}



fn quicksort(array: RArray2f64){
    
}
*/
