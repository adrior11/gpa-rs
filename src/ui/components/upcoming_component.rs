use ratatui::{buffer::Buffer, crossterm::event::KeyEvent, layout::Rect, text::Span};

use crate::{
    model::Gpa,
    ui::{traits::Component, util, Message},
};

pub struct UpcomingComponent {}

impl UpcomingComponent {
    pub fn new() -> Self {
        Self {}
    }

    fn render_summary(&mut self, area: Rect, buf: &mut Buffer) {}
}

impl Component for UpcomingComponent {
    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa, is_focused: bool) {
        let container_area = util::container_border(area, buf, "Upcoming", None, is_focused);

        // self.render_bar();
        // self.render_legend();
    }

    fn on_key(&mut self, key: KeyEvent, gpa: &mut Gpa) -> anyhow::Result<Message> {
        Ok(Message::None)
    }

    fn commands(&self) -> Vec<Span> {
        vec![]
    }
}
