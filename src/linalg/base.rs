use super::rarray::Rarray;
//use rand::distributions::Uniform;

pub fn zeros(height: usize, width: usize) -> Rarray {
    Rarray {
        width,
        height,
        data: vec![0.; width * height]
    }
}

pub fn ones(size: usize) -> Rarray {
    let mut data = vec![0.; size * size];

    let mut positions = vec![0; size];
    for offset in 0..size {
        positions[offset] = size * offset;
    }

    for pos in positions {
        data[pos] = 1.;
    }

    Rarray {
        width: size,
        height: size,
        data
    }
}

pub fn random(height: usize, width: usize) -> Rarray {
    let mut data = vec![0.; height * width]; 

    for i in 0..(height * width) {
        data[i] = 0.;//Uniform::from(0..1);
    }

    Rarray {
        width,
        height,
        data
    }
}

pub fn vstack() {} 

pub fn hstack() {}

pub fn reshape() {}

pub fn flatten() {}

pub fn  
