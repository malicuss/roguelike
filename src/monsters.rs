use rand::RngExt;
use rand::rng;

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
}

impl Default for Monster {
    fn default() -> Self {
        Self::new()
    }
}
