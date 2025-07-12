use crate::{color, material};

pub struct Metal {
    albedo: color::Color,
}

impl Metal {
    pub fn new(albedo: color::Color) -> Metal {
        Metal { albedo }
    }

    pub fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hit_record::HitRecord, attenuation: &mut color::Color, scattered: &mut crate::ray::Ray) -> bool {
        let reflected = crate::vector::reflect(&r_in.direction(), &rec.normal);
        *scattered = crate::ray::Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}

impl material::Material for Metal {
    fn scatter(
        &self, 
        r_in: &crate::ray::Ray, 
        rec: &crate::hit_record::HitRecord, 
        attenuation: &mut color::Color, 
        scattered: &mut crate::ray::Ray
    ) -> bool {
        self.scatter(r_in, rec, attenuation, scattered)
    }
}