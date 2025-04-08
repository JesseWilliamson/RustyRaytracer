use std::{io::Write, ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Color {
    pub fn write_color<W: Write>(self, out: &mut W) -> Result<(), std::io::Error> {
        let r = self.x;
        let g = self.y;
        let b = self.z;

        let r_byte = (255.999 * r) as i32;
        let g_byte = (255.999 * g) as i32;
        let b_byte = (255.999 * b) as i32;

        write!(out, "{} {} {}\n", r_byte, g_byte, b_byte)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.y * _rhs,
            y: self.y * _rhs,
            z: self.y * _rhs,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

pub struct ray {
    orig: Point3,
    dir: Vec3,
}

impl ray {
    pub fn orig(&self) -> &Point3 {
        &self.orig
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + self.dir * t;
    }
}
