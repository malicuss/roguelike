#[derive(Debug, Clone, Copy)]
pub enum Direction {
    InPlace,
    Up,
    Right,
    Down,
    Left,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}
impl Direction {
    pub fn vector(self) -> (i16, i16) {
        match self {
            Self::InPlace => (0, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::UpRight => (1, -1),
            Self::UpLeft => (-1, -1),
            Self::DownRight => (1, 1),
            Self::DownLeft => (-1, 1),
        }
    }
    pub fn from_vector(v: (i16, i16)) -> Self {
        match v {
            (0, -1) => Self::Up,
            (0, 1) => Self::Down,
            (-1, 0) => Self::Left,
            (1, 0) => Self::Right,
            (1, -1) => Self::UpRight,
            (-1, -1) => Self::UpLeft,
            (1, 1) => Self::DownRight,
            (-1, 1) => Self::DownLeft,
            _ => Self::InPlace,
        }
    }
}

#[derive(Debug)]
pub enum Location {
    North,
    East,
    West,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl Pos {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn from_dir(dir: Direction) -> Pos {
        let (dx, dy) = dir.vector();
        Pos::new(dx as i16, dy as i16)
    }

    pub fn to_tuple(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    pub fn add(self, d: Pos) -> Pos {
        Pos::new(self.x + d.x, self.y + d.y)
    }

    pub fn step(self, dir: Direction, pos: Pos) -> Pos {
        let next = self.add(Pos::from_dir(dir));
        Pos::new(next.x.clamp(0, pos.x), next.y.clamp(0, pos.y))
    }
}
