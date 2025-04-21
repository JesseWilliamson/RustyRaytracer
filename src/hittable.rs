use std::rc::Rc;

use crate::interval::Interval;
use crate::rays::Ray;
use crate::vectors::{self, Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max();
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
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
