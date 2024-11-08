use rand::Rng;

pub fn exponential_decay(x: f64, a: f64) -> f64 {
    f64::exp(-a * x)
}

pub fn random_bool(probability: f64) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(probability)
}
