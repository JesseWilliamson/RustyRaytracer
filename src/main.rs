use raytracing_in_a_weekend_rust::camera::CameraBuilder;
use raytracing_in_a_weekend_rust::point;
use raytracing_in_a_weekend_rust::random_scene;
use raytracing_in_a_weekend_rust::vector;
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

    let world = random_scene::random_scene();
    let camera_position = point::Point3::new(13.0, 2.0, 3.0);
    let camera_look_at = point::Point3::new(0.0, 0.0, 0.0);
    let camera_up_vector = vector::Vec3::new(0.0, 1.0, 0.0);

    let camera = CameraBuilder::default()
        .image_width(1200)
        .aspect_ratio(16.0 / 9.0)
        .vertical_fov(20.0)
        .look_from(camera_position)
        .look_at(camera_look_at)
        .vup(camera_up_vector)
        .samples_per_pixel(500)
        .max_depth(50)
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    camera.render(&world, &mut file)?;
    Ok(())
}
