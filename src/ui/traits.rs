use ratatui::{buffer::Buffer, crossterm::event::KeyEvent, layout::Rect, text::Span};

use crate::model::Gpa;

use super::message::Message;

// TODO: add function for focus next/previous child component returning boolean
pub trait Component {
    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa, is_focused: bool);
    fn on_key(&mut self, key: KeyEvent, gpa: &mut Gpa) -> anyhow::Result<Message>;
    fn commands(&self) -> Vec<Span>;
}

pub trait Page {
    fn name(&self) -> &str;
    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa);
    fn on_key(&mut self, key: KeyEvent) -> anyhow::Result<Message>;
    fn commands(&self) -> Vec<Span>;
}
