use color_eyre::section;

use crate::game::{
    Room,
    action_timer::ActionTimer,
    helpers::{ActionCost, Direction, Pos},
};

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
    movement_buf: Option<(Direction, Pos)>,
}

impl Man {
    pub fn tick(&mut self) {
        // hero logic goes here
        // need to decide how "move" should work
        self.action_timer.on_tick();
        if let Some(dir) = self.movement_buf {
            let cost = match dir.0 {
                Direction::UpRight
                | Direction::UpLeft
                | Direction::DownRight
                | Direction::DownLeft => ActionCost::Increased,
                _ => ActionCost::Standart,
            };
            if self.action_timer.try_spend_energy(cost) {
                self.move_pending();
                self.movement_buf = None;
            }
        }
    }

    pub fn spawn_in_room(&mut self, position: Pos) {
        self.position = position;
    }

    /// pending movement
    pub fn move_hero(&mut self, direction: Direction, active_room: &Room) {
        self.movement_buf = Some((direction, active_room.size()));
    }

    fn move_pending(&mut self) {
        if let Some(buf) = self.movement_buf {
            self.position.step(buf.0, Pos::new(buf.1.x, buf.1.y));
            let pos = self.position.step(buf.0, Pos::new(buf.1.x, buf.1.y));
            self.position = pos;
            self.movement_buf = None;
        }
    }
}

impl Default for Man {
    fn default() -> Self {
        Self {
            //speed: 100,
            hp: 4,
            position: Pos::default(),
            action_timer: ActionTimer::default(),
            movement_buf: None,
        }
    }
}
