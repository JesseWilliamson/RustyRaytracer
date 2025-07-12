use rand::Rng;
use indicatif::ProgressBar;

use crate::hittable;
use crate::hittable_list;
use crate::interval;
use crate::ray;
use crate::utils;
use crate::{vector, color, point};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    vertical_fov: f64,
    look_from: point::Point3,
    look_at: point::Point3,
    vup: vector::Vec3,
    u: vector::Vec3,
    v: vector::Vec3,
    w: vector::Vec3,
    samples_per_pixel: i32,
    max_depth: u32,
    center: point::Point3,
    pixel00_loc: point::Point3,
    pixel_delta_u: vector::Vec3,
    pixel_delta_v: vector::Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
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
        let focal_length = focus_dist; // Use focus_dist for depth of field
        let theta = utils::degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let center = look_from;

        let w = vector::unit_vector(look_from - look_at);
        let u = vector::unit_vector(vector::cross(vup, w));
        let v = vector::cross(w, u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center
            - w * focal_length
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let lens_radius = (defocus_angle.to_radians() / 2.0).tan() * focus_dist;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            vertical_fov,
            look_from,
            look_at,
            vup,
            u,
            v,
            w,
            samples_per_pixel,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            focus_dist,
            lens_radius,
        }
    }

    fn ray_color(r: &ray::Ray, world: &hittable_list::HittableList, depth: u32) -> color::Color {
        if depth == 0 {
            return color::Color::new(0.0, 0.0, 0.0);
        }
        let hit_record = hittable::Hittable::hit(world, r, &interval::Interval::new(0.001, f64::INFINITY));
        match hit_record {
            Some(rec) => {
                let mut scattered = ray::Ray::new(point::Point3::new(0.0, 0.0, 0.0), vector::Vec3::new(0.0, 0.0, 0.0));
                let mut attenuation = color::Color::new(0.0, 0.0, 0.0);
                if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    Camera::ray_color(&scattered, world, depth - 1) * attenuation
                } else {
                    color::Color::new(0.0, 0.0, 0.0)
                }
            },
            None => {
                let unit_direction = vector::unit_vector(r.direction());
                let a = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - a) * color::Color::new(1.0, 1.0, 1.0)
                    + color::Color::new(0.5, 0.7, 1.0) * a
            }
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> ray::Ray {
        // Random point in pixel
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // Depth of field: sample point on lens
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        let ray_origin = self.center + offset;
        let ray_direction = pixel_sample - ray_origin;

        ray::Ray::new(ray_origin, ray_direction)
    }

    /// Returns a random point in the unit disk (for lens sampling)
    fn random_in_unit_disk() -> vector::Vec3 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let p = vector::Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns a vector to a random point in the [0, 1) x [0, 1) unit square.
    /// See "Generating Sample Rays" in Ray Tracing in One Weekend.
    fn sample_square() -> vector::Vec3 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        vector::Vec3::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            0.0,
        )
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
