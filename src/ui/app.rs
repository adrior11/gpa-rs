// TODO: command pattern for key handling
// TODO: dynamic layout
// TODO: if there are no lectures show welcome container instead of records container
// TODO: dim buf on float & dim rework
use std::{cell::RefCell, rc::Rc};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Margin, Rect},
    text::{Line, Span},
    widgets::Widget,
};

use crate::model::Gpa;

use super::{
    containers::{CalendarContainer, InsightsContainer, RecordsContainer, UpcomingContainer},
    message::Message,
    pane::{Pane, PaneId},
    theme::THEME,
};

// NOTE: app must persists gpa in order to save changes
pub struct App {
    panes: Vec<Pane>,
    focused: PaneId,
}

impl App {
    pub fn new(gpa: Gpa) -> Self {
        let model = Rc::new(RefCell::new(gpa));
        Self {
            panes: vec![
                Pane::new(PaneId::Upcoming, UpcomingContainer::new()),
                Pane::new(PaneId::Calendar, CalendarContainer::new()),
                Pane::new(PaneId::Insights, InsightsContainer::new(model.clone())),
                Pane::new(PaneId::Courses, RecordsContainer::new(model)),
            ],
            focused: PaneId::Courses,
        }
    }

    fn get_pane(&self, id: PaneId) -> &Pane {
        self.panes
            .iter()
            .find(|pane| pane.id == id)
            .expect("Pane not found")
    }

    fn get_pane_mut(&mut self, id: PaneId) -> &mut Pane {
        self.panes
            .iter_mut()
            .find(|pane| pane.id == id)
            .expect("Pane not found")
    }

    pub fn on_key(&mut self, key: KeyEvent) -> anyhow::Result<Message> {
        // application-wide key handling
        #[allow(clippy::single_match)]
        match key.code {
            KeyCode::Char('q' | 'Q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return Ok(Message::Quit)
            }
            _ => {}
        };

        // container-specific key handling
        let pane = self.get_pane_mut(self.focused);
        pane.container.on_key(key)
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let header = Line::from(vec![
            Span::styled("GPA-RS ", THEME.header),
            Span::styled(format!("v{}", env!("CARGO_PKG_VERSION")), THEME.version),
        ])
        .left_aligned();

        Widget::render(header, area.inner(Margin::new(2, 0)), buf);
    }

    fn render_containers(&mut self, area: Rect, buf: &mut Buffer) {
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

        for (pane, pane_area) in self
            .panes
            .iter_mut()
            // zip the panes with their respective areas in order of declaration
            .zip([upcoming_area, calendar_area, insights_area, records_area])
        {
            let is_focused = self.focused == pane.id;
            pane.container.render(pane_area, buf, is_focused, false);
        }
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let container_cmds = self.get_pane(self.focused).container.commands();
        let generic_cmds = [
            Span::styled("^q ", THEME.hotkey),
            Span::styled("Quit  ", THEME.text),
        ];

        let footer_left = Line::from(
            container_cmds
                .iter()
                .chain(generic_cmds.iter())
                .cloned()
                .collect::<Vec<Span>>(),
        )
        .left_aligned();
        let footer_right = Line::from(vec![
            Span::styled("|", THEME.subtext),
            Span::styled(" ?", THEME.hotkey),
            Span::styled(" Help", THEME.text),
        ])
        .right_aligned();

        buf.set_style(area, THEME.background);
        let cmd_area = area.inner(Margin::new(1, 0));
        Widget::render(footer_left, cmd_area, buf);
        Widget::render(footer_right, cmd_area, buf);
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ];
        let [header_area, container_area, footer_area] = Layout::vertical(constraints)
            .constraints(constraints)
            .areas(area);

        self.render_header(header_area, buf);
        self.render_containers(container_area, buf);
        self.render_footer(footer_area, buf);
    }
}
