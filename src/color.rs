use crate::vector::Vec3;
use crate::interval::Interval;
use std::io::Write;

pub type Color = Vec3;

impl Color {
    pub fn write_color<W: Write>(&self, out: &mut W) -> Result<(), std::io::Error> {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();
        r = Color::linear_to_gamma(r);
        g = Color::linear_to_gamma(g);
        b = Color::linear_to_gamma(b);
        let intensity = Interval::new(0.000, 0.999);
        let r_byte = (255.999 * intensity.clamp(r)) as i32;
        let g_byte = (255.999 * intensity.clamp(g)) as i32;
        let b_byte = (255.999 * intensity.clamp(b)) as i32;
        writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
    }

    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }
}
