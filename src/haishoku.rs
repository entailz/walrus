use crate::color::Color;
use image;
use std::collections::HashMap;

pub struct Haishoku {
    pub dominant: Option<Color>,
    pub palette: Vec<(f32, Color)>, // (percentage, color)
}

impl Haishoku {
    pub fn new() -> Self {
        Haishoku {
            dominant: None,
            palette: Vec::new(),
        }
    }

    pub fn load_haishoku(&mut self, image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let colors_mean = self.get_colors_mean(image_path)?;

        self.palette = self.calculate_palette(&colors_mean);

        self.dominant = self.calculate_dominant(&colors_mean);

        Ok(())
    }

    fn get_colors(
        &self,
        image_path: &str,
    ) -> Result<Vec<(u32, Color)>, Box<dyn std::error::Error>> {
        let img = image::open(image_path)?;
        let rgb_img = img.to_rgb8();

        let thumbnail = image::imageops::thumbnail(&rgb_img, 256, 256);

        let mut color_counts: HashMap<Color, u32> = HashMap::new();

        for pixel in thumbnail.pixels() {
            let color = Color::new(pixel[0], pixel[1], pixel[2]);
            *color_counts.entry(color).or_insert(0) += 1;
        }

        let image_colors: Vec<(u32, Color)> = color_counts
            .into_iter()
            .map(|(color, count)| (count, color))
            .collect();

        Ok(image_colors)
    }

    fn sort_by_rgb(&self, image_colors: Vec<(u32, Color)>) -> Vec<(u32, Color)> {
        let mut sorted = image_colors;
        sorted.sort_by(|a, b| {
            a.1.r
                .cmp(&b.1.r)
                .then_with(|| a.1.g.cmp(&b.1.g))
                .then_with(|| a.1.b.cmp(&b.1.b))
        });
        sorted
    }

    fn group_by_accuracy(
        &self,
        sorted_colors: Vec<(u32, Color)>,
    ) -> Vec<Vec<Vec<Vec<(u32, Color)>>>> {
        let mut groups = Vec::new();
        for _ in 0..3 {
            let mut layer = Vec::new();
            for _ in 0..3 {
                let mut row = Vec::new();
                for _ in 0..3 {
                    row.push(Vec::new());
                }
                layer.push(row);
            }
            groups.push(layer);
        }

        for (count, color) in sorted_colors {
            let r_group = (color.r as f32 / 255.0 * 2.99) as usize;
            let g_group = (color.g as f32 / 255.0 * 2.99) as usize;
            let b_group = (color.b as f32 / 255.0 * 2.99) as usize;

            groups[r_group][g_group][b_group].push((count, color));
        }

        groups
    }

    pub fn get_weighted_mean(&self, grouped_colors: Vec<(u32, Color)>) -> (f32, Color) {
        if grouped_colors.is_empty() {
            return (0.0, Color::new(0, 0, 0));
        }

        let mut total_count = 0u32;
        let mut weighted_r = 0f32;
        let mut weighted_g = 0f32;
        let mut weighted_b = 0f32;

        for (count, color) in &grouped_colors {
            total_count += count;
            weighted_r += (*count as f32) * (color.r as f32);
            weighted_g += (*count as f32) * (color.g as f32);
            weighted_b += (*count as f32) * (color.b as f32);
        }

        let mean_r = (weighted_r / total_count as f32) as u8;
        let mean_g = (weighted_g / total_count as f32) as u8;
        let mean_b = (weighted_b / total_count as f32) as u8;

        (total_count as f32, Color::new(mean_r, mean_g, mean_b))
    }

    fn get_colors_mean(
        &self,
        image_path: &str,
    ) -> Result<Vec<(f32, Color)>, Box<dyn std::error::Error>> {
        let image_colors = self.get_colors(image_path)?;

        let sorted_image_colors = self.sort_by_rgb(image_colors);

        let grouped_image_colors = self.group_by_accuracy(sorted_image_colors);

        let mut colors_mean = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let grouped_color = &grouped_image_colors[i][j][k];
                    if !grouped_color.is_empty() {
                        let color_mean = self.get_weighted_mean(grouped_color.clone());
                        colors_mean.push(color_mean);
                    }
                }
            }
        }

        colors_mean.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        if colors_mean.len() > 8 {
            colors_mean.truncate(8);
        }

        colors_mean = self.filter_similar_colors(colors_mean);

        Ok(colors_mean)
    }

    fn filter_similar_colors(&self, colors: Vec<(f32, Color)>) -> Vec<(f32, Color)> {
        let mut filtered = Vec::new();

        for (weight, color) in colors {
            let mut is_similar = false;

            for (_, existing_color) in &filtered {
                if self.color_distance(&color, existing_color) < 30.0 {
                    is_similar = true;
                    break;
                }
            }

            if !is_similar {
                filtered.push((weight, color));
            }
        }

        filtered
    }

    fn color_distance(&self, color1: &Color, color2: &Color) -> f32 {
        // Calculate Euclidean distance in RGB space
        let dr = (color1.r as f32 - color2.r as f32).powi(2);
        let dg = (color1.g as f32 - color2.g as f32).powi(2);
        let db = (color1.b as f32 - color2.b as f32).powi(2);
        (dr + dg + db).sqrt()
    }

    fn calculate_dominant(&self, colors_mean: &[(f32, Color)]) -> Option<Color> {
        if colors_mean.is_empty() {
            return None;
        }

        let mut sorted_colors = colors_mean.to_vec();
        sorted_colors.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        Some(sorted_colors[0].1)
    }

    fn calculate_palette(&self, colors_mean: &[(f32, Color)]) -> Vec<(f32, Color)> {
        if colors_mean.is_empty() {
            return Vec::new();
        }

        let count_sum: f32 = colors_mean.iter().map(|(count, _)| count).sum();

        let mut palette = Vec::new();
        for (count, color) in colors_mean {
            let percentage = count / count_sum;
            palette.push((percentage, *color));
        }

        palette
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_mean() {
        let haishoku = Haishoku::new();
        let colors = vec![
            (10, Color::new(255, 0, 0)),
            (5, Color::new(0, 255, 0)),
            (3, Color::new(0, 0, 255)),
        ];

        let (weight, mean_color) = haishoku.get_weighted_mean(colors);
        assert_eq!(weight, 18.0);
        assert!(mean_color.r > mean_color.g && mean_color.r > mean_color.b);
    }

    #[test]
    fn test_color_grouping() {
        let haishoku = Haishoku::new();
        let colors = vec![
            (1, Color::new(255, 255, 255)), // Should go to group [2][2][2]
            (1, Color::new(0, 0, 0)),       // Should go to group [0][0][0]
            (1, Color::new(127, 127, 127)), // Should go to group [1][1][1]
        ];

        let groups = haishoku.group_by_accuracy(colors);
        assert!(!groups[0][0][0].is_empty());
        assert!(!groups[1][1][1].is_empty());
        assert!(!groups[2][2][2].is_empty());
    }

    #[test]
    fn test_color_distance() {
        let haishoku = Haishoku::new();
        let color1 = Color::new(255, 0, 0);
        let color2 = Color::new(255, 0, 0);
        let color3 = Color::new(0, 255, 0);

        assert_eq!(haishoku.color_distance(&color1, &color2), 0.0);
        assert!(haishoku.color_distance(&color1, &color3) > 0.0);
    }
}

