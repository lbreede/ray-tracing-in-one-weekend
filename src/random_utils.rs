use nalgebra::Vector3;
use rand::Rng;

/// Generate a random float in the range [0, 1).
pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

/// Generate a random vector with each component in the range [0, 1).
pub fn random_vector() -> Vector3<f32> {
    Vector3::new(random_float(), random_float(), random_float())
}

/// Generate a random float in the range [min, max).
pub fn random_float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}

/// Generate a random vector with each component in the range [min, max).
pub fn random_vector_range(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        random_float_range(min, max),
        random_float_range(min, max),
        random_float_range(min, max),
    )
}
