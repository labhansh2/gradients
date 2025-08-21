use crate::gradients::{Addressing, GradientParam, Vec2D};

pub struct Radial {
    center: [u32; 2],
    radius: u32,
    addressing: Addressing,
}

impl Radial {
    pub fn new() -> Self {
        Radial {
            center: [(800 - 1) / 2, (800 - 1) / 2],
            radius: 400, // TODO
            addressing: Addressing::Clamp,
        }
    }

    pub fn center(mut self, center: [u32; 2]) -> Self {
        self.center = center;
        self
    }

    pub fn radius(mut self, radius_in_px: u32) -> Self {
        self.radius = radius_in_px;
        self
    }
}

// impl GradientParam for Radial {
//     fn t(&self, coordinate: [u32; 2]) -> f64 {
//         let v = Vec2D::new(self.center, coordinate);

//         v.magnitude()
//     }
// }
