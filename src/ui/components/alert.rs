use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Widget},
};

use crate::{
    model::Gpa,
    ui::{traits::Overlay, THEME},
};

pub struct Alert {
    message: String,
    on_confirm: Option<Box<dyn FnMut()>>,
}

impl Alert {
    pub fn new(message: &str, on_confirm: Option<Box<dyn FnMut()>>) -> Self {
        Self {
            message: message.to_string(),
            on_confirm,
        }
    }
}

impl Overlay for Alert {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _gpa: &Gpa) {
        let block = Block::default()
            .title("Alert")
            .borders(Borders::ALL)
            .border_style(THEME.label.bold())
            .style(THEME.label);
        Widget::render(block, area, buf);

        let text_line = Line::styled(&self.message, THEME.text).centered();
        let text_area = area.inner(Margin::new(1, 1));
        Widget::render(text_line, text_area, buf);
    }

    fn on_key(&mut self, key: KeyEvent, _gpa: &mut Gpa) -> anyhow::Result<bool> {
        if key.code == KeyCode::Enter {
            if let Some(confirm_action) = &self.on_confirm {
                confirm_action();
            }
            return Ok(true);
        }
        Ok(false)
    }
}
