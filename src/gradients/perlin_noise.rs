use std::cell::RefCell;
use std::collections::HashMap;

use rand::Rng;

use crate::gradients::{
    Easing::Smootherstep as easing, GradientParam, Vec2D,
    normalize_rng,
};

pub struct PerlinNoise {
    grid_size: (u32, u32),
    grid_vector_map: RefCell<HashMap<(u32, u32), Vec2D>>,
}

impl PerlinNoise {
    pub fn new() -> Self {
        PerlinNoise {
            grid_size: (400, 400),
            grid_vector_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn grid_size(mut self, grid_size: (u32, u32)) -> Self {
        self.grid_size = grid_size;
        self
    }

    fn get_gradient_vector(&self, coordinate: (u32, u32)) -> Vec2D {
        // TODO: can use a hash as seed
        if let Some(v) =
            self.grid_vector_map.borrow().get(&coordinate)
        {
            return v.clone();
        } else {
            let mut rnd = rand::rng();

            let theta: f64 =
                rnd.random::<f64>() * std::f64::consts::TAU;
            let random_vec =
                Vec2D::new((0.0, 0.0), (theta.cos(), theta.sin()));

            self.grid_vector_map
                .borrow_mut()
                .insert(coordinate, random_vec.clone());
            return random_vec;
        };
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
