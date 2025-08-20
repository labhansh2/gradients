#[derive(Clone, Copy)]
pub enum WrapType {
    CLAMP,
    WRAP,
    MIRROR,
}

pub struct Vec2D {
    x: u32,
    y: u32,
}

impl Vec2D {
    pub fn new(
        coordinate_a: [u32; 2],
        coordinate_b: [u32; 2],
    ) -> Self {
        Vec2D {
            x: coordinate_b[0] - coordinate_a[0],
            y: coordinate_b[1] - coordinate_a[1],
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

    pub fn dot(&self, other: &Self) -> u32 {
        self.x * other.x + self.y * other.y
    }
}
