use once_cell::sync::Lazy;
use ratatui::style::{Modifier, Style};
use serde::{Deserialize, Serialize};

use super::palette::PALETTE;

pub static THEME: Lazy<Theme> = Lazy::new(Theme::default);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Theme {
    pub header: Style,
    pub version: Style,

    pub text: Style,
    pub subtext: Style,
    pub footer: Style,

    pub hotkey: Style,
    pub divider: Style,

    pub background: Style,
    pub background_bright: Style,

    pub progress_bar_complete: Style,
    pub progress_bar_incomplete: Style,

    pub calendar_days: Style,
    pub calendar_current_day: Style,
    pub calendar_surrounding_days: Style,
    pub calendar_weekdays_header: Style,

    pub success: Style,
    pub warning: Style,
    pub error: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            header: Style::new()
                .fg(PALETTE.primary_accent)
                .add_modifier(Modifier::BOLD),
            version: Style::new()
                .fg(PALETTE.primary_accent)
                .add_modifier(Modifier::DIM),
            text: Style::new().fg(PALETTE.primary),
            subtext: Style::new().fg(PALETTE.secondary),
            footer: Style::new().fg(PALETTE.secondary_accent),
            hotkey: Style::new()
                .fg(PALETTE.primary_accent)
                .add_modifier(Modifier::BOLD),
            divider: Style::new().fg(PALETTE.background),
            background: Style::new().bg(PALETTE.background),
            background_bright: Style::new().bg(PALETTE.secondary),
            progress_bar_complete: Style::new().fg(PALETTE.secondary_accent),
            progress_bar_incomplete: Style::new().fg(PALETTE.secondary),
            calendar_days: Style::new()
                .fg(PALETTE.primary)
                .add_modifier(Modifier::BOLD),
            calendar_current_day: Style::new()
                .fg(PALETTE.primary_accent)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
            calendar_surrounding_days: Style::new()
                .fg(PALETTE.secondary)
                .add_modifier(Modifier::DIM),
            calendar_weekdays_header: Style::new()
                .fg(PALETTE.secondary)
                .add_modifier(Modifier::ITALIC),
            success: Style::new().fg(PALETTE.success),
            warning: Style::new().fg(PALETTE.warning),
            error: Style::new().fg(PALETTE.error),
        }
    }
}

impl Theme {
    pub fn border_style(&self, is_focused: bool, is_dimmed: bool) -> Style {
        let s_accent = Style::new().fg(PALETTE.primary_accent);
        let s_base = Style::new().fg(PALETTE.secondary);
        match (is_focused, is_dimmed) {
            (true, true) => s_accent.add_modifier(Modifier::DIM),
            (true, false) => s_accent,
            (false, true) => s_base.add_modifier(Modifier::DIM),
            (false, false) => s_base,
        }
    }

    pub fn tab_style(&self, selected: bool) -> Style {
        let s = Style::new().fg(PALETTE.primary_accent);
        match selected {
            true => s.add_modifier(Modifier::BOLD),
            false => s.add_modifier(Modifier::DIM),
        }
    }
}
