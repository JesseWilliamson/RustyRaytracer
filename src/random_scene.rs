use raytracing_in_a_weekend_rust::{color, dielectric, hittable, hittable_list, lambertian, material, metal, point, sphere, vector};
use std::rc::Rc;

pub fn random_scene() -> hittable_list::HittableList {
    let mut world = hittable_list::HittableList::new(vec![]);
    let ground_material = Rc::new(lambertian::Lambertian::new(color::Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(sphere::Sphere::new(point::Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())) as Rc<dyn hittable::Hittable>);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = point::Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - point::Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vector::random() * vector::random();
                    Rc::new(lambertian::Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vector::random_in_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    Rc::new(metal::Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(dielectric::Dielectric::new(1.5))
                };
                world.add(Rc::new(sphere::Sphere::new(center, 0.2, sphere_material)) as Rc<dyn hittable::Hittable>);
            }
        }
    }

    let material1 = Rc::new(dielectric::Dielectric::new(1.5));
    world.add(Rc::new(sphere::Sphere::new(point::Point3::new(0.0, 1.0, 0.0), 1.0, material1.clone())) as Rc<dyn hittable::Hittable>);

    let material2 = Rc::new(lambertian::Lambertian::new(color::Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(sphere::Sphere::new(point::Point3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())) as Rc<dyn hittable::Hittable>);

    let material3 = Rc::new(metal::Metal::new(color::Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(sphere::Sphere::new(point::Point3::new(4.0, 1.0, 0.0), 1.0, material3.clone())) as Rc<dyn hittable::Hittable>);

    world
}
