use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        Color::{self, Green, Red, Yellow},
        Style,
    },
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
/// the actual drawer with all related stuff
pub fn render(app: &mut App, frame: &mut Frame) {
    // Split the whole screen into 3 horizontal zones, top to bottom:
    // Layout computes this once; after that every zone is a ready rectangle.
    let [map_area, _hud, debug_area] = Layout::vertical([
        //   map_area   — the map, takes all the remaining space (Fill(1));
        Constraint::Fill(1),
        //   _hud       — exactly 3 rows for the hero stats frame;
        Constraint::Length(3),
        //   debug_area — exactly 1 row for the debug output.
        Constraint::Length(1),
    ])
    .areas(frame.area());

    // Hero position in world coordinates (u8).
    let (hx, hy) = app.hero.position;
    // Camera: keep the hero at the screen center.
    // Camera coordinates are signed (i16) on purpose: when the hero reaches the
    // left/top edge of the room the camera goes negative and the hero stays
    // centered, while bare rock is drawn "beyond the world edge". Without the
    // sign (u16) the camera would clamp at 0 and the hero wouldn't stay centered.
    let cam_x = hx as i16 - map_area.width as i16 / 2;
    let cam_y = hy as i16 - map_area.height as i16 / 2;

    // The single "canvas" of the frame — the terminal character buffer.
    // Everything we write here is shown on the next draw.
    let buf = frame.buffer_mut();

    // "world -> screen" helper: converts world coords (u8) to screen coords (u16).
    // Returns None when the entity falls outside the visible window — that is
    // free clipping: no need to draw the whole room, only what is visible.
    // The map_area offset is added to the result so entities don't "slide"
    // whenever the layout changes.
    let to_screen = |x: u8, y: u8| -> Option<(u16, u16)> {
        let sx = x as i16 - cam_x;
        let sy = y as i16 - cam_y;
        if (0..map_area.width as i16).contains(&sx) && (0..map_area.height as i16).contains(&sy) {
            Some((map_area.x + sx as u16, map_area.y + sy as u16))
        } else {
            None
        }
    };

    // 1. Room floor and walls — the base layer; iterate over every cell of the map zone.
    // screen_x/screen_y are screen coords (u16, never negative),
    // world_x/world_y are world coords (i16, can be < 0 when the camera is past the left/top edge).
    for screen_y in 0..map_area.height {
        for screen_x in 0..map_area.width {
            let (world_x, world_y) = (cam_x + screen_x as i16, cam_y + screen_y as i16);
            // "inside" means the point falls within the room rectangle.
            // Everything else is infinite rock around it, so there is no need to
            // draw separate "walls on the left" — they follow from this check.
            let inside = world_x >= 0
                && world_y >= 0
                && world_x <= app.active_room.width as i16
                && world_y <= app.active_room.heitgh as i16;
            // cell_mut returns Option: None outside the buffer — extra protection
            // against panics, although by construction of the loop we are always inside map_area.
            if let Some(cell) = buf.cell_mut((map_area.x + screen_x, map_area.y + screen_y)) {
                if inside {
                    cell.set_char('.').set_fg(Color::Gray);
                } else {
                    cell.set_char('#').set_fg(Color::Black);
                }
            }
        }
    }
    // 2. Doors. Layer order = code order: whatever is drawn later
    // overwrites the previous layer (the floor is already drawn — the door goes on top of it).
    for &(door_x, door_y) in &app.active_room.connections {
        if let Some((x, y)) = to_screen(door_x, door_y)
            && let Some(cell) = buf.cell_mut((x, y))
        {
            cell.set_char('+').set_fg(Color::Blue);
        }
    }
    // Monsters — same scheme: world -> screen, None (off-camera) is simply skipped.
    for m in &app.active_room.monsters {
        // area coloring separate from monster
        let sx = m.position.0 as i16 - cam_x + map_area.x as i16;
        let sy = m.position.1 as i16 - cam_y + map_area.y as i16;
        area(buf, map_area, (sx, sy), m.notice_area, Green);
        area(buf, map_area, (sx, sy), m.aggression_area, Yellow);
        area(buf, map_area, (sx, sy), m.attack_area, Red);

        if let Some((x, y)) = to_screen(m.position.0, m.position.1) {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char('M').set_fg(Color::Red);
            }
        }
    }
    // The hero is drawn last among entities — always on top of floor, doors and monsters.
    if let Some((x, y)) = to_screen(hx, hy)
        && let Some(cell) = buf.cell_mut((x, y))
    {
        cell.set_char('@').set_fg(Color::Yellow);
    }
    // 5. HUD over the bottom 3 rows. render_widget is a different way of drawing:
    // the widget fills the area it is given, on top of whatever is already there
    // (that is why the Layout zones don't overlap map_area and the floor doesn't erase it).
    frame.render_widget(
        Paragraph::new(format!("HP: {} | Energy: {} | Esq — quit", app.hero.hp, app.hero.action_timer.get_energy()))
            .block(Block::default().borders(Borders::ALL)),
        _hud,
    );
    // 6. Debug line under the HUD: hero and camera coordinates are handy for
    // eyeballing that the camera and clipping behave as intended.
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

fn area(buff: &mut Buffer, map_area: Rect, position: (i16, i16), area: u8, color: Color) {
    let r = area as i32;
    let (cx, cy) = (position.0 as i32, position.1 as i32);
    for dy in -r..=r {
        for dx in -r..=r {
            // bounding box + circle filter; 4*dy*dy — compensation could be done
            // should i use it or not ?
            if dx * dx + dy * dy > r * r {
                continue;
            }
            // offscreen is negative
            let (Ok(x), Ok(y)) = (u16::try_from(cx + dx), u16::try_from(cy + dy)) else {
                continue;
            };
            // outside the map zone — into HUD/debug
            if !map_area.contains(ratatui::layout::Position { x, y }) {
                continue;
            }
            if let Some(cell) = buff.cell_mut((x, y))
                && (cell.symbol() == "." || cell.symbol() == ",")
            {
                cell.set_char(',').set_fg(color);
            }
        }
    }
}
