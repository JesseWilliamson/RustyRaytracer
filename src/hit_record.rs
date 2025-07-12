use crate::{material, point, ray, vector};
use std::rc::Rc;

pub struct HitRecord {
    /// Point of intersection between the ray and the surface
    pub p: point::Point3,
    pub normal: vector::Vec3,
    pub material: Rc<dyn material::Material>,
    /// Distance along the ray where the hit occurs (origin + t * direction)
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn p(&self) -> &point::Point3 {
        &self.p
    }
    pub fn normal(&self) -> &vector::Vec3 {
        &self.normal
    }
    pub fn material(&self) -> &Rc<dyn material::Material> {
        &self.material
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub fn face_normal(r: &ray::Ray, outward_normal: &vector::Vec3) -> (bool, vector::Vec3) {
    let front_face = vector::dot(r.direction(), *outward_normal) < 0.0;
    let normal = if front_face {
        *outward_normal
    } else {
        -(*outward_normal)
    };
    (front_face, normal)
}
