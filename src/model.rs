use std::fs;

use colored::{ColoredString, Colorize};
use serde::Deserialize;

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Debug)]
pub struct GPA {
    pub target_ects: u8,
    pub target_grade: f32,
    pub lectures: Vec<Lecture>,
}

#[derive(Deserialize, Debug)]
pub struct Lecture {
    pub title: String,
    pub ects: u8,
    pub semester: u8,
    pub grade: Option<f32>,
}

impl GPA {
    pub fn from_file(path: &str) -> Self {
        let content: String = fs::read_to_string(path).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    pub fn calc_avg(&self) {
        let mut weighted_grade: f32 = 0.0;
        let mut combined_ects: u8 = 0;
        for lecture in self.lectures.iter() {
            if let Some(grade) = lecture.grade {
                weighted_grade += grade * lecture.ects as f32;
                combined_ects += lecture.ects;
            }
        }
        let avg_grade = weighted_grade / combined_ects as f32;
        let avg_grade_str = match avg_grade {
            avg_grade if avg_grade <= self.target_grade => format!("{:.2}", avg_grade).green(),
            _ => format!("{:.2}", avg_grade).red(),
        };
        println!(
            "Average grade: {}\nAchieved ECTS: {} o/ {}",
            avg_grade_str, combined_ects, self.target_ects
        );
    }

    pub fn ects_in_file(&self) {
        let combined_ects: u8 = self.lectures.iter().map(|l| l.ects).sum();
        println!(
            "You've added courses with {} ects combined, targeting {} ects.",
            combined_ects, self.target_ects
        );
    }

    pub fn semester(&self, semester: u8) {
        println!("SEMESTER: {}", semester);
        let mut combined_ects: u8 = 0;
        for lecture in self.lectures.iter() {
            if lecture.semester == semester {
                let grade: ColoredString = match lecture.grade {
                    Some(grade) => {
                        if grade <= self.target_grade {
                            format!("{:.1}", grade).green()
                        } else {
                            format!("{:.1}", grade).red()
                        }
                    }
                    None => "-".to_string().normal(),
                };
                combined_ects += lecture.ects;
                println!("   {} ({} ects): {}", lecture.title, lecture.ects, grade)
            }
        }
        println!("ECTS: {}", combined_ects);
    }
}
