#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn to_hex_stripped(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    #[allow(dead_code)]
    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    #[allow(dead_code)]
    pub fn to_hsl(&self) -> String {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let diff = max - min;

        let l = (max + min) / 2.0;

        if diff == 0.0 {
            return format!("hsl(0, 0%, {}%)", (l * 100.0) as i32);
        }

        let s = if l > 0.5 {
            diff / (2.0 - max - min)
        } else {
            diff / (max + min)
        };

        let h = if max == r {
            (g - b) / diff + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / diff + 2.0
        } else {
            (r - g) / diff + 4.0
        };

        let h = (h * 60.0) as i32;
        let s = (s * 100.0) as i32;
        let l = (l * 100.0) as i32;

        format!("hsl({}, {}%, {}%)", h, s, l)
    }

    // Convert RGB to YIQ for luminance-based sorting (like pywal)
    pub fn to_yiq(&self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        0.299 * r + 0.587 * g + 0.114 * b
    }

    pub fn darken(&self, amount: f32) -> Color {
        let r = (self.r as f32 * (1.0 - amount)) as u8;
        let g = (self.g as f32 * (1.0 - amount)) as u8;
        let b = (self.b as f32 * (1.0 - amount)) as u8;
        Color::new(r, g, b)
    }

    pub fn lighten(&self, amount: f32) -> Color {
        let r = (self.r as f32 + (255.0 - self.r as f32) * amount) as u8;
        let g = (self.g as f32 + (255.0 - self.g as f32) * amount) as u8;
        let b = (self.b as f32 + (255.0 - self.b as f32) * amount) as u8;
        Color::new(r, g, b)
    }

    pub fn saturate(&self, amount: f32) -> Color {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let diff = max - min;

        let l = (max + min) / 2.0;

        if diff == 0.0 {
            return *self; // Grayscale
        }

        let _s = if l > 0.5 {
            diff / (2.0 - max - min)
        } else {
            diff / (max + min)
        };

        let h = if max == r {
            (g - b) / diff + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / diff + 2.0
        } else {
            (r - g) / diff + 4.0
        };

        let new_s = amount.clamp(0.0, 1.0);

        let c = (1.0 - (2.0 * l - 1.0).abs()) * new_s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r_prime, g_prime, b_prime) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r = ((r_prime + m) * 255.0) as u8;
        let g = ((g_prime + m) * 255.0) as u8;
        let b = ((b_prime + m) * 255.0) as u8;

        Color::new(r, g, b)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WeightedColor {
    pub color: Color,
    pub count: u32,
}

impl WeightedColor {
    #[allow(dead_code)]
    pub fn new(color: Color, count: u32) -> Self {
        WeightedColor { color, count }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 128, 0);
        assert_eq!(color.to_hex(), "#ff8000");
    }

    #[test]
    fn test_yiq_conversion() {
        let dark_color = Color::new(50, 50, 50);
        let light_color = Color::new(200, 200, 200);

        assert!(dark_color.to_yiq() < light_color.to_yiq());
    }
}