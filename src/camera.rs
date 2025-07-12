use crate::{color, point, vector, utils, hittable_list, hittable, interval, ray};
use indicatif::ProgressBar;

#[derive(Debug, Clone)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: u32,
    u: vector::Vec3,
    v: vector::Vec3,
    center: point::Point3,
    pixel00_loc: point::Point3,
    pixel_delta_u: vector::Vec3,
    pixel_delta_v: vector::Vec3,
    lens_radius: f64,
}

impl Camera {
    // Set up the camera coordinate system and image plane
    fn new(
        image_width: i32,
        aspect_ratio: f64,
        vertical_fov: f64,
        look_from: point::Point3,
        look_at: point::Point3,
        vup: vector::Vec3,
        samples_per_pixel: i32,
        max_depth: u32,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let focal_length = focus_dist;
        let theta = utils::degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;

        let center = look_from;
        // Camera basis vectors: w = view direction, u = right, v = up
        let w = vector::unit_vector(look_from - look_at);
        let u = vector::unit_vector(vector::cross(vup, w));
        let v = vector::cross(w, u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = center - w * focal_length - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Lens radius for defocus blur
        let lens_radius = (defocus_angle.to_radians() / 2.0).tan() * focus_dist;

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            u,
            v,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            lens_radius,
        }
    }

    fn ray_color(
        ray: &ray::Ray,
        world: &hittable_list::HittableList,
        remaining_depth: u32,
    ) -> color::Color {
        // Background gradient constants (must be local variables, not const)
        let background_top_color = color::Color::new(0.5, 0.7, 1.0);
        let background_bottom_color = color::Color::new(1.0, 1.0, 1.0);
        let blend_factor_scale: f64 = 0.5;
        let blend_factor_offset: f64 = 1.0;
        if remaining_depth == 0 {
            return color::Color::new(0.0, 0.0, 0.0);
        }
        let hit_record =
            hittable::Hittable::hit(world, ray, &interval::Interval::new(0.001, f64::INFINITY));
        match hit_record {
            Some(record) => {
                let mut scattered_ray = ray::Ray::new(
                    point::Point3::new(0.0, 0.0, 0.0),
                    vector::Vec3::new(0.0, 0.0, 0.0),
                );
                let mut attenuation = color::Color::new(0.0, 0.0, 0.0);
                if record
                    .material()
                    .scatter(ray, &record, &mut attenuation, &mut scattered_ray)
                {
                    Camera::ray_color(&scattered_ray, world, remaining_depth - 1) * attenuation
                } else {
                    color::Color::new(0.0, 0.0, 0.0)
                }
            }
            None => {
                let unit_direction = vector::unit_vector(ray.direction());
                let blend_factor = blend_factor_scale * (unit_direction.y() + blend_factor_offset);
                background_bottom_color * (1.0 - blend_factor) + background_top_color * blend_factor
            }
        }
    }

    // Generate a ray through pixel (i, j), with lens sampling for depth of field
    fn get_ray(&self, i: i32, j: i32) -> ray::Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // Defocus blur: sample point on lens
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let ray_origin = self.center + offset;
        let ray_direction = pixel_sample - ray_origin;
        ray::Ray::new(ray_origin, ray_direction)
    }

    fn random_in_unit_disk() -> vector::Vec3 {
        use rand::Rng;
        let mut rng = rand::rng();
        loop {
            let p = vector::Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    fn sample_square() -> vector::Vec3 {
        use rand::Rng;
        let mut rng = rand::rng();
        vector::Vec3::new(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0), 0.0)
    }

    pub fn render<W: std::io::Write>(
        &self,
        world: &hittable_list::HittableList,
        out: &mut W,
    ) -> Result<(), std::io::Error> {
        out.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())?;
        let bar = ProgressBar::new(self.image_height as u64);
        let max_depth = self.max_depth;
        for j in 0..self.image_height {
            bar.inc(1);
            for i in 0..self.image_width {
                let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world, max_depth);
                }
                pixel_color = pixel_color / self.samples_per_pixel as f64;
                pixel_color.write_color(out)?;
            }
        }
        bar.finish();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: i32,
    vertical_fov: f64,
    look_from: point::Point3,
    look_at: point::Point3,
    vup: vector::Vec3,
    samples_per_pixel: i32,
    max_depth: u32,
    defocus_angle: f64,
    focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            vertical_fov: 20.0,
            look_from: point::Point3::new(0.0, 0.0, 0.0),
            look_at: point::Point3::new(0.0, 0.0, -1.0),
            vup: vector::Vec3::new(0.0, 1.0, 0.0),
            samples_per_pixel: 10,
            max_depth: 10,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

impl CameraBuilder {
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }
    pub fn image_width(mut self, image_width: i32) -> Self {
        self.image_width = image_width;
        self
    }
    pub fn vertical_fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }
    pub fn look_from(mut self, look_from: point::Point3) -> Self {
        self.look_from = look_from;
        self
    }
    pub fn look_at(mut self, look_at: point::Point3) -> Self {
        self.look_at = look_at;
        self
    }
    pub fn vup(mut self, vup: vector::Vec3) -> Self {
        self.vup = vup;
        self
    }
    pub fn samples_per_pixel(mut self, spp: i32) -> Self {
        self.samples_per_pixel = spp;
        self
    }
    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }
    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }
    pub fn focus_dist(mut self, dist: f64) -> Self {
        self.focus_dist = dist;
        self
    }
    pub fn build(self) -> Camera {
        Camera::new(
            self.image_width,
            self.aspect_ratio,
            self.vertical_fov,
            self.look_from,
            self.look_at,
            self.vup,
            self.samples_per_pixel,
            self.max_depth,
            self.defocus_angle,
            self.focus_dist,
        )
    }
}
