use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout, Rect},
    text::Span,
};

use crate::{
    model::Gpa,
    ui::{
        components::{CalendarComponent, InsightsComponent, RecordsComponent, UpcomingComponent},
        traits::{Component, Page},
        Message,
    },
};

pub struct HomePage {
    components: Vec<Box<dyn Component>>,
    focused: usize,
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            components: vec![
                Box::new(UpcomingComponent::new()),
                Box::new(CalendarComponent::new()),
                Box::new(InsightsComponent::new()),
                Box::new(RecordsComponent::new()),
            ],
            focused: 3,
        }
    }
}

impl Page for HomePage {
    fn name(&self) -> &str {
        "Home"
    }
    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa) {
        let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(70)])
            .horizontal_margin(2)
            .vertical_margin(1)
            .areas(area);

        let [left_top, insights_area] =
            Layout::vertical([Constraint::Length(11), Constraint::Fill(1)]).areas(left);

        let [upcoming_area, calendar_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(30)]).areas(left_top);

        let [_, records_area] =
            Layout::vertical([Constraint::Length(5), Constraint::Fill(1)]).areas(right);

        for (idx, (comp, comp_area)) in self
            .components
            .iter_mut()
            // zip the components with their respective areas in order of declaration
            .zip([upcoming_area, calendar_area, insights_area, records_area])
            .enumerate()
        {
            let is_focused = self.focused == idx;
            comp.render(comp_area, buf, gpa, is_focused);
        }
    }

    fn on_key(&mut self, key: KeyEvent) -> anyhow::Result<Message> {
        match key.code {
            // focus next component
            ratatui::crossterm::event::KeyCode::Tab => {
                self.focused = (self.focused + 1) % self.components.len();
                Ok(Message::None)
            }
            // focus previous component
            ratatui::crossterm::event::KeyCode::BackTab => {
                self.focused = (self.focused + self.components.len() - 1) % self.components.len();
                Ok(Message::None)
            }
            _ => self.components[self.focused].on_key(key, &mut Gpa::default()),
        }
    }

    fn commands(&self) -> Vec<Span> {
        self.components[self.focused].commands()
    }
}
