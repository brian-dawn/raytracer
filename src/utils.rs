use rand::prelude::*;

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random() -> f64 {
    let mut rng = thread_rng();

    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();

    rng.gen_range(min, max)
}
