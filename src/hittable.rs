use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub mat: Box<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vector3<f32>, t: f32, mat: Box<dyn Material>) -> Self {
        Self {
            p,
            normal: Vector3::new(0.0, 0.0, 0.0),
            mat,
            t,
            front_face: false,
        }
    }
    /// Sets the hit record normal vector.
    ///
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_anything = None;
        for object in self.objects.iter() {
            if let Some(rec) = object.hit(&r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }
        hit_anything
    }
}
