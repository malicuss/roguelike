use color_eyre::Result;

use crate::{man::Man, monsters::Monster};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up, Right, Down, Left,
    UpRight, UpLeft, DownRight, DownLeft,
}
impl Direction {
    pub fn Vector(self)->(i8,i8){
        match self {
            Self::Up => (0, -1),   Self::Down => (0, 1),
            Self::Left => (-1, 0), Self::Right => (1, 0),
            Self::UpRight => (1, -1),  Self::UpLeft => (-1, -1),
            Self::DownRight => (1, 1), Self::DownLeft => (-1, 1),
        }
    }
}

#[derive(Debug)]
pub enum Location {
    North,
    East,
    West,
    South
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
    pub fn init(&mut self) {
        self.monsters.push(Monster::spawn(self));
        self.connections
            .push(self.create_door(Location::East, DoorPostion::Right));
        self.connections
            .push(self.create_door(Location::East, DoorPostion::Left));
        self.connections
            .push(self.create_door(Location::North, DoorPostion::Centered));
    }

    /// we always looking down and right
    fn create_door(&self, location: Location, position: DoorPostion) -> (u8, u8) {
        let offset = |len: u8, pos| match pos {
            DoorPostion::Left => 3 * len / 4,
            DoorPostion::Centered => len / 2,
            DoorPostion::Right => len / 4,
        };

        match location {
            Location::North => (offset(self.width, position), self.heitgh),
            Location::South => (offset(self.width, position), 0),
            Location::East => (self.width, offset(self.heitgh, position)),
            Location::West => (0, offset(self.heitgh, position))
        }
    }
    fn debug_position_centered(&self) -> (u8, u8) {
        (self.width / 2, self.heitgh / 2)
    }
}
