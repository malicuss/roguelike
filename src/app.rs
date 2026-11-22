use color_eyre::Result;

use crate::man::Man;
use crate::room::{Direction, Room};

/// Graveyard. night. man vs skeletons. skeletons - slow and steady(speed always the same). man - hero. no fight, just run. hp - 4 hits.
/// graveyard 20 zones 5x4. labirinth. Rooms appears when they entered.
///
/// App should store all main application logic, not related to drawing and all that stuff
#[derive(Debug, Default)]
pub struct App {
    // game fns:
    pub hero: Man,
    pub rooms_visited: Vec<Room>,
    pub active_room: Room,

    // nogame fns:
    /// should app quit
    pub should_quit: bool,
    /// counter
    pub counter: u8,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        // tick to hero, always goes first

        // tick to monsters
    }

    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn move_hero(&mut self, direction: Direction) {
        let (dx, dy) = direction.Vector();
        let x =
            (self.hero.position.0 as i16 + dx as i16).clamp(0, self.active_room.width as i16) as u8;
        let y = (self.hero.position.1 as i16 + dy as i16).clamp(0, self.active_room.heitgh as i16)
            as u8;
        self.hero.position = (x, y);
    }

    pub fn init(&mut self) -> Result<()> {
        self.hero = Man::default();
        self.rooms_visited = vec![];

        self.active_room = Room::new()
            .init()
            .expect("faild too init room")
            .spawn_hero(&mut self.hero)
            .expect("faild to place hero");
        Ok(())
    }
}
