// TODO: command pattern for key handling
// TODO: dynamic layout
// TODO: if there are no lectures show welcome container instead of records container
// TODO: dim buf on float & dim rework

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Margin, Rect},
    text::{Line, Span},
    widgets::Widget,
};

use crate::{file_util, model::Gpa};

use super::{message::Message, pages::HomePage, theme::THEME, traits::Page};

pub struct App {
    gpa: Gpa,
    pages: Vec<Box<dyn Page>>,
    focused: usize,
}

impl App {
    pub fn new(gpa: Gpa) -> Self {
        Self {
            gpa,
            pages: vec![Box::new(HomePage::new())],
            focused: 0,
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) -> anyhow::Result<Message> {
        // application-wide key handling
        #[allow(clippy::single_match)]
        match key.code {
            KeyCode::Char('q' | 'Q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.gpa.save(&file_util::get_config_path())?;
                return Ok(Message::Quit);
            }
            _ => {}
        };

        // container-specific key handling
        self.pages[self.focused].on_key(key)
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let header = Line::from(vec![
            Span::styled("GPA-RS ", THEME.header),
            Span::styled(format!("v{}", env!("CARGO_PKG_VERSION")), THEME.version),
        ])
        .left_aligned();

        Widget::render(header, area.inner(Margin::new(2, 0)), buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let container_cmds = self.pages[self.focused].commands();
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
        self.pages[self.focused].render(container_area, buf, &self.gpa);
        self.render_footer(footer_area, buf);
    }
}
