use crate::{hit_record, interval, rays};

pub trait Hittable {
    fn hit(&self, r: &rays::Ray, ray_t: &interval::Interval) -> Option<hit_record::HitRecord>;
}
