use std::fmt::Error;

use image::{Rgb, RgbImage};

#[derive(Clone)]
struct Color(u8, u8, u8);

struct ColorBand {
    colors: Vec<Color>,
    positions: Vec<f64>,
}

impl ColorBand {
    fn new(colors: Vec<Color>) -> Self {
        let len = colors.len();

        let positions: Vec<f64> = (0..len).map(|i| i as f64 / len as f64).collect();
        ColorBand { colors, positions }
    }

    fn positions(self, positions: Vec<f64>) -> Result<Self, Error> {
        if positions.len() != self.colors.len() {
            return Err(Error);
        }
        Ok(ColorBand {
            colors: self.colors,
            positions: positions,
        })
    }

    fn interpolate(&self, t: f64) -> Color {
        // TODO : fix multi color interpolation

        let size = self.colors.len();
        let pos = t * size as f64;
        let mut pos = pos.floor();
        // [c1, c2, c3]
        if pos == 1.0 {
            pos = pos - 1.0;
        }

        let new_t = t
            - self.positions[pos as usize] / self.positions[pos as usize + 1]
            - self.positions[pos as usize];

        // i = 3/4 * 2 = 6/4 -> 1
        // c_a = color[i]
        // c_b = color[i+1]
        // n_t = 3/4 - postion[i] (1/2) / postion[i+1] - position[i]
        // c2 *
        // 1 * 2 = 2 -> 2
        // 1/2 * 2 = 1

        // interpolation eq
        // color = c_a * (1 - t) + c_b * t

        // only supports 2 colors for now
        let r = self.colors[pos as usize].0 as f64 * new_t
            + self.colors[pos as usize + 1].0 as f64 * (1.0 - t);
        let g = self.colors[pos as usize].1 as f64 * new_t
            + self.colors[pos as usize + 1].1 as f64 * (1.0 - new_t);
        let b = self.colors[pos as usize].2 as f64 * new_t
            + self.colors[pos as usize + 1].2 as f64 * (1.0 - new_t);

        let rr = r as u8;
        let gg = g as u8;
        let bb = b as u8;
        println!("{rr} {gg} {bb}");
        Color(r as u8, b as u8, g as u8)
    }
}

struct Vec2D {
    x: i32,
    y: i32,
}

impl Vec2D {
    fn new(coordinate_a: [i32; 2], coordinate_b: [i32; 2]) -> Self {
        Vec2D {
            x: coordinate_b[0] - coordinate_a[0],
            y: coordinate_b[1] - coordinate_a[1],
        }
    }
    fn add(&self, other: &Self) -> Self {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn subtract(&self, other: &Self) -> Self {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn dot(&self, other: &Self) -> i32 {
        self.x * other.x + self.y * other.y
    }
}

#[derive(Clone, Copy)] // 
enum WrapType {
    CLAMP,
    WRAP,
    MIRROR,
}

trait GradientParam {
    fn t(&self, coordinate: [i32; 2]) -> f64;
}

struct Linear {
    start: [i32; 2],
    end: [i32; 2],
    mode: WrapType,
}

impl Linear {
    fn new() -> Self {
        Linear {
            start: [0, 0],
            end: [200, 20],
            mode: WrapType::CLAMP,
        }
    }

    fn start(&self, start: [i32; 2]) -> Self {
        Linear {
            start: start,
            end: self.end,
            mode: self.mode,
        }
    }

    fn end(&self, end: [i32; 2]) -> Self {
        Linear {
            start: self.start,
            end: end,
            mode: self.mode,
        }
    }
}

impl GradientParam for Linear {
    fn t(&self, coordinate: [i32; 2]) -> f64 {
        let d = Vec2D::new(self.start, self.end);
        let v = Vec2D::new(self.start, coordinate);

        d.dot(&v) as f64 / d.dot(&d) as f64 % 1 as f64
    }
}

#[derive(Clone, Copy)]
struct GradientSize {
    height: i32,
    width: i32,
}

fn generate_gradient<M: GradientParam>(
    gradient_size: GradientSize,
    method: M,
    colors: ColorBand,
) -> Vec<Vec<Color>> {
    let mut gradient_img: Vec<Vec<Color>> = Vec::new();

    // TODO : optimize this
    for y in 0..gradient_size.height {
        let mut row: Vec<Color> = Vec::new();
        for x in 0..gradient_size.width {
            let t = method.t([x, y]);
            print!("[{x}, {y}] : {t}      ->>>>>>");
            row.push(colors.interpolate(t));
        }
        gradient_img.push(row[..].to_vec());
        row.clear();
    }

    gradient_img
}

fn main() {
    let method = Linear::new().start([0, 0]).end([799, 799]);
    let colors = vec![Color(255, 0, 0), Color(0, 255, 0)];
    let colors = ColorBand::new(colors);
    let gradient_size = GradientSize {
        height: 800,
        width: 800,
    };
    let img_matrix = generate_gradient(gradient_size, method, colors);

    let mut img = RgbImage::new(gradient_size.width as u32, gradient_size.height as u32);
    for (y, row) in img_matrix.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            img.put_pixel(x as u32, y as u32, Rgb([pixel.0, pixel.1, pixel.2]));
        }
    }

    img.save("output/output3.png").unwrap();
}
