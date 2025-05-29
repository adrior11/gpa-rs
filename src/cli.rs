use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

/// GPA-RS – personal GPA tracker
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[clap(flatten)]
    pub filter: Option<FilterBy>,

    /// Order by column (repeatable)
    #[arg(short, long, value_enum, action = ArgAction::Append)]
    pub order_by: Vec<OrderBy>,

    /// Reverse the final order
    #[arg(short, long)]
    pub reverse: bool,

    /// Show only stats (hide the rows)
    #[arg(short, long)]
    pub short: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Interactive text-user-interface
    Tui,
}

#[derive(Clone, Debug, Args)]
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
    #[arg(short = 'd', long, group = "filter_by")]
    pub completed: Option<bool>,
}

#[derive(Clone, ValueEnum)]
pub enum OrderBy {
    Title,
    Credits,
    Semester,
    Grade,
    Completed,
}

#[derive(Clone, Debug)]
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
