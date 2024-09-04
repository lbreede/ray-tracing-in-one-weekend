mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Metal};
use nalgebra::Vector3;
use sphere::Sphere;
use vec3::{random_float, random_vector, random_vector_range,random_float_range};


fn main() {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(
        Vector3::new(0.5,0.5,0.5)
    ));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));


    for a in -11..11{
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Vector3::new(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vector().component_mul(&random_vector());
                    let sphere_material = Box::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vector_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Box::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Box::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Vector3::new(0.0,1.0,0.0), 1.0, material1)));

    let material2 = Box::new(Lambertian::new(Vector3::new(0.4,0.2,0.1)));
    world.add(Box::new(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Box::new(Metal::new(Vector3::new(0.7,0.6,0.5), 0.0));
    world.add(Box::new(Sphere::new(Vector3::new(4.0,1.0,0.0), 1.0, material3)));

    let image_width = 400; // 1200
    let samples_per_pixel = 2; // 500
    let max_depth = 10; // 50

    let cam = Camera::new(
        16.0 / 9.0, // 16.0 / 9.0
        image_width,
        samples_per_pixel,
        max_depth,
        20.0, // 20.0
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0,0.0,0.0),
        Vector3::new(0.0,1.0,0.0),
        0.6, // 0.6
        10.0, // 10.0
    );

    cam.render(&mut world);
}
