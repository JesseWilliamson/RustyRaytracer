use std::rc::Rc;
use crate::{rays, vector, point, material};

pub struct HitRecord {
    pub p: point::Point3,
    pub normal: vector::Vec3,
    pub material: Rc<dyn material::Material>,
    pub t: f64,
    pub front_face: bool,
}

pub fn face_normal(r: &rays::Ray, outward_normal: &vector::Vec3) -> (bool, vector::Vec3) {
    let front_face = vector::dot(r.direction(), *outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
    } else {
        -(*outward_normal)
    };
    (front_face, normal)
}
