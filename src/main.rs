mod color;
mod gradients;

pub use color::{Color, ColorLine};
pub use gradients::{
    Conical, Diamond, Easing, Gradient, GradientConfig, Linear,
    Radial, Sinusoidal, Square, Polynomial, Spiral
};


fn main() -> Result<(), std::fmt::Error> {
    let colors = ColorLine::new(vec![
        Color(173, 216, 230),
        Color(221, 160, 221),
    ]);

    let method = Spiral::new().spiral_factor(0.01);

    Gradient::new(method, colors)
        .to_rbg_img()
        .save("output/img71.png")
        .map_err(|_| std::fmt::Error)?;

    Ok(())
}
