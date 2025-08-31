use crate::gradients::GradientParam;
use rand::Rng;

pub struct RandomNoise;

impl GradientParam for RandomNoise {
    fn t(&self, coordinate: (f64, f64)) -> f64 {
        let mut rand = rand::rng();

        rand.random_range(0.0..1.0)
    }
}
