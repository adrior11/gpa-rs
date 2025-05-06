mod cli;
mod model;
mod thesis_parser;

use clap::Parser;
use cli::{Cli, Pattern};
use model::GPA;

const GPA_FILE_PATH: &str = "src/data/gpa.json";

fn handle_cli(pattern: Pattern, gpa: &mut GPA) {
    match pattern {
        Pattern::Ects => gpa.calc_avg(),
        Pattern::File => gpa.ects_in_file(),
        Pattern::Semester { args } => gpa.semester(args),
    }
}

fn run() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let mut gpa = GPA::from_file(GPA_FILE_PATH);

    match args.pattern {
        Some(pattern) => handle_cli(pattern, &mut gpa),
        // TODO: replace with help command
        None => gpa.calc_avg(),
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
