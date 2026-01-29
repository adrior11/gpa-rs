// TODO: for editing utilize selection in ta, for quick deleting
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Span,
    widgets::Widget,
};

use crate::{model::Gpa, ui::THEME};

use super::TextArea;

pub struct FormInput {
    label: String,
    text_area: TextArea,
}

impl FormInput {
    pub fn new(label: &str, placeholder: &str) -> Self {
        Self {
            label: label.to_string(),
            text_area: TextArea::new(placeholder),
        }
    }
}

pub struct Form {
    fields: Vec<FormInput>,
    completed: bool,
    focused: usize,
    error: Option<String>,
}

impl Form {
    pub fn new(fields: Vec<FormInput>) -> Self {
        Self {
            fields,
            completed: false,
            focused: 0,
            error: None,
        }
    }

    pub fn submit(&mut self, _gpa: &mut Gpa) -> anyhow::Result<()> {
        // TODO:
        Ok(())
    }

    pub fn focus_next(&mut self) {
        self.focused = (self.focused + 1) % self.fields.len();
    }

    pub fn focus_prev(&mut self) {
        self.focused = (self.focused + self.fields.len() - 1) % self.fields.len();
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let length = self.fields.len();
        let rows = Layout::vertical([Constraint::Length(2)].repeat(length))
            .spacing(1)
            .split(area);

        for (idx, row) in rows.iter().enumerate() {
            let [label_area, ta_area] =
                Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(*row);

            let field = &mut self.fields[idx];
            field.text_area.focused(idx == self.focused);
            field.text_area.render(ta_area, buf);

            let label = Span::styled(&field.label, THEME.label).into_left_aligned_line();
            Widget::render(label, label_area, buf);
        }
    }
}
