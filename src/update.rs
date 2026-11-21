use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn updadte(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        KeyCode::Char('s')=>app.move_hero(crate::room::Direction::Down),
        KeyCode::Char('w')=>app.move_hero(crate::room::Direction::Up),
        KeyCode::Char('a')=>app.move_hero(crate::room::Direction::Left),
        KeyCode::Char('d')=>app.move_hero(crate::room::Direction::Right),
        _ => {},
    }
}
