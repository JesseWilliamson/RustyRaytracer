use crate::{hit_record, hittable, interval, rays, vectors};

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
    fn hit(&self, r: &rays::Ray, ray_t: &interval::Interval) -> Option<hit_record::HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = vectors::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + discriminant_sqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        // Outward normal is the normal to the surface of the sphere pointing outwards,
        // normal is relative from the ray in case it's hitting from inside.
        let (front_face, normal) = hit_record::face_normal(r, &outward_normal);
        Some(hit_record::HitRecord {
            p,
            normal,
            t,
            front_face,
        })
    }
}
