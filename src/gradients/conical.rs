use crate::gradients::{GradientParam, Vec2D};
pub struct Conical {
    center: [i32; 2],
    theta_0: f64, // init angle in radians
}

impl Conical {
    pub fn new() -> Self {
        Conical {
            center: [800 / 2, 800 / 2],
            theta_0: 0.0,
        }
    }

    pub fn center(mut self, center: [i32; 2]) -> Self {
        self.center = center;
        self
    }

    // angle is counted from the
    pub fn theta_0(mut self, theta_in_radians: f64) -> Self {
        self.theta_0 = theta_in_radians;
        self
    }
}

impl GradientParam for Conical {
    fn t(&self, coordinate: [i32; 2]) -> f64 {
        let v = Vec2D::new(self.center, coordinate);

        let angle = v.angle();
        let angle = angle - self.theta_0;
        let angle = ((angle % (2.0 * std::f64::consts::PI))
            + (2.0 * std::f64::consts::PI))
            % (2.0 * std::f64::consts::PI);

        angle / (2.0 * std::f64::consts::PI)
    }
}
