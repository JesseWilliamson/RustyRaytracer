use crate::{color, material, vector};

pub struct Metal {
    albedo: color::Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: color::Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }

    pub fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hit_record::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        let reflected = crate::vector::reflect(&r_in.direction(), rec.normal())
            + self.fuzz * vector::random_unit_vector();
        *scattered = crate::ray::Ray::new(*rec.p(), reflected);
        *attenuation = self.albedo;
        vector::dot(scattered.direction(), *rec.normal()) > 0.0
    }
}

impl material::Material for Metal {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hit_record::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        self.scatter(r_in, rec, attenuation, scattered)
    }
}
