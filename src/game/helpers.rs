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