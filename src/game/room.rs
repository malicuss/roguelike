use crate::game::Monster;
use crate::game::helpers::{Location, Pos};

#[derive(Debug)]
enum DoorPostion {
    Centered,
    Left,
    Right,
}

/// Room with monsters.
#[derive(Debug, Default, Clone)]
pub struct Room {
    pub height: i16,
    pub width: i16,
    // connections to other rooms
    pub connections: Vec<Pos>,

    pub monsters: Vec<Monster>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            height: 80,
            width: 80,
            monsters: vec![],
            connections: vec![],
        }
    }

    pub fn size(&self) -> Pos {
        Pos::new(*&self.width, *&self.height)
    }

    pub fn center(&self) -> Pos {
        Pos::new(*&self.width / 2, *&self.height / 2)
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
    fn create_door(&self, location: Location, position: DoorPostion) -> Pos {
        let offset = |len: i16, pos| match pos {
            DoorPostion::Left => 3 * len / 4,
            DoorPostion::Centered => len / 2,
            DoorPostion::Right => len / 4,
        };

        match location {
            Location::North => Pos::new(offset(self.width, position), self.height),
            Location::South => Pos::new(offset(self.width, position), 0),
            Location::East => Pos::new(self.width, offset(self.height, position)),
            Location::West => Pos::new(0, offset(self.height, position)),
        }
    }
}
