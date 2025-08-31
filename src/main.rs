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
    ]);

    let method = PerlinNoise::new().grid_size((100, 100));

    Gradient::new(method, colors)
        .size(100, 100)
        // .config(GradientConfig { easing: Easing::Smootherstep })
        .to_rbg_img()
        .save("output/img81.png")
        .map_err(|_| std::fmt::Error)?;

    Ok(())
}
