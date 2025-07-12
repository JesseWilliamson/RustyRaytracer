use crate::{color, hit_record, material, rays, vector::random_in_unit_sphere};

pub struct Lambertian {
    albedo: color::Color,
}

impl Lambertian {
    pub fn new(albedo: color::Color) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn scatter(&self, ray: &rays::Ray, hit_record: &hit_record::HitRecord, attenuation: &mut color::Color, scattered: &mut rays::Ray) -> bool {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        
        *scattered = rays::Ray::new(hit_record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

// Implement the Material trait for Lambertian
impl material::Material for Lambertian {
    fn scatter(
        &self,
        r_in: &rays::Ray,
        rec: &hit_record::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut rays::Ray
    ) -> bool {
        self.scatter(r_in, rec, attenuation, scattered)
    }
}