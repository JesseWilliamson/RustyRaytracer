use rand::Rng;

pub fn random_f64() -> f64 {
    rand::rng().random::<f64>()
}

pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
