mod commands;
mod file_util;
mod model;
mod ui;

use clap::Parser;

use commands::{Cli, Command};
use ui::App;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut gpa = file_util::load_or_create_config()?;

    match args.command {
        Some(Command::Tui) => {
            let mut app = App::default();
            let mut terminal = ratatui::init();
            app.run(&mut gpa, &mut terminal)?;
            ratatui::restore();
        }
        None => {
            gpa.overview(
                &mut std::io::stdout(),
                args.filter.as_ref(),
                &args.columns(),
                &args.order_by,
                args.reverse,
                args.short,
            )?;
        }
    }

    Ok(())
}
