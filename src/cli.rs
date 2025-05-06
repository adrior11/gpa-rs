use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    pub pattern: Option<Pattern>,
}

#[derive(Subcommand)]
pub enum Pattern {
    #[command(alias = "e")]
    Ects,

    #[command(alias = "f")]
    File,

    #[command(
        alias = "o",
        group(
            ArgGroup::new("filter_by")
                .args(&["semester", "grade", "ects"])
                .multiple(false)
        )
    )]
    Overview {
        #[clap(flatten)]
        filter_by: Option<FilterBy>,

        #[arg(short = 's', long = "sort", value_enum)]
        sort_by: Option<SortBy>,
    },
}

#[derive(Clone, Debug, Args)]
pub struct FilterBy {
    #[arg(short = 'n', long = "semester", group = "filter_by")]
    pub semester: Option<u8>,

    #[arg(short = 'g', long = "grade", group = "filter_by")]
    pub grade: Option<f32>,

    #[arg(short = 'e', long = "ects", group = "filter_by")]
    pub ects: Option<u8>,
}

#[derive(Clone, ValueEnum)]
pub enum SortBy {
    Semester,
    Grade,
    Ects,
}
