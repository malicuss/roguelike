use crate::action_timer::ActionTimer;

#[derive(Debug)]
pub struct Man {
    /// need to figure out how to implement fixed movement speed.
    /// 100 - is 100% on 4 hp
    /// 3 hp - it is 85%
    /// 2 hp - 75%
    /// 1 hp - 60%
    pub speed: u8,
    pub hp: u8,

    /// position in room x,y
    pub position: (u8, u8),

    pub action_timer: ActionTimer,
}

impl Man {
    pub fn new(&self) -> Man {
        Self::default()
    }

    pub fn tick(&mut self){
        // hero logic goes here
        // need to decide how "move" should work
    }
}

impl Default for Man {
    fn default() -> Self {
        Self {
            speed: 100,
            hp: 4,
            position: (0, 0),
            action_timer: ActionTimer::default()
        }
    }
}
