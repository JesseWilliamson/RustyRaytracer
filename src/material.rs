pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &crate::rays::Ray,
        rec: &crate::hit_record::HitRecord,
        attenuation: &mut crate::color::Color,
        scattered: &mut crate::rays::Ray
    ) -> bool;
}