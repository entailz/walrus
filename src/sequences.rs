use crate::color::Color;
use glob;
use std::fs;

// Term sequences gen
pub struct SequenceGenerator {
    colors: Vec<Color>,
    alpha: u8,
}

impl SequenceGenerator {
    pub fn new(colors: Vec<Color>, alpha: u8) -> Self {
        SequenceGenerator { colors, alpha }
    }

    fn set_special(&self, index: u16, color: &Color, _iterm_name: &str) -> String {
        // Check if we're on macOS (Darwin)
        // Kinda unnecessary but maybe somebody wants to use on apple XD
        #[cfg(target_os = "macos")]
        {
            if index == 11 && self.alpha != 100 {
                return format!("\x1b]{};[{}]{}\x1b\\", index, self.alpha, color.to_hex());
            } else if index == 708 && self.alpha != 100 {
                return format!("\x1b]{};[{}]{}\x1b\\", index, self.alpha, color.to_hex());
            } else {
                return format!("\x1b]P{}{}\x1b\\", iterm_name, color.to_hex_stripped());
            }
        }

        // For Linux
        #[cfg(not(target_os = "macos"))]
        {
            if (index == 11 || index == 708) && self.alpha != 100 {
                return format!("\x1b]{};[{}]{}\x1b\\", index, self.alpha, color.to_hex());
            } else {
                return format!("\x1b]{};{}\x1b\\", index, color.to_hex());
            }
        }
    }

    // Set color sequence for a specific index
    fn set_color(&self, index: u8, color: &Color) -> String {
        #[cfg(target_os = "macos")]
        {
            if index < 20 {
                return format!("\x1b]P{:1x}{}\x1b\\", index, color.to_hex_stripped());
            }
        }

        format!("\x1b]4;{};{}\x1b\\", index, color.to_hex())
    }

    // iTerm2 tab color
    #[cfg(target_os = "macos")]
    fn set_iterm_tab_color(&self, color: &Color) -> String {
        format!(
            "\x1b]6;1;bg;red;brightness;{}\x07\
             \x1b]6;1;bg;green;brightness;{}\x07\
             \x1b]6;1;bg;blue;brightness;{}\x07",
            color.r, color.g, color.b
        )
    }

    fn create_sequences(&self, vte_fix: bool) -> String {
        let mut sequences = String::new();

        for i in 0..16 {
            if i < self.colors.len() {
                sequences.push_str(&self.set_color(i as u8, &self.colors[i]));
            }
        }

        let background = &self.colors[0];
        let foreground = self.colors.get(15).unwrap_or(&self.colors[7]);
        let cursor = foreground;

        sequences.push_str(&self.set_special(10, foreground, "g"));
        sequences.push_str(&self.set_special(11, background, "h"));
        sequences.push_str(&self.set_special(12, cursor, "l"));
        sequences.push_str(&self.set_special(13, foreground, "l"));
        sequences.push_str(&self.set_special(17, foreground, "l"));
        sequences.push_str(&self.set_special(19, background, "l"));
        sequences.push_str(&self.set_color(232, background));
        sequences.push_str(&format!("\x1b]4;256;{}\x1b\\", foreground.to_hex()));

        if !vte_fix {
            sequences.push_str(&self.set_special(708, background, "l"));
        }

        #[cfg(target_os = "macos")]
        {
            sequences.push_str(&self.set_iterm_tab_color(background));
        }

        sequences
    }

    pub fn generate_sequences(&self, vte_fix: bool) -> String {
        self.create_sequences(vte_fix)
    }

    // Send sequences to all open terminals
    pub fn send_sequences_to_terminals(&self, vte_fix: bool) -> std::io::Result<()> {
        let sequences = self.generate_sequences(vte_fix);

        #[cfg(target_os = "macos")]
        let tty_pattern = "/dev/ttys00[0-9]*";

        #[cfg(not(target_os = "macos"))]
        let tty_pattern = "/dev/pts/[0-9]*";

        for entry in glob::glob(tty_pattern).expect("Failed to read tty pattern") {
            if let Ok(path) = entry {
                // Try to write sequences to the terminal
                if let Err(e) = fs::write(&path, &sequences) {
                    eprintln!("Failed to write to {}: {}", path.display(), e);
                }
            }
        }

        Ok(())
    }
}

