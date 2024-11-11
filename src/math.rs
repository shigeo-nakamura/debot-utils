use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

pub fn exponential_decay(x: f64, a: f64) -> f64 {
    f64::exp(-a * x)
}

pub fn random_bool(probability: f64, seed: Option<u64>) -> bool {
    let mut rng: Box<dyn RngCore> = match seed {
        Some(seed_value) => Box::new(StdRng::seed_from_u64(seed_value)),
        None => Box::new(rand::thread_rng()),
    };
    rng.gen_bool(probability)
}
