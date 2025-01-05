mod linalg;
mod stats;
mod test;
mod RArray;

use RArray::*;

fn main() {
    let Z = RArray2f64::zeros(2, 2);
    println!("{}", Z);
}

#[cfg(test)]
mod tests{
}