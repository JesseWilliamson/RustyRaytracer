use crate::vector::Vec3;
use crate::interval::Interval;
use std::io::Write;

pub type Color = Vec3;

impl Color {
    pub fn write_color<W: Write>(&self, out: &mut W) -> Result<(), std::io::Error> {
        let r = self.x();
        let g = self.y();
        let b = self.z();
        let intensity = Interval::new(0.000, 0.999);
        let r_byte = (255.999 * intensity.clamp(r)) as i32;
        let g_byte = (255.999 * intensity.clamp(g)) as i32;
        let b_byte = (255.999 * intensity.clamp(b)) as i32;
        writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
    }
}
