use crate::gradients::{Addressing, GradientParam, Vec2D};

pub struct Polynomial {
    origin: [i32; 2],
    direction: Option<Vec2D>,
    exponent: f64,
    max_distance: f64,
    addressing: Addressing,
}

impl Polynomial {
    pub fn new() -> Self {
        Polynomial {
            origin: [400, 400],
            direction: None,
            exponent: 2.0,
            max_distance: 400.0,
            addressing: Addressing::Clamp,
        }
    }

    pub fn origin(mut self, origin: [i32; 2]) -> Self {
        self.origin = origin;
        self
    }

    pub fn direction(mut self, direction: Vec2D) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn direction_from_coordinates(
        mut self,
        coordinate_a: [i32; 2],
        coordinate_b: [i32; 2],
    ) -> Self {
        let d = Vec2D::new(coordinate_a, coordinate_b);
        self.direction = Some(d);
        self
    }

    pub fn exponent(mut self, exponent: f64) -> Self {
        self.exponent = exponent;
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

impl GradientParam for Polynomial {
    fn t(&self, coordinate: [i32; 2]) -> f64 {
        let v = Vec2D::new(self.origin, coordinate);

        let position: f64 = if let Some(d) = &self.direction {
            (v.dot(&d) as f64 / d.magnitude()) / self.max_distance
        } else {
            v.magnitude() / self.max_distance
        };

        // t = position^n where n controls curvature
        let t =
            position.abs().powf(self.exponent) * position.signum();

        self.addressing.apply(t)
    }
}
