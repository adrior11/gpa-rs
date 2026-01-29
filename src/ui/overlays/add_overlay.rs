use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
};

use crate::{
    model::Gpa,
    ui::{
        components::{Form, FormInput},
        traits::Overlay,
        util,
    },
};

const PLACEHOLDERS: [&str; 4] = ["My Course", "0", "1", "0.0"];

pub struct AddOverlay {
    form: Form,
}

impl Default for AddOverlay {
    fn default() -> Self {
        let fields = vec![
            FormInput::new("Title", PLACEHOLDERS[0]),
            FormInput::new("Credits", PLACEHOLDERS[1]),
            FormInput::new("Semester", PLACEHOLDERS[2]),
            FormInput::new("Grade", PLACEHOLDERS[3]),
        ];

        Self {
            form: Form::new(fields),
        }
    }
}

impl Overlay for AddOverlay {
    fn render(&mut self, area: Rect, buf: &mut Buffer, _gpa: &Gpa) {
        let popup_area = util::popup_border(area, buf, "New Course");
        self.form.render(popup_area, buf);
    }

    fn on_key(&mut self, key: KeyEvent, gpa: &mut Gpa) -> anyhow::Result<bool> {
        match key.code {
            KeyCode::Esc => {
                return Ok(true);
            }
            KeyCode::Tab => {
                self.form.focus_next();
            }
            KeyCode::BackTab => {
                self.form.focus_prev();
            }
            KeyCode::Enter => {
                self.form.submit(gpa)?;
                return Ok(true);
            }
            _ => {}
        }
        Ok(false)
    }
}
