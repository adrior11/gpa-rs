mod cli;
mod file_util;
mod gpa;

use clap::Parser;
use cli::{Cli, Pattern};
use gpa::GPA;

fn handle_cli(pattern: Pattern, gpa: &mut GPA) {
    match pattern {
        Pattern::File => gpa.credits_in_file(),
        Pattern::Overview {
            filter_by,
            sort_by,
            desc,
            short,
        } => gpa.overview(filter_by, sort_by, desc, short),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut gpa = file_util::load_or_create_config()?;

    if let Some(pattern) = args.pattern {
        handle_cli(pattern, &mut gpa);
    } else {
        // Default: cargo run -- o -S
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
