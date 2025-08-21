mod color;
mod gradients;

pub use color::{Color, ColorLine};
pub use gradients::{Easing, Gradient, Linear, Radial};

use crate::gradients::GradientConfig;

fn main() -> Result<(), std::fmt::Error> {
    let colors = ColorLine::new(vec![
        Color(255, 0, 0),
        Color(0, 255, 0),
        Color(0, 0, 255),
        Color(255, 255, 0),
    ]);

    let method = Linear::new()
        .start([9, 9])
        .end([189, 189])
        .addressing(gradients::Addressing::Clamp);

    Gradient::new(method, colors)
        .size(200, 200)
        .config(GradientConfig {
            easing: Easing::Linear,
        })
        .to_rbg_img()
        .save("output/img5.png")
        .map_err(|_| std::fmt::Error)?;

    Ok(())
}
