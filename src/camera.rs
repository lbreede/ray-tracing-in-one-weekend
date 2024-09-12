use std::time::Instant;

use nalgebra::Vector3;

use crate::color::write_color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::random_utils::{random_float, random_float_range};
use crate::ray::Ray;

pub struct Camera {
    image_width: u16,
    image_height: u16,
    samples_per_pixel: u32,
    max_depth: u32,

    defocus_angle: f32,
    pixel_samples_scale: f32,
    center: Vector3<f32>,
    pixel00_loc: Vector3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,
    defocus_disk_u: Vector3<f32>,
    defocus_disk_v: Vector3<f32>,
}

impl Camera {
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
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f32::INFINITY)) {
            if let Some(scatter) = rec.mat.scatter(r, &rec) {
                return scatter.attenuation.component_mul(&Camera::ray_color(
                    &scatter.scattered,
                    depth - 1,
                    world,
                ));
            }
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }

    /// Construct a camera ray originating from the defocus disk and directed at a randomly sampled
    /// point around the pixel location i, j.
    fn get_ray(&self, i: u16, j: u16) -> Ray {
        let offset = Camera::sample_square();
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

    fn random_in_unit_disk() -> Vector3<f32> {
        loop {
            let p = Vector3::new(
                random_float_range(-1.0, 1.0),
                random_float_range(-1.0, 1.0),
                0.0,
            );
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }

    fn sample_square() -> Vector3<f32> {
        Vector3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
    }
    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Vector3<f32> {
        let p = Camera::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

pub struct CameraBuilder {
    aspect_ratio: f32,
    image_width: u16,
    samples_per_pixel: u32,
    max_depth: u32,
    vfov: f32,
    lookfrom: Vector3<f32>,
    lookat: Vector3<f32>,
    vup: Vector3<f32>,
    defocus_angle: f32,
    focus_dist: f32,
}

impl CameraBuilder {
    // Create a new builder with default values
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 20.0,
            lookfrom: Vector3::new(13.0, 2.0, 3.0),
            lookat: Vector3::new(0.0, 0.0, 0.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
            focus_dist: 10.0,
        }
    }

    // Setters for each field to allow customization
    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn image_width(mut self, image_width: u16) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn vfov(mut self, vfov: f32) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn lookfrom(mut self, lookfrom: Vector3<f32>) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn lookat(mut self, lookat: Vector3<f32>) -> Self {
        self.lookat = lookat;
        self
    }

    pub fn vup(mut self, vup: Vector3<f32>) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f32) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f32) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    // Build method to create the Camera object
    pub fn build(self) -> Camera {
        let image_height = ((self.image_width as f32 / self.aspect_ratio) as u16).max(1);
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;
        let center = self.lookfrom;

        // Determine viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.vup.cross(&w).normalize();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors to the next pixel.
        let pixel_delta_u = viewport_u / self.image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width: self.image_width,
            image_height,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            defocus_angle: self.defocus_angle,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
