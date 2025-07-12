use crate::{vector, interval};
use std::io::Write;

pub type Color = vector::Vec3;
impl Color {
    const COLOR_MAX: f64 = 255.999;
    const COLOR_CLAMP_MIN: f64 = 0.0;
    const COLOR_CLAMP_MAX: f64 = 0.999;

    pub fn write_color<W: Write>(&self, out: &mut W) -> Result<(), std::io::Error> {
        let mut red_linear = self.x();
        let mut green_linear = self.y();
        let mut blue_linear = self.z();

        red_linear = Color::linear_to_gamma(red_linear);
        green_linear = Color::linear_to_gamma(green_linear);
        blue_linear = Color::linear_to_gamma(blue_linear);

        let intensity = interval::Interval::new(Self::COLOR_CLAMP_MIN, Self::COLOR_CLAMP_MAX);

        let r = (Self::COLOR_MAX * intensity.clamp(red_linear)) as i32;
        let g = (Self::COLOR_MAX * intensity.clamp(green_linear)) as i32;
        let b = (Self::COLOR_MAX * intensity.clamp(blue_linear)) as i32;

        writeln!(out, "{} {} {}", r, g, b)
    }
    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}
