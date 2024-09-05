use std::sync::Arc;

use nalgebra::Vector3;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

pub struct Cube {
    center: Vector3<f32>,
    length: f32,
    mat: Arc<dyn Material>,
}

impl Cube {
    pub fn new(center: Vector3<f32>, length: f32, mat: Arc<dyn Material>) -> Self {
        Self { center, length, mat }
    }
    fn compute_normal(&self, point: Vector3<f32>) -> Vector3<f32> {
        let half_length = self.length / 2.0;
        let min = self.center - Vector3::new(half_length, half_length, half_length);
        let max = self.center + Vector3::new(half_length, half_length, half_length);

        // Determine which axis-aligned plane was hit by checking proximity to each face.
        if (point.x - min.x).abs() < 1e-6 {
            Vector3::new(-1.0, 0.0, 0.0)
        } else if (point.x - max.x).abs() < 1e-6 {
            Vector3::new(1.0, 0.0, 0.0)
        } else if (point.y - min.y).abs() < 1e-6 {
            Vector3::new(0.0, -1.0, 0.0)
        } else if (point.y - max.y).abs() < 1e-6 {
            Vector3::new(0.0, 1.0, 0.0)
        } else if (point.z - min.z).abs() < 1e-6 {
            Vector3::new(0.0, 0.0, -1.0)
        } else {
            Vector3::new(0.0, 0.0, 1.0)
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Half of the cube's side length.
        let half_length = self.length / 2.0;

        // Define min and max points of the cube.
        let min = self.center - Vector3::new(half_length, half_length, half_length);
        let max = self.center + Vector3::new(half_length, half_length, half_length);

        // Initialize t_min and t_max with the ray interval.
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;

        // Iterate over the 3 axes (x, y, z).
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let mut t0 = (min[i] - r.origin()[i]) * inv_d;
            let mut t1 = (max[i] - r.origin()[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return None; // No intersection occurs.
            }
        }

        // Calculate the hit point and determine the outward normal.
        let hit_point = r.at(t_min);
        let outward_normal = self.compute_normal(hit_point);

        // Create the HitRecord.
        let mut rec = HitRecord::new(
            hit_point,
            t_min,
            Arc::clone(&self.mat),
            // outward_normal,
            // true, // Placeholder, actual front face check done in set_face_normal.
        );
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}