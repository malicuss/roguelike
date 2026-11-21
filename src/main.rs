use color_eyre::{eyre::Ok, eyre::Result};
use ratatui::{
    Terminal,
    backend::{CrosstermBackend},
};
use tui::Tui;

use crate::event::EventHandler;

pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;
pub mod monsters;
pub mod room;
pub mod hero;

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
            event::Event::Mouse(_) => {},
            event::Event::Resize(_, _) => todo!(),
        }
    }

    tui.exit()?;
    Ok(())
}
