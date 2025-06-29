use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    text::Span,
    widgets::{Row, StatefulWidget, TableState, Widget},
};
use tui_textarea::TextArea;

use crate::{
    model::Gpa,
    ui::{message::Message, theme::THEME, traits::Component, util},
};

const TAB_HEADERS: [&str; 2] = [" Courses (q) ", " Exams (w) "];
const FILTER_PLACEHOLDER: &str = "Filter courses";

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Focus {
    #[default]
    Table,
    Filter,
}

pub struct RecordsComponent {
    selected_state: usize,
    selected_page: usize,
    focus: Focus,
    filter: TextArea<'static>,
}

impl RecordsComponent {
    pub fn new() -> Self {
        let mut ta = TextArea::default();
        // TODO: adjust cursor style + blink
        // TODO: adjust set text to not have underline
        ta.set_style(THEME.subtext);
        ta.set_cursor_style(THEME.background);
        ta.set_placeholder_text(FILTER_PLACEHOLDER);

        Self {
            selected_state: 0,
            selected_page: 0,
            focus: Focus::default(),
            filter: ta,
        }
    }

    fn apply_filter(&mut self) {
        // let mut filters = [None; 4];
        // todo!("validate filters and apply them");
        // self.gpa.apply_filters(filters);
        self.selected_state = 0;
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let h_chunks = Layout::horizontal([Constraint::Fill(1); TAB_HEADERS.len()]).split(area);

        for (i, tab) in TAB_HEADERS.into_iter().enumerate() {
            let tab_area = h_chunks[i];

            let focused = self.selected_page == i;
            if focused {
                buf.set_style(tab_area, THEME.background);
            }

            let tab_style = util::tab_style(focused);
            let line = Span::styled(tab, tab_style).into_centered_line();
            Widget::render(line, tab_area, buf);
        }
    }

    fn render_filter(&mut self, area: Rect, buf: &mut Buffer) {
        let focused = self.focus == Focus::Filter;
        let empty = self.filter.is_empty();

        if !focused {
            buf.set_style(area, THEME.background);
            self.filter
                .set_style(if empty { THEME.subtext } else { THEME.text });
        } else {
            buf.set_style(area, THEME.background_bright);
            self.filter.set_style(THEME.text);
        };

        if !empty {
            let filter_indicator = Span::styled(" ■ ", THEME.success).into_right_aligned_line();
            Widget::render(filter_indicator, area, buf);
        }

        Widget::render(&self.filter, area, buf);
    }

    fn render_table(&self, area: Rect, buf: &mut Buffer, gpa: &Gpa, is_focused: bool) {
        let visible_rows = area.height.saturating_sub(1) as usize;
        let half = visible_rows / 2;
        let total = gpa.courses.len();

        let offset = if self.selected_state <= half {
            0
        } else if self.selected_state + half >= total {
            total.saturating_sub(visible_rows)
        } else {
            self.selected_state.saturating_sub(half)
        };

        let mut state = TableState::new()
            .with_offset(offset)
            .with_selected(if is_focused {
                Some(self.selected_state)
            } else {
                None
            });

        let header_row =
            Row::new(vec!["Title", "Credits", "Semester", "Grade", "Done"]).style(THEME.subtext);
        let table = gpa
            .to_table()
            .header(header_row)
            .row_highlight_style(THEME.background);

        StatefulWidget::render(table, area, buf, &mut state);
    }
}

impl Component for RecordsComponent {
    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa, is_focused: bool) {
        let container_area = util::container_border(area, buf, "Records", None, is_focused);
        let constraints = [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ];
        let layout = Layout::vertical(constraints).spacing(1);
        let [tab_area, filter_area, table_area] = layout.areas(container_area);
        let [_, div1, div2, _] = layout.spacers(container_area);

        self.render_tabs(tab_area, buf);
        util::render_divider(div1, buf);
        self.render_filter(filter_area, buf);
        util::render_divider(div2, buf);
        match self.selected_page {
            0 => self.render_table(table_area, buf, gpa, is_focused),
            1 => {} // TODO: exams page
            _ => unreachable!(),
        }
    }

    fn on_key(&mut self, key: KeyEvent, gpa: &mut Gpa) -> anyhow::Result<Message> {
        match self.focus {
            Focus::Table => match key.code {
                KeyCode::Char('/') => {
                    self.focus = Focus::Filter;
                }
                KeyCode::Char('k' | 'K') | KeyCode::Up => {
                    self.selected_state = self.selected_state.saturating_sub(1);
                }
                KeyCode::Char('j' | 'J') | KeyCode::Down => {
                    self.selected_state = self
                        .selected_state
                        .saturating_add(1)
                        .min(gpa.courses.len().saturating_sub(1));
                }
                KeyCode::Char('q' | 'Q') => {
                    self.selected_state = 0;
                    self.selected_page = 0;
                }
                KeyCode::Char('w' | 'W') => {
                    self.selected_state = 0;
                    self.selected_page = 1;
                }
                _ => {}
            },
            Focus::Filter => match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    self.apply_filter();
                    self.focus = Focus::Table;
                }
                _ => {
                    self.filter.input(key);
                }
            },
        };

        Ok(Message::None)
    }

    fn commands(&self) -> Vec<Span> {
        match self.focus {
            Focus::Table => {
                vec![
                    Span::styled("a ", THEME.hotkey),
                    Span::styled("Add course  ", THEME.text),
                    Span::styled("d ", THEME.hotkey),
                    Span::styled("Delete course  ", THEME.text),
                    Span::styled("e ", THEME.hotkey),
                    Span::styled("Edit course  ", THEME.text),
                    Span::styled("/ ", THEME.hotkey),
                    Span::styled("Filter  ", THEME.text),
                ]
            }
            Focus::Filter => {
                vec![
                    Span::styled("esc ", THEME.hotkey),
                    Span::styled("Close filter  ", THEME.text),
                    Span::styled("enter ", THEME.hotkey),
                    Span::styled("Apply filter  ", THEME.text),
                ]
            }
        }
    }
}
