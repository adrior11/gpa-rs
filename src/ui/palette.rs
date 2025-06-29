use once_cell::sync::Lazy;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

// NOTE: just for reference of my terminal colors
// RED: #eb6f92
// YELLOW: #f6c177
// CYAN: #ebbcba
// BLUE #31748f
// GREEN: #9ccfd8
// MAGENTA: #c4a7e7
// BLACK: #26233a
// WHITE: #e0def4
// DARKGRAY: #908caa

pub static PALETTE: Lazy<Palette> = Lazy::new(Palette::default);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Palette {
    pub primary: Color,
    pub secondary: Color,
    pub primary_accent: Color,
    pub secondary_accent: Color,
    pub background: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::DarkGray,
            primary_accent: Color::Blue,
            secondary_accent: Color::Red,
            background: Color::Black,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
        }
    }
}
