use rand::RngExt;
use rand::rng;

use crate::action_timer::{ActionCost, ActionTimer};
use crate::room::Room;

#[derive(Debug, Clone, Copy)]
enum MonsterMovement {
    Stand,
    Walk,
    Run,
}

#[derive(Debug, Clone, Copy)]
enum MonsterState {
    Idle,      // waiting on place.
    Wandering, // random wandering 10 - 50 ticks direction not changing
    Hunting,   // hero noticed - moving to hero
    Searching, // no hero in vicinity, moving to last known position, 3-5 sec Idle
}

#[derive(Debug, Clone, Copy)]
pub struct MonsterCtx {
    pub hero_position: (u8, u8),
    pub width: u8,
    pub height: u8,
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub attack_speed: f32,
    pub walk_speed: f32,
    pub hp: u8,
    pub is_alive: bool,
    /// resurection cooldown in sec. if -1 not resurectable.
    pub res_cd_sec: i32,
    /// position in room x,y
    pub position: (u8, u8),
    /// how far can it bite
    pub attack_area: u8,
    /// how far can it agro
    pub aggression_area: u8,
    /// how far it can see
    pub notice_area: u8,

    pub action_timer: ActionTimer,

    ///behave logic, to be extracted
    // state
    state: MonsterState,
    // how many tick til end of current state.
    state_ticks_left: u8,
    // direction
    direction: (i16, i16),
}

impl Monster {
    pub fn new() -> Monster {
        Self {
            attack_speed: 0.6,
            walk_speed: 0.7,
            hp: 10,
            is_alive: true,
            res_cd_sec: -1,
            position: (0, 0),
            attack_area: 2,
            aggression_area: 6,
            notice_area: 17,
            action_timer: ActionTimer::new(0.7, 2.0),
            state: MonsterState::Wandering,
            state_ticks_left: 0,
            direction: (0, 0),
        }
    }

    pub fn spawn(room: &Room) -> Self {
        let mut r = Self::new();
        let mut rng = rng();
        r.position = (
            rng.random_range(0..room.width),
            rng.random_range(0..room.heitgh),
        );
        r
    }

    pub fn tick(&mut self, ctx: &MonsterCtx) {
        // monster (BASIC) logic goes here
        // think how to do this logic modifiable.

        // where to go if no hero in vicinity - random
        self.behave_resolver(ctx);
        // hero in vicinity - walk to him
        // hero in aggro zone - run to him
        // hero in bite zone - bite him
        // if run is not an option - walk
        // if bite is not an option but walk is available - waite and bit if zone is bite, walk if zone is agro
    }

    fn position_change(&mut self, ctx: &MonsterCtx) {
        let x = (self.position.0 as i16 + self.direction.0).clamp(0, ctx.width as i16) as u8;
        let y = (self.position.1 as i16 + self.direction.1).clamp(0, ctx.height as i16) as u8;
        self.position = (x, y);
    }

    fn behave_resolver(&mut self, ctx: &MonsterCtx) {
        self.action_timer.on_tick();

        let mut rng = rng();

        // state resolver goes here.

        match self.state {
            MonsterState::Idle => todo!(),
            MonsterState::Wandering => {
                if self.state_ticks_left > 0 {
                    self.state_ticks_left = self.state_ticks_left.saturating_sub(1);
                } else {
                    let dx = rng.random_range(-1..=1) as i16;
                    let dy = rng.random_range(-1..=1) as i16;
                    self.direction = (dx, dy);
                    self.state_ticks_left = rng.random_range(10..50);
                }

                if self.action_timer.try_spend_energy(ActionCost::Doubled) {
                    self.position_change(ctx);
                }
            }
            MonsterState::Hunting => todo!(),
            MonsterState::Searching => todo!(),
        }
    }
}

impl Default for Monster {
    fn default() -> Self {
        Self::new()
    }
}
