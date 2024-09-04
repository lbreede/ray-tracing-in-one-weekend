use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_disk, sample_square};
use nalgebra::Vector3;
use std::time::Instant;

pub struct Camera {
    // aspect_ratio: f64,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,

    // vfov: f32,
    // lookfrom: Vector3<f32>,
    // lookat: Vector3<f32>,
    // vup: Vector3<f32>,
    defocus_angle: f32,
    // focus_dist: f32,
    image_height: u32,
    pixel_samples_scale: f32,
    center: Vector3<f32>,
    pixel00_loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    // u: Vector3<f32>,
    // v: Vector3<f32>,
    // w: Vector3<f32>,
    defocus_disk_u: Vector3<f32>,
    defocus_disk_v: Vector3<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            1.0,
            100,
            10,
            10,
            90.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
            0.0,
            10.0,
        )
    }
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f32,
        lookfrom: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f32;

        let center = lookfrom;

        // Determine viewport dimensions
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            // aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,

            // vfov,
            // lookfrom,
            // lookat,
            // vup,
            defocus_angle,
            // focus_dist,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            // u,
            // v,
            // w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn defocus_disk_sample(&self) -> Vector3<f32> {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    pub fn render(&self, world: &mut impl Hittable) {
        eprintln!("\n=== Render Started ===\n");
        let now = Instant::now();
        print!(
            "P3\n{image_width} {image_height}\n255\n",
            image_width = self.image_width,
            image_height = self.image_height
        );

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, self.max_depth, world);
                }
                write_color(self.pixel_samples_scale * pixel_color);
            }
        }
        let elapsed = now.elapsed();
        eprintln!("\n=== Render Complete! ===\n");
        eprintln!("Resolution: {}x{} px", self.image_width, self.image_height);
        eprintln!("Samples per Pixel: {}", self.samples_per_pixel);
        eprintln!("Max Depth: {}", self.max_depth);
        eprintln!("Render Time: {:.2?}\n", elapsed);
    }
    fn ray_color(r: &Ray, depth: u32, world: &mut impl Hittable) -> Vector3<f32> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        if world.hit(&r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Vector3::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation.component_mul(&Camera::ray_color(&scattered, depth - 1, world));
            }
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f32 + offset.x) * self.pixel_delta_u
            + (j as f32 + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }
}
