use raytracing_in_a_weekend_rust::camera::Camera;
use raytracing_in_a_weekend_rust::{hittable_list, sphere, vector, color, point, lambertian, hittable, metal};
use std::env;
use std::fs::File;
use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", &args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    println!("{}", file_name);
    let mut file = File::create(file_name)?;
    
    let material_ground = Rc::new(lambertian::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(lambertian::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(metal::Metal::new(color::Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(metal::Metal::new(color::Color::new(0.8, 0.6, 0.2)));
    let world = hittable_list::HittableList::new(vec![
        Rc::new(sphere::Sphere::new(point::Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())) as Rc<dyn hittable::Hittable>,
        Rc::new(sphere::Sphere::new(point::Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone())) as Rc<dyn hittable::Hittable>,
        Rc::new(sphere::Sphere::new(point::Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())) as Rc<dyn hittable::Hittable>,
        Rc::new(sphere::Sphere::new(point::Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone())) as Rc<dyn hittable::Hittable>,
    ]);

    let camera = Camera::new(400, 16.0 / 9.0, 100);

    camera.render(&world, &mut file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::{vector, color};
    #[test]
    fn vectors() {
        let a = vector::Vec3::new(0.0, 3.0, 0.0);
        let b = vector::Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(a + b, vector::Vec3::new(1.0, 4.0, 1.0));
        assert_eq!(a - b, vector::Vec3::new(-1.0, 2.0, -1.0));
        assert_eq!(b.length_squared(), 3.0);
        assert_eq!(vector::dot(a, b), 3.0);
        let c = color::Color::new(0.9, 0.1, 0.5);
        let mut stdout = io::stdout();
        c.write_color(&mut stdout).unwrap();
    }
}
