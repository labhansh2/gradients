use image::{Rgb, RgbImage};

// shape based
pub mod conical;
pub mod diamond;
pub mod linear;
pub mod radial;
pub mod square;

// function based
pub mod polynomial;
pub mod sinusoidal;
pub mod spiral;
pub mod utils;

// noise based
pub mod perlin_noise;
pub mod random_noise;

// utils
use crate::color::{Color, ColorLine};
pub use utils::*;

// shape based
pub use conical::*;
pub use diamond::*;
pub use linear::*;
pub use radial::*;
pub use square::*;

// function based
pub use polynomial::*;
pub use sinusoidal::*;
pub use spiral::*;

// noise based
pub use perlin_noise::*;
pub use random_noise::*;

pub trait GradientParam {
    fn t(&self, coordinate: (f64, f64)) -> f64;
}

pub struct GradientConfig {
    pub easing: Easing,
}

pub struct Gradient<M: GradientParam> {
    height: u32,
    width: u32,
    method: M,
    colors: ColorLine,
    config: GradientConfig,
}

impl<M: GradientParam> Gradient<M> {
    pub fn new(method: M, colors: ColorLine) -> Self {
        Gradient {
            height: 800,
            width: 800,
            method: method,
            colors: colors,
            config: GradientConfig {
                easing: Easing::Linear,
            },
        }
    }

    pub fn size(mut self, height: u32, width: u32) -> Self {
        self.height = height;
        self.width = width;
        self
    }

    pub fn config(mut self, config: GradientConfig) -> Self {
        self.config = config;
        self
    }

    // TO DO
    // pub fn method_from_str(& self, method: &str) -> Self {}

    pub fn to_rbg_img(&self) -> RgbImage {
        let mut rbg_img = RgbImage::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let t = self.method.t((x as f64, y as f64));
                let t = self.config.easing.apply(t);
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
                let t = self.method.t((x as f64, y as f64));
                let t = self.config.easing.apply(t);
                print!("[{}, {}]: {} -> ", x, y, t);
                row.push(self.colors.interpolate(t));
            }
            img_matrix.push(row[..].to_vec());
            row.clear();
        }

        img_matrix
    }
}
