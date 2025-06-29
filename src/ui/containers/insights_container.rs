use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};

use crate::ui::{component::Component, types::Model, util, Message, THEME};

pub struct InsightsContainer {
    model: Model,
}

impl InsightsContainer {
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    fn render_summary(&mut self, area: Rect, buf: &mut Buffer) {
        let [top_area, bottom_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        let avg_str = format!("{:.2}", self.model.borrow().get_stats().avg());
        // let credits_str = format!("{}", self.model.borrow().get_stats().total());

        let left_header = Span::styled("Weighted Average", THEME.subtext).into_left_aligned_line();
        // let right_header =
        //     Span::styled("Achieved credits", THEME.subtext).into_right_aligned_line();

        let left_summary = Span::styled(avg_str, THEME.text).into_left_aligned_line();
        // let right_summary = Span::styled(credits_str, THEME.text).into_right_aligned_line();

        Widget::render(left_header, top_area, buf);
        // Widget::render(right_header, top_area, buf);
        Widget::render(left_summary, bottom_area, buf);
        // Widget::render(right_summary, bottom_area, buf);
    }

    fn render_progress_bar(&mut self, area: Rect, buf: &mut Buffer) {
        let [info_area, bar_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        let gpa = self.model.borrow();
        let stats = gpa.get_stats();
        let bar = progress_bar(stats.total(), stats.all, area.width.into());

        let achieved_line = Span::styled(format!("╭ Achieved: {}", stats.total()), THEME.text)
            .into_left_aligned_line();
        let want_line = Span::styled(
            format!("Want: {} ╮", gpa.grading_system.credit_goal),
            THEME.text,
        )
        .into_right_aligned_line();

        Widget::render(achieved_line, info_area, buf);
        Widget::render(want_line, info_area, buf);
        Widget::render(bar, bar_area, buf);
    }
}

impl Component for InsightsContainer {
    fn render(&mut self, area: Rect, buf: &mut Buffer, is_focused: bool, is_dimmed: bool) {
        let container_area =
            util::container_border(area, buf, "Insights", Some("\\"), is_focused, is_dimmed);

        let constraints = [
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Fill(1),
        ];
        let [header_area, progress_area, _] = Layout::vertical(constraints)
            .spacing(1)
            .areas(container_area);

        self.render_summary(header_area, buf);
        self.render_progress_bar(progress_area, buf);
    }

    fn on_key(&mut self, key: KeyEvent) -> anyhow::Result<Message> {
        Ok(Message::None)
    }

    fn commands(&self) -> Vec<Span> {
        vec![]
    }
}

// NOTE: highlight bars that inc/dec the average
fn progress_bar(completed: u16, total: u16, width: usize) -> Line<'static> {
    // avoid division by zero
    let filled: usize = (completed as f32 / total as f32 * width as f32).round() as usize;
    let empty: usize = width - filled;

    let completed_part = Span::styled("/".repeat(filled), THEME.progress_bar_complete);
    let remaining_part = Span::styled("/".repeat(empty), THEME.progress_bar_incomplete);

    Line::from(vec![completed_part, remaining_part])
}
