pub enum Addressing {
    Clamp,
    Wrap,
    Mirror,
}

impl Addressing {
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Self::Clamp => t.max(0.0).min(1.0),
            Self::Wrap => {
                let result = t - t.floor();
                if result == 0.0 && t > 0.0 {
                    1.0
                } else {
                    result
                }
            }
            Self::Mirror => {
                let wrapped = (t - t.floor()).abs();
                if (t.floor() as i32) % 2 == 0 {
                    wrapped
                } else {
                    1.0 - wrapped
                }
            }
        }
    }
}

// add support for custom easing
pub enum Easing {
    Linear,     // t
    Smoothstep, // 3t² - 2t³
    Smootherstep, // 6t⁵ - 15t⁴ + 10t³
                // Custom(F) // add later
}

impl Easing
// where F: Fn(f64) -> f64
{
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Self::Linear => t,
            Self::Smoothstep => 3.0 * t * t - 2.0 * t * t * t,
            Self::Smootherstep => {
                6.0 * t.powi(5) - 15.0 * t.powi(4) + 10.0 * t.powi(3)
            } // Self::Custom(f) => f(t) // later
        }
    }
}

pub fn normalize_rng(
    t: f64,
    current_range: [f64; 2],
    desired_range: [f64; 2],
) -> f64 {
    (t - current_range[0]) / (current_range[1] - current_range[0])
        * (desired_range[1] - desired_range[0])
        + desired_range[0]
}

#[derive(Clone)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(
        coordinate_a: (f64, f64),
        coordinate_b: (f64, f64),
    ) -> Self {
        Vec2D {
            x: coordinate_b.0 - coordinate_a.0,
            y: coordinate_b.1 - coordinate_a.1,
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x).powf(2.0) + (self.y).powf(2.0)).sqrt()
    }

    pub fn angle(&self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }
}
