use crate::{rays, vectors};

pub struct HitRecord {
    pub p: vectors::Point3,
    pub normal: vectors::Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub fn face_normal(r: &rays::Ray, outward_normal: &vectors::Vec3) -> (bool, vectors::Vec3) {
    let front_face = vectors::dot(r.direction(), *outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
    } else {
        -(*outward_normal)
    };
    (front_face, normal)
}
