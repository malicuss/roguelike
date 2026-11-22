use color_eyre::Result;

use crate::{hero::Man, monsters::Monster};

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum DoorPostion {
    Centered,
    Left,
    Right,
}

/// Room with monsters.
#[derive(Debug, Default, Clone)]
pub struct Room {
    pub heitgh: u8,
    pub width: u8,
    // connections to other rooms
    pub connections: Vec<(u8, u8)>,

    pub monsters: Vec<Monster>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            heitgh: 80,
            width: 80,
            monsters: vec![],
            connections: vec![],
        }
    }

    // what should be in room: connection (enter/exit) up to 5, monsters, some stuff(later)
    pub fn init(&mut self) -> Result<&Self> {
        self.monsters.push(Monster::spawn(self));
        self.connections
            .push(self.create_door(Direction::Right, DoorPostion::Right));
        self.connections
            .push(self.create_door(Direction::Right, DoorPostion::Left));
        Ok(self)
    }

    /// Span hero for first time or after death. DEBUG_POSITION to be removed.
    pub fn spawn_hero(&self, hero: &mut Man) -> Result<Self> {
        hero.position = self.debug_position_centered();
        Ok(self.clone())
    }

    /// we always looking down and right
    fn create_door(&self, location: Direction, position: DoorPostion) -> (u8, u8) {
        let offset = |len: u8, pos| match pos {
            DoorPostion::Left => 3 * len / 4,
            DoorPostion::Centered => len / 2,
            DoorPostion::Right => len / 4,
        };

        match location {
            Direction::Up => (offset(self.width, position), self.heitgh),
            Direction::Down => (offset(self.width, position), 0),
            Direction::Right => (self.width, offset(self.heitgh, position)),
            Direction::Left => (0, offset(self.heitgh, position)),
        }
    }
    fn debug_position_centered(&self) -> (u8, u8) {
        (self.width / 2, self.heitgh / 2)
    }
}
