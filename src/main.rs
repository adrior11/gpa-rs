#![allow(dead_code)]
mod commands;
mod file_util;
mod model;
mod prompts;
mod ui;
mod utils;

use std::time::Duration;

use clap::Parser;
use ratatui::crossterm::event::{self, Event, KeyEvent, KeyEventKind};

use commands::{Cli, Command};
use ui::{App, Message};

fn handle_keypress() -> anyhow::Result<Option<KeyEvent>> {
    let keypress = if event::poll(Duration::from_millis(500))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => Some(key),
            _ => None,
        }
    } else {
        None
    };
    Ok(keypress)
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut gpa = file_util::load_or_create_config()?;

    gpa.save(&file_util::get_config_path())?;

    match args.command {
        Some(Command::Settings) => {
            match prompts::start(&mut gpa) {
                Err(e) if utils::was_interrupted(&*e) => Ok(()),
                other => other,
            }?;
        }
        Some(Command::Tui) => {
            let mut app = App::new(gpa);
            let mut terminal = ratatui::init();
            loop {
                terminal.draw(|frame| {
                    let area = frame.area();
                    let buf = frame.buffer_mut();
                    app.render(area, buf);
                })?;

                if let Some(key) = handle_keypress()? {
                    match app.on_key(key) {
                        Ok(Message::Quit) => break,
                        Ok(Message::None) => { /* ignore */ }
                        Err(e) => {
                            eprintln!("Error: {e}");
                            break;
                        }
                    }
                }
            }
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
