pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hit_record::HitRecord,
        attenuation: &mut crate::color::Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool;
}
