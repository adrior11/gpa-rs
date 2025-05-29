use clap::{ArgAction, ArgGroup, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub pattern: Option<Pattern>,
}

#[derive(Subcommand)]
pub enum Pattern {
    #[command(
        alias = "o", // TODO: omit
        subcommand_negates_reqs = true,
        args_conflicts_with_subcommands = true,
        group(
            ArgGroup::new("filter_by")
                .args(&["semester", "grade", "credits", "completed"])
                .multiple(true)
        )
    )]
    Overview {
        #[clap(flatten)]
        filter_by: Option<FilterBy>,

        #[arg(short = 's', long = "sort", value_enum, action = ArgAction::Append)]
        sort_by: Vec<SortBy>,

        #[arg(short = 'd', long = "desc")]
        desc: bool,

        #[arg(short = 'S', long = "short")]
        short: bool,
    },

    #[command(alias = "t")]
    Tui,
}

#[derive(Clone, Debug, Args)]
pub struct FilterBy {
    #[arg(short = 't', long = "title", group = "filter_by")]
    pub title: Option<String>,

    #[arg(short = 'n', long = "semester", group = "filter_by")]
    pub semester: Option<u16>,

    #[arg(short = 'g', long = "grade", group = "filter_by")]
    pub grade: Option<f32>,

    #[arg(short = 'c', long = "credits", group = "filter_by")]
    pub credits: Option<u16>,

    #[arg(short = 'C', long = "completed", group = "filter_by")]
    pub completed: Option<bool>,
}

#[derive(Clone, ValueEnum)]
pub enum SortBy {
    Title,
    Credits,
    Semester,
    Grade,
    Completed,
}
