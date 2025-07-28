use crate::color::Color;
use crate::haishoku::Haishoku;

pub struct PywalGenerator {
    haishoku: Haishoku,
}

impl PywalGenerator {
    pub fn new() -> Self {
        PywalGenerator {
            haishoku: Haishoku::new(),
        }
    }

    fn gen_colors(&mut self, image_path: &str) -> Result<Vec<Color>, Box<dyn std::error::Error>> {
        self.haishoku.load_haishoku(image_path)?;

        let colors: Vec<Color> = self
            .haishoku
            .palette
            .iter()
            .map(|(_, color)| *color)
            .collect();
        Ok(colors)
    }

    fn adjust(&self, mut cols: Vec<Color>, light: bool) -> Vec<Color> {
        // Sort by YIQ (luminance) like pywal does
        cols.sort_by(|a, b| a.to_yiq().partial_cmp(&b.to_yiq()).unwrap());

        let mut raw_colors = Vec::new();
        raw_colors.extend_from_slice(&cols);
        raw_colors.extend_from_slice(&cols);

        while raw_colors.len() < 16 {
            raw_colors.extend_from_slice(&cols);
        }
        raw_colors.truncate(16);

        if !raw_colors.is_empty() {
            raw_colors[0] = raw_colors[0].lighten(0.40);
        }

        self.generic_adjust(raw_colors, light)
    }

    fn generic_adjust(&self, mut colors: Vec<Color>, light: bool) -> Vec<Color> {
        if light {
            for i in 0..colors.len() {
                colors[i] = colors[i].saturate(0.60);
                colors[i] = colors[i].darken(0.5);
            }

            colors[0] = colors[0].lighten(0.95);
            colors[7] = colors[0].darken(0.75);
            colors[8] = colors[0].darken(0.25);
            colors[15] = colors[7];
        } else {
            colors[0] = colors[0].darken(0.80);
            colors[7] = colors[0].lighten(0.75);
            colors[8] = colors[0].lighten(0.25);
            colors[15] = colors[7];
        }

        colors
    }

    fn saturate_colors(&self, mut colors: Vec<Color>, amount: f32) -> Vec<Color> {
        if amount > 0.0 && amount <= 1.0 {
            for i in 0..colors.len() {
                if i != 0 && i != 7 && i != 8 && i != 15 {
                    colors[i] = colors[i].saturate(amount);
                }
            }
        }
        colors
    }

    pub fn generate_from_image(
        &mut self,
        image_path: &str,
        saturation_factor: f32,
        light: bool,
    ) -> Result<Vec<Color>, Box<dyn std::error::Error>> {
        let cols = self.gen_colors(image_path)?;

        if cols.is_empty() {
            return Err("No colors found in image".into());
        }

        let mut adjusted_colors = self.adjust(cols, light);

        if saturation_factor != 1.0 {
            adjusted_colors = self.saturate_colors(adjusted_colors, saturation_factor);
        }

        Ok(adjusted_colors)
    }
}

