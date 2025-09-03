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
        // Line 1
        Color(51, 102, 255),
        Color(102, 51, 255),
        Color(153, 51, 255),
        Color(204, 102, 255),
        Color(255, 153, 255),
        // Line 2
        // Color(255, 255, 255),
        // Color(3, 35, 45),
        // Color(237, 98, 93),
        // Color(66, 182, 198),
        // Color(247, 159, 136),
        // Color(178, 223, 230),
        // Color(255, 255, 255)
    ]);

    let method =
        PerlinNoise::new().grid_size((512, 512)).n_octaves(5)?;

    // let method = Linear::new();
    Gradient::new(method, colors)
        .size(1024, 1024)
        .to_rbg_img()
        .save("output/img85.png")
        .map_err(|_| std::fmt::Error)?;

    Ok(())
}
