use std::{cmp::Ordering, fs};

use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{
    cli::{FilterBy, SortBy},
    file_util,
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GPA {
    pub target_credits: u8,
    pub target_grade: f32,
    pub lectures: Vec<Lecture>,
}

impl Default for GPA {
    fn default() -> Self {
        Self {
            target_credits: 180,
            target_grade: 4.0,
            lectures: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
        sort_by: Vec<SortBy>,
        desc: bool,
        short: bool,
    ) {
        let mut rows: Vec<&Lecture> = self
            .lectures
            .iter()
            .filter(|l| filter_by.as_ref().is_none_or(|f| f.matches(l)))
            .collect();

        for key in sort_by.iter().rev() {
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
        let header_line = format!(
            "{:<40} {:>7} {:>4} {:>6} {:>3}",
            "Title", "Credits", "Sem", "Grade", "✓"
        )
        .bold();
        println!("{header_line}");
        println!("{}", "─".repeat(66).dimmed());
    }

    fn print_row(lec: &Lecture, target: f32) {
        let grade = match lec.grade {
            Some(g) if g <= target => format!("{g:.1}").green(),
            Some(g) => format!("{g:.1}").red(),
            None => "-".dimmed(),
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
        println!("  {}", Self::progress_bar(stats.total(), max, 30));

        // TODO: add warning line if there are more points added, then needed

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

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(file_util::get_config_path(), json)?;
        Ok(())
    }

    pub fn add_lecture(&mut self, lec: Lecture) -> Result<()> {
        self.lectures.push(lec);
        self.sort_default();
        self.save()
    }

    pub fn update_lecture(&mut self, idx: usize, mut f: impl FnMut(&mut Lecture)) -> Result<()> {
        let lec = self
            .lectures
            .get_mut(idx)
            .ok_or_else(|| anyhow!("No lecture at index {idx}"))?;
        f(lec);
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

    fn sort_default(&mut self) {
        self.lectures
            .sort_by(|a, b| a.semester.cmp(&b.semester).then(a.title.cmp(&b.title)));
    }

    fn progress_bar(current: u8, total: u8, width: usize) -> String {
        let total = total.max(1); // avoid div-by-zero
        let filled = ((current as f32 / total as f32) * width as f32).round() as usize;
        let done = "█".repeat(filled).cyan();
        let rest = "░".repeat(width - filled).dimmed();
        format!("[{}{}]", done, rest)
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
