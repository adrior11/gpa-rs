use ratatui::{buffer::Buffer, crossterm::event::KeyEvent, layout::Rect, text::Span};

use super::message::Message;

// TODO: add function for focus next/previous child component returning boolean
// NOTE: would a update function be useful?
pub trait Component {
    fn render(&mut self, area: Rect, buf: &mut Buffer, is_focused: bool, is_dimmed: bool);
    fn on_key(&mut self, key: KeyEvent) -> anyhow::Result<Message>;
    fn commands(&self) -> Vec<Span>;
}
