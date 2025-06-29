use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout, Rect},
    text::Span,
};

use crate::ui::{component::Component, util, Message};

pub struct UpcomingContainer {}

impl UpcomingContainer {
    pub fn new() -> Self {
        Self {}
    }

    fn render_summary(&mut self, area: Rect, buf: &mut Buffer) {}
}

impl Component for UpcomingContainer {
    fn render(&mut self, area: Rect, buf: &mut Buffer, is_focused: bool, is_dimmed: bool) {
        let container_area =
            util::container_border(area, buf, "Upcoming", None, is_focused, is_dimmed);

        // self.render_bar();
        // self.render_legend();
    }

    fn on_key(&mut self, key: KeyEvent) -> anyhow::Result<Message> {
        Ok(Message::None)
    }

    fn commands(&self) -> Vec<Span> {
        vec![]
    }
}
