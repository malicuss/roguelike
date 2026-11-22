/// 20 standart ticks per second
#[derive(Debug)]
pub struct ActionTimer {
    energy: u32,           // 1000 Energy
    fill_rate: u32, // fill energy rate enrgy/per tick, 50 per tick

    energy_modifier: f32,
    fill_rate_modifier: f32,
}

impl Default for ActionTimer {
    fn default() -> Self {
        Self {
            energy: ActionTimer::DEFAULT_ENERGY,
            fill_rate: ActionTimer::DEFAULT_FILL_RATE,
            energy_modifier: 1.0,
            fill_rate_modifier: 1.0,
        }
    }
}

impl Clone for ActionTimer {
    fn clone(&self) -> Self {
        Self { energy: self.energy.clone(), fill_rate: self.fill_rate.clone(), energy_modifier: 1.0, fill_rate_modifier:1.0 }
    }
}

impl ActionTimer {
    pub const DEFAULT_ENERGY: u32 = 1000;
    pub const DEFAULT_FILL_RATE: u32 = 50;

    pub fn new(energy_delta: f32, fill_rate_delta: f32) -> ActionTimer {
        Self {
            energy: (ActionTimer::DEFAULT_ENERGY as f32 * energy_delta) as u32,
            fill_rate: (ActionTimer::DEFAULT_FILL_RATE as f32 * fill_rate_delta) as u32,
            energy_modifier: 1.0,
            fill_rate_modifier: 1.0,
        }
    }

    pub fn update_energy_modifier(&mut self, modifier: f32) {
        self.energy_modifier += modifier;
    }

    pub fn update_fill_rate(&mut self, modifier: f32) {
        self.fill_rate_modifier += modifier;
    }

    pub fn set_default(&mut self){
        self.energy_modifier = 1.0;
        self.fill_rate_modifier = 1.0;
    }

    pub fn get_energy(&self)->u32{
        (self.energy as f32 * self.energy_modifier) as u32
    }

    pub fn get_fill_rate(&self)->u32{
        (self.fill_rate as f32 * self.fill_rate_modifier) as u32
    }
}
