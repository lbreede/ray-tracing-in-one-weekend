use std::sync::Arc;

use nalgebra::Vector3;

use hittable::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::{random_float, random_float_range, random_vector, random_vector_range};

use crate::camera::CameraBuilder;
use crate::cube::Cube;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;
mod cube;

fn test_scene(world: &mut HittableList) {
    for a in -11..11 {
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Cube::new(center, 0.2 * 2.0, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vector_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Cube::new(center, 0.2 * 2.0, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Cube::new(center, 0.2 * 2.0, sphere_material)));
                }
            }
        }
    }
}

fn main() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    test_scene(&mut world);


    // let material1 = Arc::new(Dielectric::new(1.5));
    // world.add(Box::new(Sphere::new(
    //     Vector3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     material1,
    // )));
    //
    // let material2 = Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    // world.add(Box::new(Sphere::new(
    //     Vector3::new(-4.0, 1.0, 0.0),
    //     1.0,
    //     material2,
    // )));

    // world.add(Box::new(Cube::new(
    //     Vector3::new(-4.0, 1.0, 0.0),
    //     2.0,
    //     material2,
    // )));

    //
    // let material3 = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    // world.add(Box::new(Sphere::new(
    //     Vector3::new(4.0, 1.0, 0.0),
    //     1.0,
    //     material3,
    // )));

    let cam = CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(Vector3::new(13.0, 2.0, 3.0))
        .lookat(Vector3::new(0.0, 0.0, 0.0))
        .vup(Vector3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    cam.render(&mut world);
}
