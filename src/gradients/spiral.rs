use crate::gradients::{Addressing, GradientParam, Vec2D};

pub struct Spiral {
    center: [i32; 2],
    spiral_factor: f64,
    addressing: Addressing,
}

impl Spiral {
    pub fn new() -> Self {
        // this might need better defaults
        Spiral {
            center: [400, 400],
            spiral_factor: 0.02,
            addressing: Addressing::Wrap,
        }
    }

    pub fn center(mut self, center: [i32; 2]) -> Self {
        self.center = center;
        self
    }

    pub fn spiral_factor(mut self, spiral_factor: f64) -> Self {
        self.spiral_factor = spiral_factor;
        self
    }

    pub fn addressing(mut self, addressing: Addressing) -> Self {
        self.addressing = addressing;
        self
    }
}

impl GradientParam for Spiral {
    fn t(&self, coordinate: [i32; 2]) -> f64 {
        let v = Vec2D::new(self.center, coordinate);

        let angle = v.angle();
        let distance = v.magnitude();

        // t = (angle + distance * spiral_factor) / (2π)
        let t = (angle + distance * self.spiral_factor)
            / (2.0 * std::f64::consts::PI);

        self.addressing.apply(t)
    }
}
