use colored::Colorize;
use serde::Deserialize;

use crate::cli::{FilterBy, SortBy};

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Debug)]
pub struct GPA {
    pub target_credits: u8,
    pub target_grade: f32,
    pub lectures: Vec<Lecture>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Lecture {
    pub title: String,
    pub credits: u8,
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

    pub fn summary(&self) {
        let mut weighted = 0.0;
        let mut graded_credits = 0u8;
        let mut points_only_credits = 0u8;

        for lec in &self.lectures {
            match lec.grade {
                Some(g) => {
                    weighted += g * lec.credits as f32;
                    graded_credits += lec.credits;
                }
                None if lec.completed => {
                    points_only_credits += lec.credits;
                }
                _ => {}
            }
        }

        let avg = if graded_credits > 0 {
            weighted / graded_credits as f32
        } else {
            0.0
        };

        let avg_str = if avg <= self.target_grade {
            format!("{:.2}", avg).green()
        } else {
            format!("{:.2}", avg).red()
        };

        let total_credits = graded_credits + points_only_credits;
        println!(
            "Average over {} achieved Credits: {}",
            graded_credits, avg_str
        );
        println!(
            "Credits: {}/{} (graded {}/{} + points-only {}/{})",
            total_credits,
            self.target_credits,
            graded_credits,
            self.target_credits,
            points_only_credits,
            self.target_credits,
        );
    }

    pub fn credits_in_file(&self) {
        let combined_credits: u8 = self.lectures.iter().map(|l| l.credits).sum();
        println!(
            "You've added courses with {} credits combined, targeting {} credits.",
            combined_credits, self.target_credits
        );
    }

    pub fn overview(&self, filter_by: Option<FilterBy>, sort_by: Option<SortBy>, desc: bool) {
        let mut list = self.lectures.clone();

        if let Some(ref f) = filter_by {
            if let Some(sem) = f.semester {
                list.retain(|lec| lec.semester == sem);
            } else if let Some(max_grade) = f.grade {
                list.retain(|lec| lec.grade.is_some_and(|g| g == max_grade));
            } else if let Some(credits) = f.credits {
                list.retain(|lec| lec.credits == credits);
            } else if let Some(completed) = f.completed {
                list.retain(|lec| lec.completed == completed);
            }
        }

        if let Some(ref key) = sort_by {
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
                    list.sort_by_key(|l| l.credits);
                }
                SortBy::Completed => {
                    list.sort_by_key(|l| l.completed);
                }
            }
        }

        if desc {
            list.reverse();
        }

        println!(
            "{:<40} {:>7} {:>4} {:>6} {:>3}",
            "Title", "Credits", "Sem", "Grade", "✓"
        );
        println!("{}", "-".repeat(66));

        let mut combined_credits = 0;
        for lec in list {
            combined_credits += lec.credits;

            let grade_str = match lec.grade {
                Some(g) if g <= self.target_grade => format!("{:.1}", g).green(),
                Some(g) => format!("{:.1}", g).red(),
                None => "-".normal(),
            };
            let done_flag = if lec.completed { "✓" } else { "✗" };

            println!(
                "{:<40} {:>7} {:>4} {:>6} {:>3}",
                lec.title, lec.credits, lec.semester, grade_str, done_flag
            );
        }

        if filter_by.is_some() || sort_by.is_some() {
            println!("\nCredits in selection: {}", combined_credits);
        } else {
            println!()
        }

        let total: u8 = self.lectures.iter().map(|l| l.credits).sum();
        println!("⇢ Total Credits in file: {}", total);
    }
}
