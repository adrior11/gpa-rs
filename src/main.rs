mod cli;
mod gpa;
mod utils;

use clap::Parser;
use cli::{Cli, Pattern};
use gpa::GPA;

const GPA_FILE_PATH: &str = "src/data/gpa.json";

fn handle_cli(pattern: Pattern, gpa: &mut GPA) {
    match pattern {
        Pattern::Summary => gpa.summary(),
        Pattern::File => gpa.credits_in_file(),
        Pattern::Overview {
            filter_by,
            sort_by,
            desc,
        } => gpa.overview(filter_by, sort_by, desc),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut gpa = GPA::from_file(GPA_FILE_PATH)?;

    if let Some(pattern) = args.pattern {
        handle_cli(pattern, &mut gpa);
    } else {
        gpa.summary();
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
