/// default Energy spent is 100 per action
#[derive(Debug, Clone)]
pub enum ActionCost {
    Free,        // 0
    Lowered,     // 0.5
    Standart,    // 1
    Increased,   // 1,3
    Doubled,     // 2
    Tripled,     // 3
    Overwhelmed, // all or nothing
}

/// 20 standart ticks per second
#[derive(Debug)]
pub struct ActionTimer {
    energy: u8,    // 100 Energy
    fill_rate: u8, // fill energy rate enrgy/per tick, 10 per tick

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
        Self {
            energy: self.energy.clone(),
            fill_rate: self.fill_rate.clone(),
            energy_modifier: 1.0,
            fill_rate_modifier: 1.0,
        }
    }
}

impl ActionTimer {
    pub const DEFAULT_ENERGY: u8 = 100;
    pub const DEFAULT_FILL_RATE: u8 = 10;
    /// 2 sec for full energy fill
    const DEFAULT_ENERGY_SPENT: u8 = 50;

    pub fn new(energy_delta: f32, fill_rate_delta: f32) -> ActionTimer {
        Self {
            energy: (ActionTimer::DEFAULT_ENERGY as f32 * energy_delta) as u8,
            fill_rate: (ActionTimer::DEFAULT_FILL_RATE as f32 * fill_rate_delta) as u8,
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

    pub fn set_default(&mut self) {
        self.energy_modifier = 1.0;
        self.fill_rate_modifier = 1.0;
    }

    pub fn get_energy(&self) -> u32 {
        (self.energy as f32 * self.energy_modifier) as u32
    }

    pub fn get_fill_rate(&self) -> u32 {
        (self.fill_rate as f32 * self.fill_rate_modifier) as u32
    }

    pub fn try_spend_energy(&mut self, action_cost: ActionCost) -> bool {
        let mut cost: u8 = 0;
        match action_cost {
            ActionCost::Free => {}
            ActionCost::Lowered => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 0.5) as u8,
            ActionCost::Standart => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 1.0) as u8,
            ActionCost::Increased => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 1.3) as u8,
            ActionCost::Doubled => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 2.0) as u8,
            ActionCost::Tripled => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 3.0) as u8,
            ActionCost::Overwhelmed => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 100.0) as u8,
        }

        if self.energy >= cost {
            self.energy = self.energy.saturating_sub(cost);
            true
        } else {
            false
        }
    }

    pub fn on_tick(&mut self) {
        let nrg_per_tick = (self.fill_rate as f32 * self.fill_rate_modifier) as u8;
        self.energy = self.energy.saturating_add(nrg_per_tick);
    }
}
