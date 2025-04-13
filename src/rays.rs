use crate::{
    hittable::Hittable,
    sphere,
    vectors::{self, dot, unit_vector, Color, Point3, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

pub fn ray_color(r: Ray) -> Color {
    let sphere_center = vectors::Point3::new(0.0, 0.0, -1.0);
    let sphere = sphere::Sphere::new(sphere_center, 0.5);
    let hit_record = sphere.hit(r, -1000.0, 1000.0);
    match hit_record {
        Some(rec) => {
            let n = unit_vector(r.at(rec.t) - Vec3::new(0.0, 0.0, -1.0));
            0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
        }
        None => {
            let unit_direction = unit_vector(r.direction());
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + Color::new(0.5, 0.7, 1.0) * a
        }
    }
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn color(self) -> Color {
        let unit_direction = unit_vector(self.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
