use rand::RngExt;
use rand::rng;

use crate::action_timer::ActionTimer;
use crate::room::Room;

#[derive(Debug, Clone)]
pub struct Monster {
    pub attack_speed: u8,
    pub walk_speed: u8,
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
    
}

impl Monster {
    pub fn new() -> Monster {
        Self {
            attack_speed: 60,
            walk_speed: 70,
            hp: 10,
            is_alive: true,
            res_cd_sec: -1,
            position: (0, 0),
            attack_area: 2,
            aggression_area: 6,
            notice_area: 17,
            action_timer: ActionTimer::new(0.7, 0.5),
        }
    }

    pub fn spawn( room: &Room) -> Self {
        let mut r = Self::new();
        let mut rng = rng();
        r.position = (
            rng.random_range(0..room.width),
            rng.random_range(0..room.heitgh),
        );
        r
    }

    pub fn tick(&mut self){
        // monster (BASIC) logic goes here
        // think how to do this logic modifiable.

        // where to go if no hero in vicinity - random
        // hero in vicinity - walk to him
        // hero in aggro zone - run to him
        // hero in bite zone - bite him
        // if run is not an option - walk
        // if bite is not an option but walk is available - waite and bit if zone is bite, walk if zone is agro
    }
}

impl Default for Monster {
    fn default() -> Self {
        Self::new()
    }
}
