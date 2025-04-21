use std::f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }

    pub fn empty() -> Interval {
        Interval {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min: NEG_INFINITY,
            max: INFINITY,
        }
    }
}
