use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rand::{self, Rng};

use crate::gradients::{
    Easing::Smootherstep as easing, GradientParam, Vec2D,
    normalize_rng,
};

pub struct PerlinNoise {
    grid_size: (u32, u32),
    seed: u64,
}

impl PerlinNoise {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        PerlinNoise {
            grid_size: (400, 400),
            seed: rng.random() // random seed if not seeded explicitly 
        }
    }

    pub fn grid_size(mut self, grid_size: (u32, u32)) -> Self {
        self.grid_size = grid_size;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    fn get_gradient_vector(&self, coordinate: (u32, u32)) -> Vec2D {
        let mut hasher = DefaultHasher::new();
        self.seed.hash(&mut hasher);
        coordinate.0.hash(&mut hasher);
        coordinate.1.hash(&mut hasher);
        let hash = hasher.finish();

        let theta: f64 =
            (hash as f64 / u64::MAX as f64) * std::f64::consts::TAU;

        Vec2D::new((0.0, 0.0), (theta.cos(), theta.sin()))
    }
}

impl GradientParam for PerlinNoise {
    fn t(&self, coordinate: (f64, f64)) -> f64 {
        // 0 -> left-top
        // 1 -> right-top
        // 2 -> right-bottom
        // 3 -> left-bottom

        let cell_x =
            (coordinate.0 / self.grid_size.0 as f64).floor() as u32;
        let cell_y =
            (coordinate.1 / self.grid_size.1 as f64).floor() as u32;

        let corner_0 =
            (cell_x * self.grid_size.0, cell_y * self.grid_size.1);

        let corner_1 = (corner_0.0 + self.grid_size.0, corner_0.1);
        let corner_2 = (
            corner_0.0 + self.grid_size.0,
            corner_0.1 + self.grid_size.1,
        );
        let corner_3 = (corner_0.0, corner_0.1 + self.grid_size.1);

        let v_0 = self.get_gradient_vector(corner_0);
        let v_1 = self.get_gradient_vector(corner_1);
        let v_2 = self.get_gradient_vector(corner_2);
        let v_3 = self.get_gradient_vector(corner_3);

        let vec_frm_0 = Vec2D {
            x: (coordinate.0 - corner_0.0 as f64)
                / self.grid_size.0 as f64,
            y: (coordinate.1 - corner_0.1 as f64)
                / self.grid_size.1 as f64,
        };
        let vec_frm_1 = Vec2D {
            x: (coordinate.0 - corner_1.0 as f64)
                / self.grid_size.0 as f64,
            y: (coordinate.1 - corner_1.1 as f64)
                / self.grid_size.1 as f64,
        };
        let vec_frm_2 = Vec2D {
            x: (coordinate.0 - corner_2.0 as f64)
                / self.grid_size.0 as f64,
            y: (coordinate.1 - corner_2.1 as f64)
                / self.grid_size.1 as f64,
        };
        let vec_frm_3 = Vec2D {
            x: (coordinate.0 - corner_3.0 as f64)
                / self.grid_size.0 as f64,
            y: (coordinate.1 - corner_3.1 as f64)
                / self.grid_size.1 as f64,
        };

        let influence_0 = v_0.dot(&vec_frm_0);
        let influence_1 = v_1.dot(&vec_frm_1);
        let influence_2 = v_2.dot(&vec_frm_2);
        let influence_3 = v_3.dot(&vec_frm_3);

        let d_x = easing.apply(
            (coordinate.0 - corner_0.0 as f64)
                / self.grid_size.0 as f64,
        );
        let d_y = easing.apply(
            (coordinate.1 - corner_0.1 as f64)
                / self.grid_size.1 as f64,
        );

        let top_influence =
            influence_0 + d_x * (influence_1 - influence_0);
        let bot_influence =
            influence_3 + d_x * (influence_2 - influence_3);

        let raw =
            top_influence + d_y * (bot_influence - top_influence);
        println!("Raw: {}", raw);

        normalize_rng(raw, [-1.0, 1.0], [0.0, 1.0])
    }
}
