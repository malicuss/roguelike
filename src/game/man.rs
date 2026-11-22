use crate::game::{action_timer::ActionTimer, helpers::Pos};

#[derive(Debug)]
pub struct Man {
    /// need to figure out how to implement fixed movement speed.
    /// 100 - is 100% on 4 hp
    /// 3 hp - it is 85%
    /// 2 hp - 75%
    /// 1 hp - 60%
    // pub speed: u8,
    pub hp: u8,

    /// position in room x,y
    pub position: Pos,

    pub action_timer: ActionTimer,
}

impl Man {
    pub fn tick(&mut self) {
        // hero logic goes here
        // need to decide how "move" should work
        self.action_timer.on_tick();
    }

    pub fn spawn_in_room(&mut self, position: Pos) {
        self.position = position;
    }
}

impl Default for Man {
    fn default() -> Self {
        Self {
            //speed: 100,
            hp: 4,
            position: Pos::default(),
            action_timer: ActionTimer::default(),
        }
    }
}
