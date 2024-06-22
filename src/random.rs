extern crate rand;

pub(crate) fn random_f64() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

pub(crate) fn random_f64_with_bounds(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_f64()
}
