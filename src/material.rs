use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::random_vector_range;
use nalgebra::Vector3;

pub struct ScatterResult {
    pub attenuation: Vector3<f32>,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if near_zero(&scatter_direction) {
            scatter_direction = rec.normal;
        }

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut reflected = reflect(&r_in.direction, &rec.normal);
        reflected = reflected.normalize() + self.fuzz * random_unit_vector();

        Some(Ray::new(rec.p, reflected))
            .filter(|ray| ray.direction.dot(&rec.normal) > 0.0)
            .map(|scattered| ScatterResult {
                attenuation: self.albedo,
                scattered,
            })
    }
}

pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over the
    /// refractive index of the enclosing media
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
    /// Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || (Dielectric::reflectance(cos_theta, ri) > rand::random::<f32>()) {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        Some(ScatterResult {
            attenuation: Vector3::new(1.0, 1.0, 1.0),
            scattered: Ray::new(rec.p, direction),
        })
    }
}

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(n) * n
}
fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = -uv.dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn random_unit_vector() -> Vector3<f32> {
    loop {
        let p: Vector3<f32> = random_vector_range(-1.0, 1.0);
        let lensq = p.magnitude_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}
/// Return `true` if the vector is close to zero in all dimensions.
fn near_zero(v: &Vector3<f32>) -> bool {
    const S: f32 = 1e-8;
    v.x.abs() < S && v.y.abs() < S && v.z.abs() < S
}
