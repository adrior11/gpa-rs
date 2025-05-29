mod cli;
mod file_util;
mod gpa;
mod tui;

use clap::Parser;

use cli::{Cli, Pattern};
use gpa::GPA;
use tui::Tui;

fn handle_cli(pattern: Pattern, gpa: &mut GPA) -> Result<(), Box<dyn std::error::Error>> {
    match pattern {
        Pattern::File => gpa.credits_in_file(),
        Pattern::Overview {
            filter_by,
            sort_by,
            desc,
            short,
        } => gpa.overview(filter_by, sort_by, desc, short),
        Pattern::Tui => Tui::start(gpa)?,
    };
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut gpa = file_util::load_or_create_config()?;

    if let Some(pattern) = args.pattern {
        handle_cli(pattern, &mut gpa)?;
    } else {
        gpa.overview(None, vec![], false, true);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
