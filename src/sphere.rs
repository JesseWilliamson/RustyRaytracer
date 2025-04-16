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
    fn hit(&self, r: rays::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<hittable::HitRecord> {
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

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        // Outward normal is the normal to the surface of the sphere pointing outwards,
        // normal is relative from the ray in case it's hitting from inside.
        let (front_face, normal) = hittable::face_normal(r, outward_normal);
        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
        })
    }
}
