mod color;
mod gradients;

pub use color::{Color, ColorLine};
pub use gradients::{
    Easing, Gradient, GradientConfig, Linear,
    Radial, Conical, Square, Diamond
};

fn main() -> Result<(), std::fmt::Error> {
    let colors = ColorLine::new(vec![
        Color(255, 0, 0),
        Color(0, 255, 0),
        Color(255, 0, 0),
        Color(0, 255, 0),
        Color(255, 0, 0), // Color(0, 0, 255),
                          // Color(255, 255, 0),
                          // Color(255, 99, 71),
                          // Color(255, 215, 0),
                          // Color(65, 105, 225),
                          // Color(255, 182, 193),
                          // Color(173, 216, 230),
                          // Color(221, 160, 221),
    ]);

    let method = Linear::new()
        .start([9, 9])
        .end([189, 189])
        .addressing(gradients::Addressing::Clamp);

    let method_2 = Radial::new()
        .center([400, -1200])
        .radius(1000)
        .addressing(gradients::Addressing::Wrap);

    let method_3 = Conical::new();

    let method_4 = Square::new();

    let method_5 = Diamond::new();

    Gradient::new(method_5, colors)
        // .size(height, width)
        .config(GradientConfig {
            easing: Easing::Linear,
        })
        .to_rbg_img()
        .save("output/conical1.png")
        .map_err(|_| std::fmt::Error)?;

    Ok(())
}
