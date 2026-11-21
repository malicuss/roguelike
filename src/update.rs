use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn updadte(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        _ => {},
    }
}
