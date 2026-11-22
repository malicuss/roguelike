use crate::game::{Man, MonsterCtx, Room, helpers::{Direction,Pos}};
use color_eyre::Result;

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
    /// 10 ticks per secont
    pub const DEFAULT_TICK: u64 = 100;

    pub fn new() -> Self {
        Self::default()
    }

    ///
    pub fn tick(&mut self) {
        // ctx first
        let ctx: MonsterCtx = MonsterCtx {
            hero_position: self.hero.position,
            room_pos:self.active_room.size()
        };

        // tick to hero, always goes first
        self.hero.tick();

        // tick to monsters
        for m in &mut self.active_room.monsters {
            m.tick(&ctx);
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn move_hero(&mut self, direction: Direction) {

        self.hero.position.step(direction, Pos::new(self.active_room.width, self.active_room.height));
        let pos = self.hero.position.step(direction, Pos::new(self.active_room.width, self.active_room.height));
        self.hero.position = pos;
    }

    /// initializing world
    pub fn init(&mut self) -> Result<()> {
        self.hero = Man::default();
        self.rooms_visited = vec![];
        self.active_room = Room::new();
        self.active_room.init();
        self.hero
            .spawn_in_room(self.active_room.center());
        Ok(())
    }
}
