use color_eyre::Result;

use crate::man::Man;
use crate::monsters::MonsterCtx;
use crate::room::{Direction, Room};

/// Graveyard. night. man vs skeletons. skeletons - slow and steady(speed always the same). man - hero. no fight, just run. hp - 4 hits.
/// graveyard 20 zones 150x140. labirinth. Rooms appears when they entered.
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
    /// each N ms tick appear
    pub const DEFAULT_TICK: u64 = 100;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        // ctx first
        let ctx: MonsterCtx = MonsterCtx { hero_position: self.hero.position, width: self.active_room.width, height: self.active_room.heitgh };

        // tick to hero, always goes first
        self.hero.tick();

        // tick to monsters
        for m in &mut self.active_room.monsters{
            m.tick(&ctx);
        }
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
        self.active_room = Room::new();
        self.active_room.init();
        self.hero.spawn_in_room((self.active_room.width/2,self.active_room.heitgh/2));
        Ok(())
    }
}
