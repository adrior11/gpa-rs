// FIXME: row columns for grade and completed are not right aligned
use std::{cmp::Ordering, fmt::Write, fs, io, path::PathBuf};

use anyhow::Context;
use colored::Colorize;
use ratatui::{layout::Constraint, widgets::Table};
use serde::{Deserialize, Serialize};

use crate::{
    commands::{Column, FilterBy},
    model::{Course, GradingSystem, Stats},
};

use super::{semester::Semester, Exam};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Gpa {
    pub target_average: f32,
    pub grading_system: GradingSystem,
    pub semesters: Vec<Semester>,
    pub courses: Vec<Course>,
}

impl Default for Gpa {
    fn default() -> Self {
        Self {
            target_average: 2.0,
            grading_system: GradingSystem::default(),
            semesters: Vec::new(),
            courses: Vec::new(),
        }
    }
}

impl Gpa {
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json).with_context(|| format!("writing GPA to {path:?}"))?;
        Ok(())
    }

    pub fn get_course_mut(&mut self, idx: usize) -> anyhow::Result<&mut Course> {
        self.courses
            .get_mut(idx)
            .ok_or_else(|| anyhow::anyhow!("No course at index {idx}"))
    }

    pub fn add_course(&mut self, course: Course) {
        self.courses.push(course);
        self.sort_default();
    }

    pub fn delete_course(&mut self, idx: usize) -> anyhow::Result<()> {
        if idx < self.courses.len() {
            self.courses.remove(idx);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No course at index {idx}"))
        }
    }

    pub fn get_stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.courses
            .iter()
            .for_each(|c| stats.add(c, &self.grading_system));
        stats
    }

    pub fn overview<W: io::Write>(
        &self,
        out: &mut W,
        filter: Option<&FilterBy>,
        cols: &[Column],
        order_by: &[Column],
        reverse: bool,
        short: bool,
    ) -> anyhow::Result<()> {
        let mut rows: Vec<&Course> = self
            .courses
            .iter()
            .filter(|l| filter.as_ref().is_none_or(|f| f.matches(l)))
            .collect();

        for key in order_by.iter().rev() {
            rows.sort_by(|a, b| key.compare(a, b));
        }
        if reverse {
            rows.reverse();
        }

        let stats = self.get_stats();

        if !short {
            write_header(out, cols)?;
            for course in &rows {
                self.write_row(out, cols, course)?;
            }
            writeln!(out)?;
        }

        self.write_average(out, &stats)?;
        self.write_progress(out, &stats, filter)?;

        Ok(())
    }

    pub fn all_exams(&self) -> Vec<&Exam> {
        self.courses
            .iter()
            .flat_map(|c| c.upcoming_exams())
            .collect::<Vec<_>>()
    }

    pub fn to_table(&self) -> Table {
        let table_widths = [
            Constraint::Min(20),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
        ];

        let rows = self.courses.iter().map(|c| c.to_row());

        Table::new(rows, table_widths).column_spacing(1)
    }

    fn write_row<W: io::Write>(
        &self,
        w: &mut W,
        cols: &[Column],
        course: &Course,
    ) -> anyhow::Result<()> {
        let mut line = String::new();
        if cols.contains(&Column::Title) {
            write!(line, " {:<40}", course.title)?;
        }
        if cols.contains(&Column::Credits) {
            write!(line, " {:>7}", course.credits)?;
        }
        if cols.contains(&Column::Semester) {
            write!(line, " {:>4}", course.semester)?;
        }
        if cols.contains(&Column::Grade) {
            let g = match course.grade {
                Some(g) if !self.grading_system.is_pass(g) && self.grading_system.ignore_failed => {
                    format!("{g:.1}").red().bold()
                }
                Some(g) if self.grading_system.better(g, self.target_average) => {
                    format!("{g:.1}").green()
                }
                Some(g) => format!("{g:.1}").red(),
                None => "-".dimmed(),
            };
            write!(line, " {g:>6}")?;
        }
        if cols.contains(&Column::Completed) {
            write!(
                line,
                " {:>3}",
                if course.completed { "✓" } else { "" }.dimmed()
            )?;
        }
        writeln!(w, "{line}")?;
        Ok(())
    }

    fn write_average<W: io::Write>(&self, w: &mut W, stats: &Stats) -> anyhow::Result<()> {
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

    fn write_progress<W: io::Write>(
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

    pub fn sort_by(&mut self, column: &Column, reverse: bool) {
        self.courses.sort_by(|a, b| {
            let ord = column.compare(a, b);
            if reverse {
                ord.reverse()
            } else {
                ord
            }
        });
    }

    fn sort_default(&mut self) {
        self.courses
            .sort_by(|a, b| a.semester.cmp(&b.semester).then(a.title.cmp(&b.title)));
    }
}

// NOTE: move this somewhere else
impl FilterBy {
    pub fn matches(&self, course: &Course) -> bool {
        if let Some(t) = &self.title {
            if !course.title.contains(t) {
                return false;
            }
        }
        if let Some(ref s) = self.semester {
            if !s.matches(&course.semester) {
                return false;
            }
        }
        if let Some(ref g) = self.grade {
            match course.grade {
                Some(val) if g.matches(&val) => {}
                _ => return false,
            }
        }
        if let Some(ref c) = self.credits {
            if !c.matches(&course.credits) {
                return false;
            }
        }
        if let Some(b) = self.completed {
            if course.completed != b {
                return false;
            }
        }
        true
    }
}

// NOTE: move this somewhere else
impl Column {
    pub fn compare(&self, a: &Course, b: &Course) -> Ordering {
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
            Self::Completed => b.completed.cmp(&a.completed), // completed courses first
        }
    }
}

