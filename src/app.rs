use color_eyre::Result;

use crate::hero::Man;
use crate::room::Room;

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

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true
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
