use std::fmt::Error;

#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub struct ColorLine {
    colors: Vec<Color>,
    spread: Vec<f64>,
}

impl ColorLine {
    pub fn new(colors: Vec<Color>) -> Self {
        let len = colors.len();

        let spread: Vec<f64> = (0..len)
            .map(|i| (i as f64) / (len as f64 - 1.0))
            .collect();
        ColorLine {
            colors: colors,
            spread: spread,
        }
    }

    pub fn spread(self, spread: Vec<f64>) -> Result<Self, Error> {
        if spread.len() != self.colors.len() {
            return Err(Error);
        }
        Ok(ColorLine {
            colors: self.colors,
            spread: spread,
        })
    }

    pub fn interpolate(&self, t: f64) -> Color {
        // TODO: this is ugly but works; clean later
        if t == 1.0 {
            return self.colors.last().unwrap().clone();
        };

        let len = self.colors.len();
        let mut curr_index: f64 = 0.0;
        let mut next_index: f64 = 1.0;

        while next_index < len as f64 - 1.0 {
            if t >= self.spread[curr_index as usize]
                && t < self.spread[next_index as usize]
            {
                break;
            }
            curr_index += 1.0;
            next_index += 1.0;
        }

        let new_t = (t - self.spread[curr_index as usize])
            / (self.spread[next_index as usize]
                - self.spread[curr_index as usize]);

        let r = self.colors[curr_index as usize].0 as f64
            * (1.0 - new_t)
            + self.colors[next_index as usize].0 as f64 * new_t;
        let g = self.colors[curr_index as usize].1 as f64
            * (1.0 - new_t)
            + self.colors[next_index as usize].1 as f64 * new_t;
        let b = self.colors[curr_index as usize].2 as f64
            * (1.0 - new_t)
            + self.colors[next_index as usize].2 as f64 * new_t;

        let rr = r as u8;
        let gg = g as u8;
        let bb = b as u8;
        print!("Color({rr}, {gg}, {bb})");
        println!(
            "\x1b[48;2;{};{};{}m  \x1b[0m Color({rr}, {gg}, {bb})\n",
            rr, gg, bb
        );

        Color(r as u8, g as u8, b as u8)
    }
}
