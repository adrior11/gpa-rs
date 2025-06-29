use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

/// Rust-powered personal command-line GPA tracker
#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[clap(flatten)]
    pub filter: Option<FilterBy>,

    /// Select columns to display (repeatable)
    #[arg(short, long, value_enum, num_args = 1.., action = ArgAction::Append)]
    pub select: Vec<Column>,

    /// Order by column (repeatable)
    #[arg(short, long, value_enum, num_args = 1.., action = ArgAction::Append)]
    pub order_by: Vec<Column>,

    /// Reverse the final order
    #[arg(short, long)]
    pub reverse: bool,

    /// Show only stats (hide the rows)
    #[arg(long)]
    pub short: bool,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum Command {
    /// Launch interactive TUI for managing courses and GPA configuration
    ///
    /// Aliases: [cfg, config, set]
    #[command(alias = "cfg", alias = "config", alias = "set")]
    Settings,

    #[command(alias = "t")]
    Tui,
}

#[derive(Args, Clone, Debug, Default, PartialEq)]
#[group(
    id = "filter_by",
    multiple = true,
    args = ["semester", "grade", "credits", "completed"]
)]
pub struct FilterBy {
    /// Filter: title contains <STRING>
    #[arg(short, long, group = "filter_by")]
    pub title: Option<String>,

    /// Filter: semester equals <NUM> or in <NUM>..<NUM>
    #[arg(short = 'n', long, group = "filter_by", value_parser = parse_range::<u16>)]
    pub semester: Option<NumRange<u16>>,

    /// Filter: grade equals <DECIMAL> or in <DECIMAL>..<DECIMAL>
    #[arg(short, long, group = "filter_by", value_parser = parse_range::<f32>)]
    pub grade: Option<NumRange<f32>>,

    /// Filter: credits equals <NUM> or in <NUM>..<NUM>
    #[arg(short, long, group = "filter_by", value_parser = parse_range::<u16>)]
    pub credits: Option<NumRange<u16>>,

    /// Filter: completed state
    #[arg(short = 'd', long, group = "filter_by", alias = "done")]
    pub completed: Option<bool>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Column {
    Title = 0,
    Credits,
    Semester,
    Grade,
    Completed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumRange<T> {
    Single(T),
    Range(RangeInclusive<T>),
}

impl<T: PartialOrd> NumRange<T> {
    pub fn matches(&self, v: &T) -> bool {
        match self {
            Self::Single(x) => v == x,
            Self::Range(r) => r.contains(v),
        }
    }
}

impl Cli {
    pub fn columns(&self) -> Vec<Column> {
        let mut cols: Vec<Column> = vec![
            Column::Title,
            Column::Credits,
            Column::Semester,
            Column::Grade,
            Column::Completed,
        ]
        .into_iter()
        .collect();

        if self.select.is_empty() {
            return cols;
        }

        cols.retain(|c| self.select.contains(c));

        cols
    }
}

fn parse_range<T>(s: &str) -> Result<NumRange<T>, String>
where
    T: FromStr + PartialOrd + Display,
    <T as FromStr>::Err: Display,
{
    let clean = s.replace("..=", "..");
    if let Some((a, b)) = clean.split_once("..").or_else(|| clean.split_once('-')) {
        let start: T = a.trim().parse().map_err(|e| format!("{e}"))?;
        let end: T = b.trim().parse().map_err(|e| format!("{e}"))?;
        if start > end {
            return Err("range must be low..high".into());
        }
        Ok(NumRange::Range(start..=end))
    } else {
        Ok(NumRange::Single(
            s.trim().parse().map_err(|e| format!("{e}"))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli_args() {
        let args = Cli::parse_from([
            "gpa",
            "--semester",
            "5",
            "--select",
            "title",
            "grade",
            "--order-by",
            "semester",
            "grade",
            "--reverse",
            "--short",
        ]);
        assert!(args.filter.is_some());
        assert_eq!(args.order_by, vec![Column::Semester, Column::Grade]);
        assert_eq!(args.select, vec![Column::Title, Column::Grade]);
        assert!(args.reverse);
        assert!(args.short);

        let cols = args.columns();
        assert_eq!(cols, vec![Column::Title, Column::Grade]);
    }

    #[test]
    fn parse_cli_no_args() {
        let args = Cli::parse_from([""]);
        assert!(args.filter.is_none());
        assert!(args.select.is_empty());
        assert!(args.order_by.is_empty());
        assert!(!args.reverse);
        assert!(!args.short);

        let cols = args.columns();
        assert_eq!(
            cols,
            vec![
                Column::Title,
                Column::Credits,
                Column::Semester,
                Column::Grade,
                Column::Completed
            ]
        );
    }

    #[test]
    fn parse_single_u16() {
        let r = parse_range::<u16>("8").unwrap();
        assert!(matches!(r, NumRange::Single(8)));
        assert!(r.matches(&8));
        assert!(!r.matches(&7));
    }

    #[test]
    fn parse_single_16_with_spaces() {
        let r = parse_range::<u16>("   42   ").unwrap();
        assert_eq!(r, NumRange::Single(42));
        assert!(r.matches(&42));
    }

    #[test]
    fn parse_u16_range() {
        let r = parse_range::<u16>("2..5").unwrap();
        assert!(matches!(r, NumRange::Range(_)));
        assert!(r.matches(&2) && r.matches(&4) && r.matches(&5));
        assert!(!r.matches(&6));
    }

    #[test]
    fn parse_u16_hyphen_range() {
        let r = parse_range::<u16>("10-12").unwrap();
        assert!(r.matches(&10) && r.matches(&11) && r.matches(&12));
    }

    #[test]
    fn parse_single_float() {
        let r = parse_range::<f32>("1.14").unwrap();
        assert!(matches!(r, NumRange::Single(1.14)));
        assert!(r.matches(&1.14));
        assert!(!r.matches(&3.0));
    }

    #[test]
    fn parse_float_range() {
        let r = parse_range::<f32>("1.0..=2.5").unwrap();
        assert!(r.matches(&1.7));
        assert!(!r.matches(&0.9));
    }

    #[test]
    fn parse_float_hyphen_range() {
        let r = parse_range::<f32>("1.0-2.5").unwrap();
        assert!(r.matches(&1.7));
        assert!(!r.matches(&0.9));
    }

    #[test]
    fn parse_u16_range_rejects_non_number() {
        let err = parse_range::<u16>("abc").unwrap_err();
        assert!(err.contains("invalid digit"));
    }

    #[test]
    fn parse_float_range_reject_non_number() {
        let err = parse_range::<f32>("abc").unwrap_err();
        assert!(err.contains("invalid float literal"));
    }

    #[test]
    fn parse_range_rejects_reverse_order() {
        let err = parse_range::<f32>("10.0..5.0").unwrap_err();
        assert_eq!(err, "range must be low..high");
    }

    #[test]
    fn parse_range_rejects_reverse_hyphen() {
        let err = parse_range::<f32>("12.0-3.0").unwrap_err();
        assert_eq!(err, "range must be low..high");
    }
}
