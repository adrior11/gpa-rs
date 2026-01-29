use ratatui::{buffer::Buffer, crossterm::event::KeyEvent, layout::Rect, text::Span};

use crate::{
    model::Gpa,
    ui::{traits::Container, util},
};

pub struct UpcomingContainer {}

impl UpcomingContainer {
    pub fn new() -> Self {
        Self {}
    }

    fn render_summary(&mut self, area: Rect, buf: &mut Buffer) {}
}

impl Container for UpcomingContainer {
    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa, is_focused: bool) {
        let component_area = util::component_border(area, buf, "Upcoming", None, is_focused);

        // self.render_bar();
        // self.render_legend();
    }

    fn on_key(&mut self, key: KeyEvent, gpa: &mut Gpa) -> anyhow::Result<()> {
        Ok(())
    }

    fn commands(&self) -> Vec<Span> {
        vec![]
    }
}
