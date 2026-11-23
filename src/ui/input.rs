use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::game::App;
use crate::game::helpers::*;

pub fn handle_key(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') if key.modifiers == KeyModifiers::CONTROL => {
            app.quit()
        }
        KeyCode::Char('s') | KeyCode::Char('2') => app.hero.move_hero(Direction::Down, &app.active_room),
        KeyCode::Char('w') | KeyCode::Char('8') => app.hero.move_hero(Direction::Up, &app.active_room),
        KeyCode::Char('a') | KeyCode::Char('4') => app.hero.move_hero(Direction::Left, &app.active_room),
        KeyCode::Char('d') | KeyCode::Char('6') => app.hero.move_hero(Direction::Right, &app.active_room),
        KeyCode::Char('e') | KeyCode::Char('9') => app.hero.move_hero(Direction::UpRight, &app.active_room),
        KeyCode::Char('q') | KeyCode::Char('7') => app.hero.move_hero(Direction::UpLeft, &app.active_room),
        KeyCode::Char('c') | KeyCode::Char('3') => app.hero.move_hero(Direction::DownRight, &app.active_room),
        KeyCode::Char('z') | KeyCode::Char('1') => app.hero.move_hero(Direction::DownLeft, &app.active_room),
        _ => {}
    }
}
