use crate::gradients::{Addressing, GradientParam, Vec2D};

pub struct Linear {
    start: (f64, f64),
    end: (f64, f64),
    addressing: Addressing,
}

impl Linear {
    pub fn new() -> Self {
        Linear {
            start: (0.0, 0.0),
            end: (800.0, 800.0),
            addressing: Addressing::Clamp,
        }
    }

    pub fn start(mut self, start: (f64, f64)) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: (f64, f64)) -> Self {
        self.end = end;
        self
    }

    pub fn addressing(mut self, addressing: Addressing) -> Self {
        self.addressing = addressing;
        self
    }
}

impl GradientParam for Linear {
    fn t(&self, coordinate: (f64, f64)) -> f64 {
        let d = Vec2D::new(self.start, self.end);
        let v = Vec2D::new(self.start, coordinate);

        self.addressing.apply(d.dot(&v) as f64 / d.dot(&d) as f64)
    }
}