fn write_header<W: io::Write>(w: &mut W, cols: &[Column]) -> anyhow::Result<()> {
    let mut line = String::new();
    if cols.contains(&Column::Title) {
        write!(line, " {:<40}", "Title")?;
    }
    if cols.contains(&Column::Credits) {
        write!(line, " {:>7}", "Credits")?;
    }
    if cols.contains(&Column::Semester) {
        write!(line, " {:>4}", "Sem")?;
    }
    if cols.contains(&Column::Grade) {
        write!(line, " {:>6}", "Grade")?;
    }
    if cols.contains(&Column::Completed) {
        write!(line, " {:>3}", "✓")?;
    }
    writeln!(w, "{}", line.bold())?;

    let divider_len = if cols.contains(&Column::Completed) {
        line.len() - 1
    } else {
        line.len() + 1
    };
    writeln!(w, "{}", "─".repeat(divider_len).dimmed())?;

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

    fn all_columns() -> Vec<Column> {
        vec![
            Column::Title,
            Column::Credits,
            Column::Semester,
            Column::Grade,
            Column::Completed,
        ]
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
    fn write_header_with_all_columns() -> anyhow::Result<()> {
        let mut buf = Vec::<u8>::new();
        write_header(&mut buf, &all_columns())?;
        let out = String::from_utf8(buf)?;
        assert!(out.contains("Title"));
        assert!(out.contains("Credits"));
        assert!(out.contains("Sem"));
        assert!(out.contains("Grade"));
        assert!(out.contains("✓"));
        Ok(())
    }

    #[test]
    fn write_header_with_no_selected_columns() -> anyhow::Result<()> {
        let mut buf = Vec::<u8>::new();
        write_header(&mut buf, &[])?;
        let out = String::from_utf8(buf)?;
        assert!(!out.contains("Title"));
        assert!(!out.contains("Credits"));
        assert!(!out.contains("Sem"));
        assert!(!out.contains("Grade"));
        assert!(!out.contains("✓"));
        Ok(())
    }

    #[test]
    fn order_by_compare() {
        let a = Course::new("A".to_string(), 5, 1, Some(1.7), true);
        let b = Course::new("B".to_string(), 5, 2, Some(2.3), false);
        let c = Course::new("C".to_string(), 5, 2, None, false);

        assert_eq!(Column::Title.compare(&a, &b), Ordering::Less);
        assert_eq!(Column::Credits.compare(&a, &b), Ordering::Equal);
        assert_eq!(Column::Semester.compare(&a, &b), Ordering::Less);
        assert_eq!(Column::Completed.compare(&a, &b), Ordering::Less);

        assert_eq!(Column::Grade.compare(&a, &b), Ordering::Less); // some & some
        assert_eq!(Column::Grade.compare(&a, &c), Ordering::Less); // some & none
        assert_eq!(Column::Grade.compare(&c, &b), Ordering::Greater); // none & some
        assert_eq!(Column::Grade.compare(&c, &c), Ordering::Equal); // none & none
    }

    #[test]
    fn filter_by_matches() {
        let course = Course::new("Programming I".to_string(), 6, 1, Some(1.3), true);

        let f = FilterBy {
            title: Some("Programming".into()),
            credits: Some(NumRange::Range(RangeInclusive::new(3, 9))),
            semester: Some(NumRange::Single(1)),
            grade: Some(NumRange::Single(1.3)),
            completed: Some(true),
        };
        assert!(f.matches(&course));
    }

    #[test]
    fn filter_by_matches_negative() {
        let course = Course::new("Programming I".to_string(), 6, 1, Some(1.3), true);

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
            assert!(!f.matches(&course));
        }
    }

    #[test]
    fn gpa_sorting_and_mut_access() {
        let mut g = Gpa::default();
        g.courses
            .push(Course::new("Calculus".to_string(), 5, 2, None, false));
        g.courses
            .push(Course::new("Algorithms".to_string(), 5, 1, None, false));
        g.sort_default();

        assert_eq!(g.courses[0].title, "Algorithms");
        assert_eq!(g.courses[1].title, "Calculus");

        {
            let l = g.get_course_mut(0).unwrap();
            l.grade = Some(1.0);
        }
        assert_eq!(g.courses[0].grade, Some(1.0));
    }

    #[test]
    fn gpa_add_and_delete_course() {
        let mut g = Gpa::default();
        assert_eq!(g.courses.len(), 0);

        let course = Course::new("Calculus".to_string(), 6, 1, Some(2.3), true);
        g.add_course(course.clone());

        assert_eq!(g.courses.len(), 1);
        assert_eq!(g.courses[0], course);

        g.delete_course(0).unwrap();
        assert_eq!(g.courses.len(), 0);
        assert!(g.delete_course(0).is_err());
    }

    #[test]
    fn overview_full_output_order_by_title() -> anyhow::Result<()> {
        let gpa = Gpa {
            courses: vec![
                Course::new("Algorithms".to_string(), 6, 1, Some(1.7), true),
                Course::new("Calculus".to_string(), 6, 2, Some(3.3), true),
                Course::new("Physics".to_string(), 8, 2, Some(4.7), true),
                Course::new("Project".to_string(), 10, 3, None, false),
            ],
            ..Default::default()
        };

        let mut buf = Vec::<u8>::new();

        gpa.overview(
            &mut buf,
            None,
            &all_columns(),
            &[Column::Title],
            true,
            false,
        )?;

        let out = String::from_utf8(buf)?;
        assert!(out.contains("Algorithms"));
        assert!(out.contains("Project"));
        assert!(out.contains("Average over 12 graded credits:")); // only Algorithms & Calculus counts
        assert!(out.contains("Progress ["));
        Ok(())
    }

    #[test]
    fn overview_short_and_filtered() -> anyhow::Result<()> {
        let mut gpa = Gpa::default();
        gpa.courses
            .push(Course::new("Math".to_string(), 5, 1, Some(1.3), true));

        let mut buf = Vec::<u8>::new();
        let filter = Some(FilterBy {
            title: Some("Math".into()),
            ..Default::default()
        });
        gpa.overview(&mut buf, filter.as_ref(), &all_columns(), &[], false, true)?;

        let txt = String::from_utf8(buf)?;
        assert!(!txt.contains("Title"));
        assert!(txt.contains("Average"));
        Ok(())
    }

    #[test]
    fn overview_short_and_no_graded_courses() -> anyhow::Result<()> {
        let mut gpa = Gpa::default();
        gpa.courses
            .push(Course::new("Math".to_string(), 5, 1, None, true));
        gpa.courses
            .push(Course::new("Project".to_string(), 5, 1, None, false));

        let mut buf = Vec::<u8>::new();
        gpa.overview(&mut buf, None, &all_columns(), &[], false, true)?;

        let txt = String::from_utf8(buf)?;
        assert!(!txt.contains("Title"));
        assert!(!txt.contains("Average"));
        assert!(txt.contains("Progress"));
        Ok(())
    }

    #[test]
    fn overview_no_columns() -> anyhow::Result<()> {
        let mut gpa = Gpa::default();
        gpa.courses
            .push(Course::new("Math".to_string(), 5, 1, Some(1.3), true));
        let mut buf = Vec::<u8>::new();
        gpa.overview(&mut buf, None, &[], &[], false, false)?;
        let txt = String::from_utf8(buf)?;
        assert!(!txt.contains("Title"));
        assert!(!txt.contains("Credits"));
        assert!(!txt.contains("Sem"));
        assert!(!txt.contains("Grade"));
        assert!(!txt.contains("✓"));
        Ok(())
    }

    #[test]
    fn get_course_mut_error() {
        let mut gpa = Gpa::default();
        assert!(gpa.get_course_mut(0).is_err());
        gpa.courses
            .push(Course::new("Math".to_string(), 5, 1, Some(1.3), true));
        assert!(gpa.get_course_mut(0).is_ok());
        assert!(gpa.get_course_mut(1).is_err());
    }
}
