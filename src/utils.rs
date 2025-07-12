use rand::Rng;

/// Returns a random real in [0,1)
pub fn random_f64() -> f64 {
    rand::thread_rng().gen::<f64>()
}

/// Returns a random real in [min,max)
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
