use crate::{hit_record, hittable, interval, material, ray, vector};
use std::rc;

pub struct Sphere {
    center: vector::Vec3,
    radius: f64,
    material: rc::Rc<dyn material::Material>,
}

impl Sphere {
    pub fn new(
        center: vector::Vec3,
        radius: f64,
        material: rc::Rc<dyn material::Material>,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

// Parameters for creating a sphere (for scene construction)
pub struct SphereSpec {
    pub center: vector::Vec3,
    pub radius: f64,
    pub material: rc::Rc<dyn material::Material>,
}

// Helper to add a Sphere from a SphereSpec to a HittableList
pub fn add_sphere(world: &mut crate::hittable_list::HittableList, spec: &SphereSpec) {
    world.add(
        rc::Rc::new(Sphere::new(spec.center, spec.radius, spec.material.clone()))
            as rc::Rc<dyn crate::hittable::Hittable>,
    );
}

impl hittable::Hittable for Sphere {
    // Ray-sphere intersection using the quadratic equation
    fn hit(&self, r: &ray::Ray, ray_t: &interval::Interval) -> Option<hit_record::HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = vector::dot(r.direction(), oc);
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
        let (front_face, normal) = hit_record::face_normal(r, &outward_normal);

        Some(hit_record::HitRecord {
            p,
            normal,
            t,
            front_face,
            material: self.material.clone(),
        })
    }
}
