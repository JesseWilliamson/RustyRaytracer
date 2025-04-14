use crate::rays::Ray;
use crate::vectors::{self, Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub fn face_normal(r: Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = vectors::dot(r.direction(), outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };
    (front_face, normal)
}

impl HitRecord {}
