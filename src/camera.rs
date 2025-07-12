use rand::Rng;
use indicatif::ProgressBar;

use crate::hittable;
use crate::hittable_list;
use crate::interval;
use crate::rays;
use crate::vectors;

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    center: vectors::Point3,
    pixel00_loc: vectors::Point3,
    pixel_delta_u: vectors::Vec3,
    pixel_delta_v: vectors::Vec3,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let center = vectors::Point3::new(0.0, 0.0, 0.0);

        let viewport_u = vectors::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vectors::Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center
            - vectors::Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(r: &rays::Ray, world: &hittable_list::HittableList, depth: u32) -> vectors::Color {
        if depth == 0 {
            return vectors::Color::new(0.0, 0.0, 0.0);
        }
        let hit_record = hittable::Hittable::hit(world, r, &interval::Interval::new(0.0, f64::INFINITY));
        match hit_record {
            Some(rec) => {
                let direction = vectors::random_on_hemisphere(rec.normal);
                0.5 * Camera::ray_color(&rays::Ray::new(rec.p, direction), world, depth - 1)
            },
            None => {
                let unit_direction = vectors::unit_vector(r.direction());
                let a = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - a) * vectors::Color::new(1.0, 1.0, 1.0)
                    + vectors::Color::new(0.5, 0.7, 1.0) * a
            }
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> rays::Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        rays::Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> vectors::Vec3 {
        // Returns a vector to a random point in the [-.5, -.5], [+.5, +.5] unit square.
        let mut rng = rand::rng();

        vectors::Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
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
        let max_depth = 50;
        for j in 0..self.image_height {
            bar.inc(1);
            for i in 0..self.image_width {
                let mut pixel_color = vectors::Color::new(0.0, 0.0, 0.0);
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
