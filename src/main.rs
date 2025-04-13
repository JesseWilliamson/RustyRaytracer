use indicatif::ProgressBar;
use raytracing_in_a_weekend_rust::{rays, sphere, vectors};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", &args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    println!("{}", file_name);
    let mut file = File::create(file_name)?;

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = std::cmp::max(1, (image_width as f64 / aspect_ratio) as i32);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = vectors::Point3::new(0.0, 0.0, 0.0);

    let viewport_u = vectors::Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vectors::Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
        - vectors::Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let bar = ProgressBar::new(image_height as u64);

    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())?;

    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = rays::Ray::new(camera_center, ray_direction);

            let pixel_color = rays::ray_color(r);
            pixel_color.write_color(&mut file)?;
        }
    }

    bar.finish();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::vectors;
    #[test]
    fn vectors() {
        let a = vectors::Vec3::new(0.0, 3.0, 0.0);
        let b = vectors::Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(a + b, vectors::Vec3::new(1.0, 4.0, 1.0));
        assert_eq!(a - b, vectors::Vec3::new(-1.0, 2.0, -1.0));
        assert_eq!(b.length_squared(), 3.0);
        assert_eq!(vectors::dot(a, b), 3.0);
        let c = vectors::Color::new(0.9, 0.1, 0.5);
        let mut stdout = std::io::stdout();
        c.write_color(&mut stdout).unwrap();
    }
}
