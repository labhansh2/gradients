use crate::gradients::{Addressing, GradientParam, Vec2D};

pub struct Radial {
    center: [i32; 2],
    radius: u32,
    addressing: Addressing,
}

impl Radial {
    pub fn new() -> Self {
        Radial {
            center: [400, 400],
            radius: (400_i32.pow(2) as f64 + (400_i32).pow(2) as f64)
                .sqrt() as u32,
            addressing: Addressing::Clamp,
        }
    }

    pub fn center(mut self, center: [i32; 2]) -> Self {
        self.center = center;
        self
    }

    pub fn radius(mut self, radius_in_px: u32) -> Self {
        self.radius = radius_in_px;
        self
    }

    pub fn radius_from_coordinates(
        mut self,
        coordinate: [i32; 2],
    ) -> Self {
        self.radius = ((coordinate[0] - self.center[0]).pow(2) as f64
            + (coordinate[1] - self.center[1]).pow(2) as f64)
            .sqrt() as u32;
        self
    }

    pub fn addressing(mut self, addressing: Addressing) -> Self {
        self.addressing = addressing;
        self
    }
}

impl GradientParam for Radial {
    fn t(&self, coordinate: [i32; 2]) -> f64 {
        let v = Vec2D::new(coordinate, self.center);

        self.addressing.apply(v.magnitude() / self.radius as f64)
    }
}
