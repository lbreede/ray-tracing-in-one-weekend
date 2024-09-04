use nalgebra::Vector3;
use rand::Rng;

pub fn random_vector() -> Vector3<f32> {
    Vector3::new(
        random_float(),
        random_float(),
        random_float(),
    )
}

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_float_range(min: f32, max:f32) -> f32 {
    min + (max-min) * random_float()
}

pub fn random_vector_range(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        random_float_range(min, max),
        random_float_range(min, max),
        random_float_range(min, max),
    )
}
pub fn random_in_unit_disk() -> Vector3<f32> {
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
pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}
pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = -uv.dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn random_unit_vector() -> Vector3<f32> {
    loop {
        let p: Vector3<f32> = random_vector_range(-1.0, 1.0);
        let lensq = p.magnitude_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}
pub fn near_zero(v: &Vector3<f32>) -> bool {
    const S: f32 = 1e-8;
    v.x.abs() < S && v.y.abs() < S && v.z.abs() < S
}

pub fn sample_square() -> Vector3<f32> {
    Vector3::new(
        random_float() - 0.5,
        random_float() - 0.5,
        0.0,
    )
}