use std::{cmp::Ordering, fs};

use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{
    cli::{FilterBy, OrderBy},
    file_util,
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GPA {
    pub target_average: f32,
    pub grading_system: GradingSystem,
    pub lectures: Vec<Lecture>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GradingSystem {
    pub credit_goal: u16,
    pub ignore_failed: bool,
    pub lower_is_better: bool,
    pub pass_mark: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Lecture {
    pub title: String,
    pub credits: u16,
    pub semester: u16,
    pub grade: Option<f32>,
    pub completed: bool,
}

#[derive(Default)]
struct Stats {
    all: u16,
    graded: u16,
    points_only: u16,
    weighted: f32,
}

impl Default for GPA {
    fn default() -> Self {
        Self {
            target_average: 2.0,
            grading_system: GradingSystem::default(),
            lectures: Vec::new(),
        }
    }
}

impl Default for GradingSystem {
    fn default() -> Self {
        Self {
            credit_goal: 180,
            pass_mark: 4.0,
            ignore_failed: true,
            lower_is_better: true,
        }
    }
}

impl GPA {
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(file_util::get_config_path(), json)?;
        Ok(())
    }

    pub fn get_lecture_mut(&mut self, idx: usize) -> Result<&mut Lecture> {
        self.lectures
            .get_mut(idx)
            .ok_or_else(|| anyhow!("No lecture at index {idx}"))
    }

    pub fn add_lecture(&mut self, lec: Lecture) -> Result<()> {
        self.lectures.push(lec);
        self.sort_default();
        self.save()
    }

    pub fn delete_lecture(&mut self, idx: usize) -> Result<()> {
        if idx < self.lectures.len() {
            self.lectures.remove(idx);
            self.save()
        } else {
            Err(anyhow!("No lecture at index {idx}"))
        }
    }

    pub fn overview(
        &self,
        filter: Option<FilterBy>,
        order_by: Vec<OrderBy>,
        desc: bool,
        short: bool,
    ) {
        let mut rows: Vec<&Lecture> = self
            .lectures
            .iter()
            .filter(|l| filter.as_ref().is_none_or(|f| f.matches(l)))
            .collect();

        for key in order_by.iter().rev() {
            rows.sort_by(|a, b| key.compare(a, b));
        }
        if desc {
            rows.reverse();
        }

        let mut stats = Stats::default();
        rows.iter()
            .for_each(|lec| stats.add(lec, &self.grading_system));

        if !short {
            self.print_header();
            for lec in &rows {
                self.print_row(lec);
            }
            println!();
        }

        self.print_average(&stats);
        self.print_progress(&stats, &filter);
    }

    fn print_header(&self) {
        let header_line = format!(
            " {:<40} {:>7} {:>4} {:>6} {:>3}",
            "Title", "Credits", "Sem", "Grade", "✓"
        )
        .bold();
        println!("{header_line}");
        println!("{}", "─".repeat(66).dimmed());
    }

    fn print_row(&self, lec: &Lecture) {
        let grade = match lec.grade {
            Some(g) if !self.grading_system.is_pass(g) && self.grading_system.ignore_failed => {
                format!("{g:.1}").red().bold()
            }
            Some(g) if self.grading_system.better(g, self.target_average) => {
                format!("{g:.1}").green()
            }
            Some(g) => format!("{g:.1}").red(),
            None => "-".dimmed(),
        };

        let flag = if lec.completed { "✓" } else { "" }.dimmed();
        println!(
            " {:<40} {:>7} {:>4} {:>6} {:>3}",
            lec.title, lec.credits, lec.semester, grade, flag
        );
    }

    fn print_average(&self, stats: &Stats) {
        let avg = stats.avg();
        let avg_str = if avg <= 0.0 {
            "".into()
        } else if self.grading_system.better(avg, self.target_average) {
            format!("{:.2}", avg).green()
        } else {
            format!("{:.2}", avg).red()
        };
        if avg > 0.0 {
            println!("Average over {} graded credits: {}", stats.graded, avg_str);
        }
    }

    fn print_progress(&self, stats: &Stats, filter: &Option<FilterBy>) {
        let max = if filter.is_some() {
            stats.all
        } else {
            self.grading_system.credit_goal
        };
        let credits_str = format!("({}/{})", stats.total(), max,).dimmed();
        println!(
            "Progress {} {}",
            self.progress_bar(stats.total(), max, 45),
            credits_str
        );
    }

    fn progress_bar(&self, current: u16, total: u16, width: usize) -> String {
        let total = total.max(1); // avoid div-by-zero
        let filled = ((current as f32 / total as f32) * width as f32).round() as usize;
        let done = "█".repeat(filled).cyan();
        let rest = "░".repeat(width - filled).dimmed();
        format!("[{}{}]", done, rest)
    }

    fn sort_default(&mut self) {
        self.lectures
            .sort_by(|a, b| a.semester.cmp(&b.semester).then(a.title.cmp(&b.title)));
    }
}

impl GradingSystem {
    pub fn is_pass(&self, g: f32) -> bool {
        if self.lower_is_better {
            g <= self.pass_mark
        } else {
            g >= self.pass_mark
        }
    }

    pub fn better(&self, a: f32, b: f32) -> bool {
        if self.lower_is_better {
            a <= b
        } else {
            a >= b
        }
    }
}

impl Stats {
    fn add(&mut self, lec: &Lecture, sys: &GradingSystem) {
        self.all += lec.credits;

        match (lec.grade, lec.completed) {
            (Some(g), _) if sys.ignore_failed && !sys.is_pass(g) => { /* ignore */ }
            (Some(g), _) => {
                self.weighted += g * lec.credits as f32;
                self.graded += lec.credits;
            }
            (None, true) => self.points_only += lec.credits,
            _ => { /* ignore */ }
        }
    }
    fn avg(&self) -> f32 {
        if self.graded > 0 {
            self.weighted / self.graded as f32
        } else {
            0.0
        }
    }
    fn total(&self) -> u16 {
        self.graded + self.points_only
    }
}

impl FilterBy {
    pub fn matches(&self, lec: &Lecture) -> bool {
        if let Some(t) = &self.title {
            if !lec.title.contains(t) {
                return false;
            }
        }
        if let Some(s) = self.semester {
            if lec.semester != s {
                return false;
            }
        }
        if let Some(g) = self.grade {
            if lec.grade != Some(g) {
                return false;
            }
        }
        if let Some(c) = self.credits {
            if lec.credits != c {
                return false;
            }
        }
        if let Some(b) = self.completed {
            if lec.completed != b {
                return false;
            }
        }
        true
    }
}

impl OrderBy {
    pub fn compare(&self, a: &Lecture, b: &Lecture) -> Ordering {
        match self {
            OrderBy::Title => a.title.cmp(&b.title),
            OrderBy::Credits => a.credits.cmp(&b.credits),
            OrderBy::Semester => a.semester.cmp(&b.semester),
            OrderBy::Grade => match (a.grade, b.grade) {
                (Some(ga), Some(gb)) => ga.partial_cmp(&gb).unwrap_or(Ordering::Equal),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            OrderBy::Completed => a.completed.cmp(&b.completed),
        }
    }
}
