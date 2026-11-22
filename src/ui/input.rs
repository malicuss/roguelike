use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::game::App;
use crate::game::helpers::*;

pub fn handle_key(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        KeyCode::Char('s') | KeyCode::Char('2') => app.move_hero(Direction::Down),
        KeyCode::Char('w') | KeyCode::Char('8') => app.move_hero(Direction::Up),
        KeyCode::Char('a') | KeyCode::Char('4') => app.move_hero(Direction::Left),
        KeyCode::Char('d') | KeyCode::Char('6') => app.move_hero(Direction::Right),
        KeyCode::Char('e') | KeyCode::Char('9') => app.move_hero(Direction::UpRight),
        KeyCode::Char('q') | KeyCode::Char('7') => app.move_hero(Direction::UpLeft),
        KeyCode::Char('c') | KeyCode::Char('3') => app.move_hero(Direction::DownRight),
        KeyCode::Char('z') | KeyCode::Char('1') => app.move_hero(Direction::DownLeft),
        _ => {}
    }
}
