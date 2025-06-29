// TODO: command pattern for key handling
// TODO: dim buf on float & dim rework

use std::{io::Stdout, time::Duration};

use anyhow::Ok;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Margin, Rect},
    prelude::CrosstermBackend,
    text::{Line, Span},
    widgets::Widget,
    Terminal,
};

use crate::{file_util, model::Gpa};

use super::{pages::HomePage, theme::THEME, traits::Page};

pub struct App {
    running: bool,
    pages: Vec<Box<dyn Page>>,
    focused: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            pages: vec![Box::new(HomePage::new())],
            focused: 0,
        }
    }
}

impl App {
    pub fn run(
        &mut self,
        gpa: &mut Gpa,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> anyhow::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();
                self.render(area, buf, gpa);
            })?;

            if let Some(key) = handle_keypress()? {
                self.on_key(key, gpa)?;
            }
        }
        Ok(())
    }

    fn on_key(&mut self, key: KeyEvent, gpa: &mut Gpa) -> anyhow::Result<()> {
        // application-wide key handling
        #[allow(clippy::single_match)]
        match key.code {
            KeyCode::Char('q' | 'Q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                gpa.save(&file_util::get_config_path())?;
                self.running = false;
            }
            _ => {}
        };

        // page-specific key handling
        self.pages[self.focused].on_key(key, gpa)
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer, gpa: &Gpa) {
        let constraints = [
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ];
        let [header_area, page_area, footer_area] = Layout::vertical(constraints)
            .constraints(constraints)
            .areas(area);

        self.render_header(header_area, buf);
        self.pages[self.focused].render(page_area, buf, gpa);
        self.render_footer(footer_area, buf);
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
        let page_cmds = self.pages[self.focused].commands();
        let generic_cmds = [
            Span::styled("^q ", THEME.hotkey),
            Span::styled("Quit  ", THEME.text),
        ];

        let footer_left = Line::from(
            page_cmds
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
}

fn handle_keypress() -> anyhow::Result<Option<KeyEvent>> {
    let keypress = if event::poll(Duration::from_millis(500))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => Some(key),
            _ => None,
        }
    } else {
        None
    };
    Ok(keypress)
}
