mod color;
mod gradients;

pub use color::{Color, ColorLine};
pub use gradients::{
    Conical, Diamond, Easing, Gradient, GradientConfig, Linear,
    PerlinNoise, Polynomial, Radial, RandomNoise, Sinusoidal, Spiral,
    Square,
};

fn main() -> Result<(), std::fmt::Error> {
    let colors = ColorLine::new(vec![
        Color(51, 102, 255),
        Color(102, 51, 255),
        Color(153, 51, 255),
        Color(204, 102, 255),
        Color(255, 153, 255),
    ])
    .spread(vec![0.0, 0.20, 0.40, 0.60, 1.0])?;

    let method = PerlinNoise::new().grid_size((1000, 1000));

    Gradient::new(method, colors)
        .size(1080, 1920)
        .to_rbg_img()
        .save("output/img78.png")
        .map_err(|_| std::fmt::Error)?;

    Ok(())
}
