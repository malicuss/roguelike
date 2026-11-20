use std::{io::IsTerminal, ops::Add};

use color_eyre::{eyre::Ok, eyre::Result};
use ratatui::{
    Terminal,
    backend::{self, CrosstermBackend},
};
use tui::Tui;

use crate::event::EventHandler;
/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

fn main() -> Result<()> {
    // create a new app
    let mut app = app::App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend);
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal?, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            event::Event::Tick => {}
            event::Event::Key(key_event) => update::updadte(&mut app, &key_event),
            event::Event::Mouse(mouse_event) => {},
            event::Event::Resize(_, _) => todo!(),
        }
    }

    tui.exit()?;
    Ok(())
}
