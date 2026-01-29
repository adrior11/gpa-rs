use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Margin, Rect},
    text::Span,
    widgets::{
        calendar::{CalendarEventStore, Monthly},
        Widget,
    },
};
use time::{Date, OffsetDateTime};

use crate::{
    model::Gpa,
    ui::{traits::Container, util, THEME},
};

pub struct CalendarContainer {
    date: Date,
}

// TODO:
fn next_month(date: Date) -> Date {
    todo!()
}

fn prev_month(date: Date) -> Date {
    todo!()
}

impl CalendarContainer {
    pub fn new() -> Self {
        Self {
            date: OffsetDateTime::now_utc().date(),
        }
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        // TODO: show context of selected month (e.g, last month, next month, 12 Jan - 18 Jan)
    }

    fn render_calendar(&mut self, area: Rect, buf: &mut Buffer) {
        Monthly::new(
            self.date,
            CalendarEventStore::today(THEME.calendar_current_day),
        )
        .show_surrounding(THEME.calendar_surrounding_days)
        .show_weekdays_header(THEME.calendar_weekdays_header)
        .default_style(THEME.calendar_days)
        .render(area.inner(Margin::new(2, 0)), buf);
    }
}

impl Container for CalendarContainer {
    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa, is_focused: bool) {
        let component_area =
            util::component_border(area, buf, "Calendar", Some("← . →"), is_focused);
        let constraints = [Constraint::Length(1), Constraint::Fill(1)];
        let [header_area, calendar_area] = Layout::vertical(constraints)
            .spacing(1)
            .areas(component_area);

        self.render_header(header_area, buf);
        self.render_calendar(calendar_area, buf);
    }

    fn on_key(&mut self, key: KeyEvent, gpa: &mut Gpa) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Char('.') => {
                self.date = OffsetDateTime::now_utc().date();
            }
            KeyCode::Left => {
                self.date = prev_month(self.date);
            }
            KeyCode::Right => {
                self.date = next_month(self.date);
            }
            _ => {}
        }
        Ok(())
    }

    fn commands(&self) -> Vec<Span> {
        vec![
            Span::styled("←/→", THEME.hotkey),
            Span::styled("Prev/Next month  ", THEME.text),
            Span::styled(".", THEME.hotkey),
            Span::styled("Goto current month  ", THEME.text),
        ]
    }
}
