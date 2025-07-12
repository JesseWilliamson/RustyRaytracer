use crate::hit_record;
use crate::hittable;
use crate::interval;
use crate::rays;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: &rays::Ray, ray_t: &interval::Interval) -> Option<hit_record::HitRecord> {
        let mut closest_so_far = ray_t.max();
        let mut hit_anything = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, &interval::Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
