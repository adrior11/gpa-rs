mod commands;
mod file_util;
mod gpa;
mod prompts;
mod utils;

use clap::Parser;

use commands::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut gpa = file_util::load_or_create_config()?;

    match args.command {
        Some(Command::Settings) => {
            match prompts::start(&mut gpa) {
                Err(e) if utils::was_interrupted(&*e) => Ok(()),
                other => other,
            }?;
        }
        None => {
            gpa.overview(
                &mut std::io::stdout(),
                args.filter.as_ref(),
                &args.order_by,
                args.reverse,
                args.short,
            )?;
        }
    }

    Ok(())
}
