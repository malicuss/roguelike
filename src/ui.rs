use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
/// the actual drawer with all related stuff
pub fn render(app: &mut App, frame: &mut Frame) {
    let [map_area, _hud, debug_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let (hx, hy) = app.hero.position;
    // камера: герой в центре экрана
    let cam_x = hx as i16 - map_area.width as i16 / 2;
    let cam_y = hy as i16 - map_area.height as i16 / 2;

    let buf = frame.buffer_mut();

    // мир (u8) -> экран (u16); None если сущность вне экрана
    let to_screen = |x: u8, y: u8| -> Option<(u16, u16)> {
        let sx = x as i16 - cam_x;
        let sy = y as i16 - cam_y;
        if (0..map_area.width as i16).contains(&sx) && (0..map_area.height as i16).contains(&sy) {
            Some((map_area.x + sx as u16, map_area.y + sy as u16))
        } else {
            None
        }
    };

    // 1. пол и стены комнаты
    for sy in 0..map_area.height {
        for sx in 0..map_area.width {
            let (wx, wy) = (cam_x + sx as i16, cam_y + sy as i16);
            let inside = wx >= 0
                && wy >= 0
                && wx <= app.active_room.width as i16
                && wy <= app.active_room.heitgh as i16;
            if let Some(cell) = buf.cell_mut((map_area.x + sx, map_area.y + sy)) {
                if inside {
                    cell.set_char('.').set_fg(Color::DarkGray);
                } else {
                    cell.set_char('#').set_fg(Color::DarkGray);
                }
            }
        }
    }
    // 2. двери
    for &(dx, dy) in &app.active_room.connections {
        if let Some((x, y)) = to_screen(dx, dy) {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char('+').set_fg(Color::Blue);
            }
        }
    }
    // монстры — та же схема
    for m in &app.active_room.monsters {
        if let Some((x, y)) = to_screen(m.position.0, m.position.1) {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char('M').set_fg(Color::Red);
            }
        }
    }
    // герой
    if let Some((x, y)) = to_screen(hx, hy) {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char('@').set_fg(Color::Yellow);
        }
    }
    // 5. HUD поверх нижних 3 строк
    frame.render_widget(
        Paragraph::new(format!("HP: {} | q — quit", app.hero.hp))
            .block(Block::default().borders(Borders::ALL)),
        _hud,
    );
    // 6. дебаг-строка под HUD
    frame.render_widget(
        Paragraph::new(format!(
            "hero:({hx},{hy}) cam:({cam_x},{cam_y}) room:{}x{} monsters:{} doors:{} counter:{}",
            app.active_room.width,
            app.active_room.heitgh,
            app.active_room.monsters.len(),
            app.active_room.connections.len(),
            app.counter,
        ))
        .style(Style::default().fg(Color::DarkGray)),
        debug_area,
    );
}
