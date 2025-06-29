use ratatui::widgets::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ui::THEME;

use super::{exam::Exam, Progress};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Course {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub title: String,
    pub credits: u16,
    pub semester: u16,
    pub grade: Option<f32>,
    pub completed: bool, // NOTE: rethink state handling
    #[serde(default = "Vec::new")]
    pub exams: Vec<Exam>,
    pub progress: Option<Progress>,
}

impl Course {
    pub fn new(
        title: String,
        credits: u16,
        semester: u16,
        grade: Option<f32>,
        completed: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            credits,
            semester,
            grade,
            completed,
            exams: Vec::new(),
            progress: None,
        }
    }

    pub fn upcoming_exams(&self) -> Vec<&Exam> {
        self.exams
            .iter()
            .filter(|exam| exam.date > time::OffsetDateTime::now_utc().date())
            .collect()
    }

    fn grade_str(&self) -> String {
        if let Some(g) = self.grade {
            if g.is_nan() {
                "-".to_string()
            } else {
                format!("{:.1}", g)
            }
        } else {
            "-".to_string()
        }
    }

    fn completed_str(&self) -> String {
        if self.completed { "✓" } else { "" }.to_string()
    }

    pub fn to_row(&self) -> Row {
        Row::new(vec![
            format!("{:<40}", self.title),
            format!("{:8}", self.credits),
            format!("{:8}", self.semester),
            format!("{:8}", self.grade_str()),
            format!("{:8}", self.completed_str()),
        ])
        .style(THEME.text)
    }
}
