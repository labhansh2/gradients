mod color;
mod gradients;

pub use color::{Color, ColorLine};
pub use gradients::{Gradient, Linear};

fn main() -> Result<(), std::fmt::Error> {
    let colors = ColorLine::new(vec![
        Color(255, 0, 0),
        Color(0, 255, 0),
        Color(0, 0, 255),
        Color(255, 255, 0),
    ])
    .spread(vec![0.0, 0.125, 0.875, 1.0])?;

    let method = Linear::new().start([0, 0]).end([511, 511]);

    Gradient::new(method, colors)
        .size(512, 512)
        .to_rbg_img()
        .save("output/img4.png")
        .map_err(|_| std::fmt::Error)?;

    Ok(())
}
