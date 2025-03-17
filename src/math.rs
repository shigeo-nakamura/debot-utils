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

pub fn compute_std(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mean = values.iter().copied().sum::<f64>() / (values.len() as f64);
    let var = values
        .iter()
        .map(|v| {
            let diff = v - mean;
            diff * diff
        })
        .sum::<f64>()
        / (values.len() as f64);
    var.sqrt()
}
