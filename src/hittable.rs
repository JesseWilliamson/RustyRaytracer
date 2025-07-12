use crate::{hit_record, interval, ray};

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_t: &interval::Interval) -> Option<hit_record::HitRecord>;
}
