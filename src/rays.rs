use crate::{vector, color, point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    orig: point::Point3,
    dir: vector::Vec3,
}

impl Ray {
    pub fn new(orig: point::Point3, dir: vector::Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> point::Point3 {
        self.orig
    }

    pub fn direction(&self) -> vector::Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> point::Point3 {
        self.orig + t * self.dir
    }

    pub fn color(&self) -> color::Color {
        let unit_direction = vector::unit_vector(self.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        color::Color::new(1.0, 1.0, 1.0) * (1.0 - a) + color::Color::new(0.5, 0.7, 1.0) * a
    }
}
