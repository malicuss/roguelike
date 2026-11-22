mod game;
mod ui;

use color_eyre::Result;
use ratatui::{Terminal, backend::CrosstermBackend};

use game::App;
use ui::{Event::*, EventHandler, Tui, input::handle_key};
fn main() -> Result<()> {
    // create a new app
    let mut app = App::new();
    _ = app.init();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend);
    let events = EventHandler::new(App::DEFAULT_TICK);
    let mut tui = Tui::new(terminal?, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Tick => app.tick(),
            Key(key_event) => handle_key(&mut app, &key_event),
            //Mouse(_) => {}
            //Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
