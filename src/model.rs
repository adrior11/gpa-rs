use colored::Colorize;
use serde::Deserialize;

use crate::cli::{FilterBy, SortBy};

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Debug)]
pub struct GPA {
    pub target_ects: u8,
    pub target_grade: f32,
    pub lectures: Vec<Lecture>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Lecture {
    pub title: String,
    pub ects: u8,
    pub semester: u8,
    pub grade: Option<f32>,
    pub completed: bool,
}

impl GPA {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content: String = std::fs::read_to_string(path)?;
        let gpa = serde_json::from_str(&content)?;
        Ok(gpa)
    }

    pub fn calc_avg(&self) {
        let mut weighted = 0.0;
        let mut graded_ects = 0u8;
        let mut points_only_ects = 0u8;

        for lec in &self.lectures {
            match lec.grade {
                Some(g) => {
                    weighted += g * lec.ects as f32;
                    graded_ects += lec.ects;
                }
                None if lec.completed => {
                    points_only_ects += lec.ects;
                }
                _ => {}
            }
        }

        let avg = if graded_ects > 0 {
            weighted / graded_ects as f32
        } else {
            0.0
        };

        let avg_str = if avg <= self.target_grade {
            format!("{:.2}", avg).green()
        } else {
            format!("{:.2}", avg).red()
        };

        let total_ects = graded_ects + points_only_ects;
        println!("Average over {} ECTS: {}", graded_ects, avg_str);
        println!(
            "ECTS: {}/{} (graded {}/{} + points-only {}/{})",
            total_ects,
            self.target_ects,
            graded_ects,
            self.target_ects,
            points_only_ects,
            self.target_ects,
        );
    }

    pub fn ects_in_file(&self) {
        let combined_ects: u8 = self.lectures.iter().map(|l| l.ects).sum();
        println!(
            "You've added courses with {} ects combined, targeting {} ects.",
            combined_ects, self.target_ects
        );
    }

    pub fn overview(&self, filter_by: Option<FilterBy>, sort_by: Option<SortBy>) {
        let mut list = self.lectures.clone();

        if let Some(f) = filter_by {
            if let Some(sem) = f.semester {
                list.retain(|lec| lec.semester == sem);
            } else if let Some(max_grade) = f.grade {
                list.retain(|lec| lec.grade.is_some_and(|g| g == max_grade));
            } else if let Some(ects) = f.ects {
                list.retain(|lec| lec.ects == ects);
            }
        }

        if let Some(key) = sort_by {
            match key {
                SortBy::Grade => {
                    list.sort_by(|a, b| match (a.grade, b.grade) {
                        (Some(ga), Some(gb)) => ga.partial_cmp(&gb).unwrap(),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    });
                }
                SortBy::Semester => {
                    list.sort_by_key(|l| l.semester);
                }
                SortBy::Ects => {
                    list.sort_by_key(|l| l.ects);
                }
            }
        }

        println!(
            "{:<40} {:>4} {:>4} {:>6} {:>3}",
            "Title", "ECTS", "Sem", "Grade", "✓"
        );
        println!("{}", "-".repeat(63));

        for lec in list {
            let grade_str = match lec.grade {
                Some(g) if g <= self.target_grade => format!("{:.1}", g).green(),
                Some(g) => format!("{:.1}", g).red(),
                None => "-".normal(),
            };
            let done_flag = if lec.completed { "✓" } else { "✗" };

            println!(
                "{:<40} {:>4} {:>4} {:>6} {:>3}",
                lec.title, lec.ects, lec.semester, grade_str, done_flag
            );
        }

        let total: u8 = self.lectures.iter().map(|l| l.ects).sum();
        println!("\nTotal ECTS in file: {}", total);
    }
}
