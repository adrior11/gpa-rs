use ratatui::{buffer::Buffer, crossterm::event::KeyEvent, layout::Rect, widgets::Widget};
use tui_textarea::TextArea as TuiTextArea;

use crate::ui::THEME;

pub struct TextArea {
    ta: TuiTextArea<'static>,
}

impl TextArea {
    pub fn new(placeholder: &str) -> Self {
        let mut ta = TuiTextArea::default();
        ta.set_cursor_line_style(THEME.default);
        ta.set_cursor_style(THEME.background);
        ta.set_placeholder_style(THEME.text_area_placeholder);
        ta.set_placeholder_text(placeholder);
        ta.set_style(THEME.text_area);
        ta.set_tab_length(0);
        Self { ta }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self.ta, area, buf)
    }

    pub fn focused(&mut self, focused: bool) {
        if focused {
            self.ta.set_cursor_style(THEME.text_area_cursor);
        } else {
            self.ta.set_cursor_style(THEME.background);
        }
    }

    pub fn input(&mut self, key: KeyEvent) {
        self.ta.input(key);
    }

    pub fn is_empty(&self) -> bool {
        self.ta.is_empty()
    }
}
