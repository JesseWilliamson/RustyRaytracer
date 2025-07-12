use crate::{color, point, vector};
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

    // Returns the position along the ray at parameter t (ray equation: origin + t*direction)
    pub fn at(&self, t: f64) -> point::Point3 {
        self.orig + t * self.dir
    }

    pub fn color(&self) -> color::Color {
        // Background gradient constants (must be local variables, not const)
        let background_top_color = color::Color::new(0.5, 0.7, 1.0);
        let background_bottom_color = color::Color::new(1.0, 1.0, 1.0);
        let blend_factor_scale: f64 = 0.5;
        let blend_factor_offset: f64 = 1.0;
        let unit_direction = vector::unit_vector(self.direction());
        let blend_factor = blend_factor_scale * (unit_direction.y() + blend_factor_offset);
        background_bottom_color * (1.0 - blend_factor) + background_top_color * blend_factor
    }
}
