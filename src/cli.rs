use clap::{Parser, Subcommand};

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

    #[command(alias = "s")]
    Semester {
        #[arg(value_name = "NUM_SEMESTER")]
        args: u8,
    },
}
