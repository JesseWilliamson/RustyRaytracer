use crate::{
    hittable::{self, HitRecord},
    rays, vectors,
};

pub struct Sphere {
    center: vectors::Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: vectors::Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(self, r: rays::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hittable::HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = vectors::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        let hit_t = root;
        let hit_point = r.at(hit_t);
        let hit_normal = (hit_point - self.center) / self.radius;
        Some(HitRecord {
            p: hit_point,
            normal: hit_normal,
            t: hit_t,
        })
    }
}
