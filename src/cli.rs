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

    /// Filter: semester equals <NUM>
    #[arg(short = 'n', long, group = "filter_by")]
    pub semester: Option<u16>,

    /// Filter: grade equals <DECIMAL>
    #[arg(short, long, group = "filter_by")]
    pub grade: Option<f32>,

    /// Filter: credits equals <NUM>
    #[arg(short, long, group = "filter_by")]
    pub credits: Option<u16>,

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
