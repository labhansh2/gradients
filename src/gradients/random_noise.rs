use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rand::{self, Rng};

use crate::gradients::GradientParam;

pub struct RandomNoise {
    seed: u64,
    frequency: f64,
}

impl RandomNoise {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        RandomNoise {
            seed: rng.random(), // random seed if not added
            frequency: 1.0,
        }
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }
}

impl GradientParam for RandomNoise {
    fn t(&self, coordinate: (f64, f64)) -> f64 {
        let scaled_x = (coordinate.0 * self.frequency).floor() as u64;
        let scaled_y = (coordinate.1 * self.frequency).floor() as u64;

        let mut hasher = DefaultHasher::new();
        self.seed.hash(&mut hasher);
        scaled_x.hash(&mut hasher);
        scaled_y.hash(&mut hasher);

        let hash = hasher.finish();

        (hash as f64) / (u64::MAX as f64)
    }
}
