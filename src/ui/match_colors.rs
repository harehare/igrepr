use colored;
use ratatui::style::Color;

pub struct MatchColors;

const COLORS: [Color; 12] = [
    Color::Yellow,
    Color::Blue,
    Color::Green,
    Color::Red,
    Color::Magenta,
    Color::Cyan,
    Color::LightYellow,
    Color::LightBlue,
    Color::LightGreen,
    Color::LightRed,
    Color::LightMagenta,
    Color::LightCyan,
];

impl MatchColors {
    pub fn get_color(index: usize) -> Color {
        COLORS[index.checked_sub(1).unwrap_or_default() % COLORS.len()]
    }

    pub fn get_cli_color(index: usize) -> colored::Color {
        match COLORS[index.checked_sub(1).unwrap_or_default() % COLORS.len()] {
            Color::Yellow => colored::Color::Yellow,
            Color::Blue => colored::Color::Blue,
            Color::Green => colored::Color::Green,
            Color::Red => colored::Color::Red,
            Color::Magenta => colored::Color::Magenta,
            Color::Cyan => colored::Color::Cyan,
            Color::LightYellow => colored::Color::BrightYellow,
            Color::LightBlue => colored::Color::BrightBlue,
            Color::LightGreen => colored::Color::BrightGreen,
            Color::LightRed => colored::Color::BrightRed,
            Color::LightMagenta => colored::Color::BrightMagenta,
            Color::LightCyan => colored::Color::BrightCyan,
            _ => colored::Color::White,
        }
    }
}
