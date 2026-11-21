#[derive(Debug)]
pub struct Man{
    /// need to figure out how to implement fixed movement speed.
    /// 100 - is 100% on 4 hp
    /// 3 hp - it is 85%
    /// 2 hp - 75%
    /// 1 hp - 60%
    pub speed: u8,
    pub hp: u8,

    /// position in room x,y
    pub position: (u8,u8),
}

impl Man {
    pub fn new(&self)->Man{
        Self::default()
    }
}

impl Default for Man {
    fn default() -> Self {
        Self { speed: 100, hp: 4, position: (0,0) }
    }    
}