use image::{Rgb, RgbImage};

pub mod linear;
pub mod utils;

use crate::color::{Color, ColorLine};
pub use linear::*;
pub use utils::*;

pub trait GradientParam {
    fn t(&self, coordinate: [u32; 2]) -> f64;
}

pub struct Gradient<M: GradientParam> {
    height: u32,
    width: u32,
    method: M,
    colors: ColorLine,
}

impl<M: GradientParam> Gradient<M> {
    pub fn new(method: M, colors: ColorLine) -> Self {
        Gradient {
            height: 800,
            width: 800,
            method: method,
            colors: colors,
        }
    }

    pub fn size(mut self, height: u32, width: u32) -> Self {
        self.height = height;
        self.width = width;
        self
    }

    // TO DO
    // pub fn method_from_str(&mut self, method: &str) -> Self {}

    pub fn to_rbg_img(&self) -> RgbImage {
        let mut rbg_img = RgbImage::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let t = self.method.t([x, y]);
                print!("[{}, {}]: {} -> ", x, y, t);
                let color = self.colors.interpolate(t);
                rbg_img.put_pixel(
                    x,
                    y,
                    Rgb([color.0, color.1, color.2]),
                );
            }
        }
        rbg_img
    }

    pub fn to_img_matrix(&self) -> Vec<Vec<Color>> {
        let mut img_matrix: Vec<Vec<Color>> = Vec::new();

        // TODO : optimize this
        for y in 0..self.height - 1 {
            let mut row: Vec<Color> = Vec::new();
            for x in 0..self.width - 1 {
                let t = self.method.t([x, y]);
                print!("[{}, {}]: {} -> ", x, y, t);
                row.push(self.colors.interpolate(t));
            }
            img_matrix.push(row[..].to_vec());
            row.clear();
        }

        img_matrix
    }
}
