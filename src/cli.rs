use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub pattern: Option<Pattern>,
}

#[derive(Subcommand)]
pub enum Pattern {
    #[command(alias = "s")]
    Summary,

    #[command(alias = "f")]
    File,

    #[command(
        alias = "o",
        group(
            ArgGroup::new("filter_by")
                .args(&["semester", "grade", "credits", "completed"])
                .multiple(false)
        )
    )]
    // TODO: Set as default command
    Overview {
        #[clap(flatten)]
        filter_by: Option<FilterBy>,

        #[arg(short = 's', long = "sort", value_enum)]
        sort_by: Option<SortBy>,

        /// Show the courses in descending order (default is ascending).
        #[arg(short = 'd', long = "desc")]
        desc: bool,
    },
}

#[derive(Clone, Debug, Args)]
pub struct FilterBy {
    #[arg(short = 'n', long = "semester", group = "filter_by")]
    pub semester: Option<u8>,

    #[arg(short = 'g', long = "grade", group = "filter_by")]
    pub grade: Option<f32>,

    #[arg(short = 'c', long = "credits", group = "filter_by")]
    pub credits: Option<u8>,

    #[arg(short = 'x', long = "completed", group = "filter_by")]
    pub completed: Option<bool>, // TODO: rename short
}

#[derive(Clone, ValueEnum)]
pub enum SortBy {
    Semester,
    Grade,
    Ects,
    Completed,
}
