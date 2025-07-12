mod random_scene;
use raytracing_in_a_weekend_rust::camera::Camera;
use raytracing_in_a_weekend_rust::{vector, point};
use random_scene::random_scene;
use std::env;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", &args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    println!("{}", file_name);
    let mut file = File::create(file_name)?;

    let world = random_scene();

    let camera = Camera::new(
        1200,                 // image_width
        16.0 / 9.0,           // aspect_ratio
        20.0,                 // vertical_fov (vfov)
        point::Point3::new(13.0, 2.0, 3.0), // lookfrom
        point::Point3::new(0.0, 0.0, 0.0),  // lookat
        vector::Vec3::new(0.0, 1.0, 0.0),   // vup
        500,                  // samples_per_pixel
        50,                   // max_depth
        0.6,                  // defocus_angle
        10.0                  // focus_dist
    );

    camera.render(&world, &mut file)?;
    Ok(())
}
