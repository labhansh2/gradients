use crate::gradients::{Addressing, GradientParam};

pub struct Diamond {
    center: (f64, f64),
    max_distance: f64,
    addressing: Addressing,
}

impl Diamond {
    pub fn new() -> Self {
        Diamond {
            center: (400.0, 400.0),
            max_distance: 400.0,
            addressing: Addressing::Clamp,
        }
    }

    pub fn center(mut self, center: (f64, f64)) -> Self {
        self.center = center;
        self
    }

    pub fn max_distance(mut self, max_distance: f64) -> Self {
        self.max_distance = max_distance;
        self
    }

    pub fn addressing(mut self, addressing: Addressing) -> Self {
        self.addressing = addressing;
        self
    }
}

impl GradientParam for Diamond {
    fn t(&self, coordinate: (f64, f64)) -> f64 {
        // |x-cx| + |y-cy|
        let distance = ((coordinate.0 - self.center.0).abs()
            + (coordinate.1 - self.center.1).abs())
            as f64;
        let t = distance / self.max_distance;

        self.addressing.apply(t)
    }
}
