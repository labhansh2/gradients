use crate::gradients::{GradientParam, Vec2D, normalize_rng};

pub struct Sinusoidal {
    origin: (f64, f64),
    direction: Option<Vec2D>, // change this later to a curve
    // use a generic which is a closure
    frequency: f64,
}

impl Sinusoidal {
    pub fn new() -> Self {
        Sinusoidal {
            origin: (400.0, 400.0),
            direction: Option::None,
            frequency: 1.0,
        }
    }

    pub fn origin(mut self, origin: (f64, f64)) -> Self {
        self.origin = origin;
        self
    }

    pub fn direction(mut self, direction: Vec2D) -> Self {
        self.direction = Option::Some(direction);
        self
    }

    pub fn direction_from_coordinates(
        mut self,
        coordinate_a: (f64, f64),
        coordinate_b: (f64, f64),
    ) -> Self {
        let d = Vec2D::new(coordinate_a, coordinate_b);
        self.direction = Some(d);
        self
    }

    pub fn frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }
}

impl GradientParam for Sinusoidal {
    fn t(&self, coordinate: (f64, f64)) -> f64 {
        let v = Vec2D::new(self.origin, coordinate);

        let distance: f64 = if let Some(d) = &self.direction {
            v.dot(&d) as f64 / d.magnitude()
        } else {
            v.magnitude()
        };

        normalize_rng(
            (distance * self.frequency * std::f64::consts::PI
                / 180.0)
                .sin(),
            [-1.0, 1.0],
            [0.0, 1.0],
        )
    }
}
