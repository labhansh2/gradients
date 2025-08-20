use crate::gradients::{GradientParam, Vec2D, WrapType};

pub struct Linear {
    start: [u32; 2],
    end: [u32; 2],
    mode: WrapType,
}

impl Linear {
    pub fn new() -> Self {
        Linear {
            start: [0, 0],
            end: [800, 800],
            mode: WrapType::CLAMP,
        }
    }

    pub fn start(mut self, start: [u32; 2]) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: [u32; 2]) -> Self {
        self.end = end;
        self
    }
}

impl GradientParam for Linear {
    fn t(&self, coordinate: [u32; 2]) -> f64 {
        let d = Vec2D::new(self.start, self.end);
        let v = Vec2D::new(self.start, coordinate);

        d.dot(&v) as f64 / d.dot(&d) as f64
    }
}
