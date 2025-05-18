use colored::Colorize;
use serde::Deserialize;
use std::cmp::Ordering;

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

#[derive(Default)]
struct Stats {
    all: u8,
    graded: u8,
    points_only: u8,
    weighted: f32,
}

impl GPA {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }

    pub fn credits_in_file(&self) {
        let sum: u8 = self.lectures.iter().map(|l| l.credits).sum();
        println!(
            "You've added courses with {sum} credits, targeting {}.",
            self.target_credits
        );
    }

    pub fn overview(
        &self,
        filter_by: Option<FilterBy>,
        sort_by: Option<SortBy>,
        desc: bool,
        short: bool,
    ) {
        let mut rows: Vec<&Lecture> = self
            .lectures
            .iter()
            .filter(|l| filter_by.as_ref().is_none_or(|f| f.matches(l)))
            .collect();

        if let Some(key) = sort_by {
            rows.sort_by(|a, b| key.compare(a, b));
        }
        if desc {
            rows.reverse();
        }

        let mut stats = Stats::default();
        rows.iter().for_each(|lec| stats.add(lec));

        if !short {
            Self::print_header();
            for lec in &rows {
                Self::print_row(lec, self.target_grade);
            }
            println!();
        }
        Self::print_footer(
            &filter_by,
            stats,
            if short { None } else { Some(rows.len()) },
            self.target_credits,
            self.target_grade,
        );
    }

    fn print_header() {
        println!(
            "{:<40} {:>7} {:>4} {:>6} {:>3}",
            "Title", "Credits", "Sem", "Grade", "✓"
        );
        println!("{}", "-".repeat(66));
    }

    fn print_row(lec: &Lecture, target: f32) {
        let grade = match lec.grade {
            Some(g) if g <= target => format!("{g:.1}").green(),
            Some(g) => format!("{g:.1}").red(),
            None => "-".normal(),
        };
        let flag = if lec.completed { "✓" } else { "✗" };
        println!(
            "{:<40} {:>7} {:>4} {:>6} {:>3}",
            lec.title, lec.credits, lec.semester, grade, flag
        );
    }

    fn print_footer(
        filter: &Option<FilterBy>,
        stats: Stats,
        shown_rows: Option<usize>,
        target_credits: u8,
        target_grade: f32,
    ) {
        let avg = stats.avg();
        let avg_str = if avg <= 0.0 {
            "".into()
        } else if avg <= target_grade {
            format!("{:.2}", avg).green()
        } else {
            format!("{:.2}", avg).red()
        };

        let (scope, max) = if filter.is_some() {
            ("Credits in selection", stats.all)
        } else {
            ("Total achieved credits", target_credits)
        };

        if avg > 0.0 {
            println!(
                "Average over {} graded credits: {}\n",
                stats.graded, avg_str
            );
        }
        println!("⇢ {scope}: {}/{}", stats.total(), max);

        if stats.points_only > 0 {
            println!(
                "{}",
                format!(
                    "  (graded {} + points-only {})",
                    stats.graded, stats.points_only
                )
                .dimmed()
            );
        }
        if let Some(len) = shown_rows {
            println!("  {}", format!("({len} course rows shown)").dimmed());
        }
    }
}

impl Stats {
    fn add(&mut self, lec: &Lecture) {
        self.all += lec.credits;

        match (lec.grade, lec.completed) {
            (Some(g), _) => {
                self.weighted += g * lec.credits as f32;
                self.graded += lec.credits;
            }
            (None, true) => self.points_only += lec.credits,
            _ => {}
        }
    }
    fn avg(&self) -> f32 {
        if self.graded > 0 {
            self.weighted / self.graded as f32
        } else {
            0.0
        }
    }
    fn total(&self) -> u8 {
        self.graded + self.points_only
    }
}

impl FilterBy {
    pub fn matches(&self, lec: &Lecture) -> bool {
        match *self {
            Self {
                semester: Some(s), ..
            } => lec.semester == s,
            Self { grade: Some(g), .. } => lec.grade == Some(g),
            Self {
                credits: Some(c), ..
            } => lec.credits == c,
            Self {
                completed: Some(b), ..
            } => lec.completed == b,
            _ => true,
        }
    }
}

impl SortBy {
    pub fn compare(&self, a: &Lecture, b: &Lecture) -> Ordering {
        match self {
            SortBy::Title => a.title.cmp(&b.title),
            SortBy::Credits => a.credits.cmp(&b.credits),
            SortBy::Semester => a.semester.cmp(&b.semester),
            SortBy::Grade => match (a.grade, b.grade) {
                (Some(ga), Some(gb)) => ga.partial_cmp(&gb).unwrap_or(Ordering::Equal),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            SortBy::Completed => a.completed.cmp(&b.completed),
        }
    }
}
