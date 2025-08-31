use crate::gradients::{Addressing, GradientParam, Vec2D};

pub struct Radial {
    center: (f64, f64),
    radius: f64,
    addressing: Addressing,
}

impl Radial {
    pub fn new() -> Self {
        Radial {
            center: (400.0, 400.0),
            radius: (400_i32.pow(2) as f64 + (400_i32).pow(2) as f64)
                .sqrt(),
            addressing: Addressing::Clamp,
        }
    }

    pub fn center(mut self, center: (f64, f64)) -> Self {
        self.center = center;
        self
    }

    pub fn radius(mut self, radius_in_px: f64) -> Self {
        self.radius = radius_in_px;
        self
    }

    pub fn radius_from_coordinates(
        mut self,
        coordinate: (f64, f64),
    ) -> Self {
        self.radius = ((coordinate.0 - self.center.0).powf(2.0)
            as f64
            + (coordinate.1 - self.center.1).powf(2.0) as f64)
            .sqrt();
        self
    }

    pub fn addressing(mut self, addressing: Addressing) -> Self {
        self.addressing = addressing;
        self
    }
}

impl GradientParam for Radial {
    fn t(&self, coordinate: (f64, f64)) -> f64 {
        let v = Vec2D::new(coordinate, self.center);

        self.addressing.apply(v.magnitude() / self.radius as f64)
    }
}
