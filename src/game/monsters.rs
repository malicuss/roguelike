use crate::game::ActionTimer;
use crate::game::Room;
use crate::game::helpers::ActionCost;
use crate::game::helpers::Direction;
use crate::game::helpers::Pos;

use rand::RngExt;
use rand::rng;

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
    pub hero_position: Pos,
    pub room_pos: Pos,
}

#[derive(Debug, Clone)]
pub struct Monster {
    /// position in room x,y
    pub position: Pos,
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
    direction: Direction,
}

impl Default for Monster {
    fn default() -> Self {
        Self::new()
    }
}

impl Monster {
    pub fn new() -> Monster {
        Self {
            position: Pos::default(),
            attack_area: 2,
            aggression_area: 6,
            notice_area: 17,
            action_timer: ActionTimer::new(1.0, 1.0),
            state: MonsterState::Wandering,
            state_ticks_left: 0,
            direction: Direction::InPlace,
        }
    }

    /// Spawns a monster in the room
    pub fn spawn(room: &Room) -> Self {
        let mut r = Self::new();
        let mut rng = rng();
        r.position = Pos::new(
            rng.random_range(0..room.width),
            rng.random_range(0..room.height),
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
        self.position = self.position.step(self.direction, ctx.room_pos);
    }

    fn behave_resolver(&mut self, ctx: &MonsterCtx) {
        self.action_timer.on_tick();

        let mut rng = rng();

        // state resolver goes here.
        match self.state {
            MonsterState::Idle => self.state = MonsterState::Wandering,
            MonsterState::Wandering => {
                if self.state_ticks_left > 0 {
                    self.state_ticks_left = self.state_ticks_left.saturating_sub(1);
                } else {
                    let dx = rng.random_range(-1..=1) as i16;
                    let dy = rng.random_range(-1..=1) as i16;
                    self.direction = Direction::from_vector((dx, dy));
                    self.state_ticks_left = rng.random_range(10..50);
                }

                if self.action_timer.try_spend_energy(ActionCost::Standart) {
                    self.position_change(ctx);
                }
            }
            MonsterState::Hunting => todo!(),
            MonsterState::Searching => todo!(),
        }
    }
}
