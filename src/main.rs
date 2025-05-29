mod cli;
mod file_util;
mod gpa;
mod tui;

use clap::Parser;

use cli::{Cli, Command};

fn was_interrupted(err: &(dyn std::error::Error + 'static)) -> bool {
    if let Some(ioe) = err.downcast_ref::<std::io::Error>() {
        return ioe.kind() == std::io::ErrorKind::Interrupted;
    }
    err.source().is_some_and(was_interrupted)
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut gpa = file_util::load_or_create_config()?;

    match args.command {
        Some(Command::Tui) => {
            match tui::start(&mut gpa) {
                Err(e) if was_interrupted(&*e) => Ok(()), // swallow Esc/Ctrl-C
                other => other,
            }?
        }
        None => {
            gpa.overview(args.filter, args.order_by, args.reverse, args.short);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
