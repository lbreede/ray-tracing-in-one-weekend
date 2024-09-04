use nalgebra::Vector3;
use rand::Rng;

pub fn random_vector() -> Vector3<f32> {
    Vector3::new(random_float(), random_float(), random_float())
}

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}

pub fn random_vector_range(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        random_float_range(min, max),
        random_float_range(min, max),
        random_float_range(min, max),
    )
}
