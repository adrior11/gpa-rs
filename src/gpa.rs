use std::{cmp::Ordering, fs, io::Write, path::PathBuf};

use anyhow::Context;
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::commands::{FilterBy, OrderBy};

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GPA {
    pub target_average: f32,
    pub grading_system: GradingSystem,
    pub lectures: Vec<Lecture>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradingSystem {
    pub credit_goal: u16,
    pub ignore_failed: bool,
    pub lower_is_better: bool,
    pub pass_mark: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json).with_context(|| format!("writing GPA to {path:?}"))?;
        Ok(())
    }

    pub fn get_lecture_mut(&mut self, idx: usize) -> anyhow::Result<&mut Lecture> {
        self.lectures
            .get_mut(idx)
            .ok_or_else(|| anyhow::anyhow!("No lecture at index {idx}"))
    }

    pub fn add_lecture(&mut self, lec: Lecture) {
        self.lectures.push(lec);
        self.sort_default();
    }

    pub fn delete_lecture(&mut self, idx: usize) -> anyhow::Result<()> {
        if idx < self.lectures.len() {
            self.lectures.remove(idx);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No lecture at index {idx}"))
        }
    }

    pub fn overview<W: Write>(
        &self,
        out: &mut W,
        filter: Option<&FilterBy>,
        order_by: &[OrderBy],
        reverse: bool,
        short: bool,
    ) -> anyhow::Result<()> {
        let mut rows: Vec<&Lecture> = self
            .lectures
            .iter()
            .filter(|l| filter.as_ref().is_none_or(|f| f.matches(l)))
            .collect();

        for key in order_by.iter().rev() {
            rows.sort_by(|a, b| key.compare(a, b));
        }
        if reverse {
            rows.reverse();
        }

        let mut stats = Stats::default();
        rows.iter()
            .for_each(|lec| stats.add(lec, &self.grading_system));

        if !short {
            write_header(out)?;
            for lec in &rows {
                self.write_row(out, lec)?;
            }
            writeln!(out)?;
        }

        self.write_average(out, &stats)?;
        self.write_progress(out, &stats, filter)?;

        Ok(())
    }

    fn write_row<W: Write>(&self, w: &mut W, lec: &Lecture) -> anyhow::Result<()> {
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
        writeln!(
            w,
            " {:<40} {:>7} {:>4} {:>6} {:>3}",
            lec.title, lec.credits, lec.semester, grade, flag
        )?;

        Ok(())
    }

    fn write_average<W: Write>(&self, w: &mut W, stats: &Stats) -> anyhow::Result<()> {
        let avg = stats.avg();
        if avg > 0.0 {
            let avg_str = if self.grading_system.better(avg, self.target_average) {
                format!("{avg:.2}").green()
            } else {
                format!("{avg:.2}").red()
            };

            writeln!(
                w,
                "Average over {} graded credits: {}",
                stats.graded, avg_str
            )?;
        }
        Ok(())
    }

    fn write_progress<W: Write>(
        &self,
        w: &mut W,
        stats: &Stats,
        filter: Option<&FilterBy>,
    ) -> anyhow::Result<()> {
        let max = if filter.is_some() {
            stats.all
        } else {
            self.grading_system.credit_goal
        };
        let credits_str = format!("({}/{})", stats.total(), max,).dimmed();
        writeln!(
            w,
            "Progress {} {}",
            progress_bar(stats.total(), max, 45),
            credits_str
        )?;
        Ok(())
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
                self.weighted += g * f32::from(lec.credits);
                self.graded += lec.credits;
            }
            (None, true) => self.points_only += lec.credits,
            _ => { /* ignore */ }
        }
    }
    fn avg(&self) -> f32 {
        if self.graded > 0 {
            self.weighted / f32::from(self.graded)
        } else {
            0.0
        }
    }
    const fn total(&self) -> u16 {
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
        if let Some(ref s) = self.semester {
            if !s.matches(&lec.semester) {
                return false;
            }
        }
        if let Some(ref g) = self.grade {
            match lec.grade {
                Some(val) if g.matches(&val) => {}
                _ => return false,
            }
        }
        if let Some(ref c) = self.credits {
            if !c.matches(&lec.credits) {
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
            Self::Title => a.title.cmp(&b.title),
            Self::Credits => a.credits.cmp(&b.credits),
            Self::Semester => a.semester.cmp(&b.semester),
            Self::Grade => match (a.grade, b.grade) {
                (Some(ga), Some(gb)) => ga.partial_cmp(&gb).unwrap_or(Ordering::Equal),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            Self::Completed => a.completed.cmp(&b.completed),
        }
    }
}

fn write_header<W: Write>(w: &mut W) -> anyhow::Result<()> {
    let header_line = format!(
        " {:<40} {:>7} {:>4} {:>6} {:>3}",
        "Title", "Credits", "Sem", "Grade", "✓"
    )
    .bold();
    writeln!(w, "{header_line}")?;
    writeln!(w, "{}", "─".repeat(66).dimmed())?;
    Ok(())
}

fn progress_bar(current: u16, total: u16, width: usize) -> String {
    // avoid div-by-zero and clamp ratio to [0, 1]
    let ratio = (f32::from(current) / f32::from(total.max(1))).min(1.0);
    let filled = (ratio * width as f32).round() as usize;
    let done = "█".repeat(filled).cyan();
    let rest = "░".repeat(width - filled).dimmed();
    format!("[{done}{rest}]")
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, ops::RangeInclusive};

    use crate::commands::NumRange;

    use super::*;

    fn lec(title: &str, credits: u16, sem: u16, grade: Option<f32>, done: bool) -> Lecture {
        Lecture {
            title: title.into(),
            credits,
            semester: sem,
            grade,
            completed: done,
        }
    }

    #[test]
    fn grading_system_pass_and_better() {
        let mut sys = GradingSystem::default();
        // lower is better
        assert!(sys.is_pass(3.7));
        assert!(!sys.is_pass(4.5));
        assert!(sys.better(1.3, 2.0));
        assert!(!sys.better(3.0, 2.0));

        // flip the polarity
        sys.lower_is_better = false;
        sys.pass_mark = 50.0;
        assert!(sys.is_pass(65.0));
        assert!(!sys.is_pass(40.0));
        assert!(sys.better(80.0, 60.0));
        assert!(!sys.better(20.0, 40.0));
    }

    #[test]
    fn stats_accumulation_and_average() {
        let sys = GradingSystem::default();
        let mut s = Stats::default();

        s.add(&lec("Math", 6, 1, Some(2.3), true), &sys); // passed
        s.add(&lec("Phys", 8, 1, Some(5.0), true), &sys); // failed
        s.add(&lec("Project", 12, 2, None, true), &sys); // passed (ungraded)

        assert_eq!(s.graded, 6);
        assert_eq!(s.points_only, 12);
        assert_eq!(s.all, 26);
        assert!((s.avg() - 2.3).abs() < 1e-6);
        assert_eq!(s.total(), 18);
    }

    #[test]
    fn stats_avg_zero_when_no_graded() {
        let s = Stats::default();
        assert_eq!(s.avg(), 0.0);
    }

    #[test]
    fn progress_bar_width_and_fill() {
        let bar = progress_bar(30, 60, 20);
        let filled = bar.chars().filter(|c| *c == '█').count();
        let empty = bar.chars().filter(|c| *c == '░').count();
        assert_eq!(filled, 10);
        assert_eq!(empty, 10);
    }

    #[test]
    fn progress_bar_clamps_overflow() {
        let bar = progress_bar(120, 60, 10);
        assert_eq!(bar.matches('█').count(), 10);
        assert_eq!(bar.matches('░').count(), 0);
    }

    #[test]
    fn order_by_compare() {
        let a = lec("A", 5, 1, Some(1.7), true);
        let b = lec("B", 5, 2, Some(2.3), false);
        let c = lec("C", 5, 2, None, false);

        assert_eq!(OrderBy::Title.compare(&a, &b), Ordering::Less);
        assert_eq!(OrderBy::Credits.compare(&a, &b), Ordering::Equal);
        assert_eq!(OrderBy::Semester.compare(&a, &b), Ordering::Less);
        assert_eq!(OrderBy::Completed.compare(&a, &b), Ordering::Greater);

        assert_eq!(OrderBy::Grade.compare(&a, &b), Ordering::Less); // some & some
        assert_eq!(OrderBy::Grade.compare(&a, &c), Ordering::Less); // some & none
        assert_eq!(OrderBy::Grade.compare(&c, &b), Ordering::Greater); // none & some
        assert_eq!(OrderBy::Grade.compare(&c, &c), Ordering::Equal); // none & none
    }

    #[test]
    fn filter_by_matches() {
        let lec = lec("Programming I", 6, 1, Some(1.3), true);

        let f = FilterBy {
            title: Some("Programming".into()),
            credits: Some(NumRange::Range(RangeInclusive::new(3, 9))),
            semester: Some(NumRange::Single(1)),
            grade: Some(NumRange::Single(1.3)),
            completed: Some(true),
        };
        assert!(f.matches(&lec));
    }

    #[test]
    fn filter_by_matches_negative() {
        let lec = lec("Programming I", 6, 1, Some(1.3), true);

        let filters = vec![
            FilterBy {
                title: Some("Test".into()),
                ..Default::default()
            },
            FilterBy {
                credits: Some(NumRange::Single(2)),
                ..Default::default()
            },
            FilterBy {
                semester: Some(NumRange::Single(2)),
                ..Default::default()
            },
            FilterBy {
                grade: Some(NumRange::Single(1.7)),
                ..Default::default()
            },
            FilterBy {
                completed: Some(false),
                ..Default::default()
            },
        ];

        for f in filters {
            assert!(!f.matches(&lec));
        }
    }

    #[test]
    fn gpa_sorting_and_mut_access() {
        let mut g = GPA::default();
        g.lectures.push(lec("Calculus", 5, 2, None, false));
        g.lectures.push(lec("Algorithms", 5, 1, None, false));
        g.sort_default();

        assert_eq!(g.lectures[0].title, "Algorithms");
        assert_eq!(g.lectures[1].title, "Calculus");

        {
            let l = g.get_lecture_mut(0).unwrap();
            l.grade = Some(1.0);
        }
        assert_eq!(g.lectures[0].grade, Some(1.0));
    }

    #[test]
    fn gpa_add_and_delete_lecture() {
        let mut g = GPA::default();
        assert_eq!(g.lectures.len(), 0);

        let lec = lec("Calculus", 6, 1, Some(2.3), true);
        g.add_lecture(lec.clone());

        assert_eq!(g.lectures.len(), 1);
        assert_eq!(g.lectures[0], lec);

        g.delete_lecture(0).unwrap();
        assert_eq!(g.lectures.len(), 0);
        assert!(g.delete_lecture(0).is_err());
    }

    #[test]
    fn overview_full_output_order_by_title() -> anyhow::Result<()> {
        let gpa = GPA {
            lectures: vec![
                lec("Algorithms", 6, 1, Some(1.7), true),
                lec("Calculus", 6, 2, Some(3.3), true),
                lec("Physics", 8, 2, Some(4.7), true),
                lec("Project", 10, 3, None, true),
            ],
            ..Default::default()
        };

        let mut buf = Vec::<u8>::new();
        gpa.overview(&mut buf, None, &[OrderBy::Title], true, false)?;

        let out = String::from_utf8(buf)?;
        assert!(out.contains("Algorithms"));
        assert!(out.contains("Project"));
        assert!(out.contains("Average over 12 graded credits:")); // only Algorithms & Calculus counts
        assert!(out.contains("Progress ["));
        Ok(())
    }

    #[test]
    fn overview_short_and_filtered() -> anyhow::Result<()> {
        let mut gpa = GPA::default();
        gpa.lectures.push(lec("Math", 5, 1, Some(1.3), true));

        let mut buf = Vec::<u8>::new();
        let filter = Some(FilterBy {
            title: Some("Math".into()),
            ..Default::default()
        });
        gpa.overview(&mut buf, filter.as_ref(), &[], false, true)?;

        let txt = String::from_utf8(buf)?;
        assert!(!txt.contains("Title"));
        assert!(txt.contains("Average"));
        Ok(())
    }

    #[test]
    fn overview_short_and_no_graded_courses() -> anyhow::Result<()> {
        let mut gpa = GPA::default();
        gpa.lectures.push(lec("Math", 5, 1, None, true));
        gpa.lectures.push(lec("Project", 5, 1, None, false));

        let mut buf = Vec::<u8>::new();
        gpa.overview(&mut buf, None, &[], false, true)?;

        let txt = String::from_utf8(buf)?;
        assert!(!txt.contains("Title"));
        assert!(!txt.contains("Average"));
        assert!(txt.contains("Progress"));
        Ok(())
    }
}
