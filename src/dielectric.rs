use crate::{color, material, ray, utils, vector};

pub struct Dielectric {
    // The ratio of the refractive index of the material to the refractive index of the enclosing media
    refractive_index: f64
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric { refractive_index }
    }

    pub fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &crate::hit_record::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray
    ) -> bool {
        *attenuation = color::Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = vector::unit_vector(r_in.direction());
        let cos_theta = vector::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || schlick_reflectance(cos_theta, ri) > utils::random_f64() {
            vector::reflect(&unit_direction, &rec.normal)
        } else {
            vector::refract(&unit_direction, &rec.normal, ri)
        };

        *scattered = ray::Ray::new(rec.p, direction);
        true
    }
}

fn schlick_reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl material::Material for Dielectric {
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
