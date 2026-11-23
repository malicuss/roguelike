use crate::game::helpers::*;

/// 20 standart ticks per second
#[derive(Debug)]
pub struct ActionTimer {
    energy_cap:u16,
    energy_current: u16,
    fill_rate: u8,
    energy_modifier: f32,
    fill_rate_modifier: f32,
}

impl Default for ActionTimer {
    fn default() -> Self {
        Self {
            energy_cap: ActionTimer::DEFAULT_ENERGY,
            energy_current: ActionTimer::DEFAULT_ENERGY,
            fill_rate: ActionTimer::DEFAULT_FILL_RATE,
            energy_modifier: 1.0,
            fill_rate_modifier: 1.0,
        }
    }
}

impl Clone for ActionTimer {
    fn clone(&self) -> Self {
        Self {
            energy_cap: self.energy_cap.clone(),
            energy_current: self.energy_current.clone(),
            fill_rate: self.fill_rate.clone(),
            energy_modifier: 1.0,
            fill_rate_modifier: 1.0,
        }
    }
}

impl ActionTimer {
    const DEFAULT_ENERGY: u16 = 100;
    const DEFAULT_FILL_RATE: u8 = 20;
    const DEFAULT_ENERGY_SPENT: u8 = 50;

    /// delta could be uniqe for monster type.
    pub fn new(energy_delta: f32, fill_rate_delta: f32) -> ActionTimer {
        Self {
            energy_cap: (ActionTimer::DEFAULT_ENERGY as f32 * energy_delta) as u16,
            energy_current: (ActionTimer::DEFAULT_ENERGY as f32 * energy_delta) as u16,
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

    /// Get actual energy for monster instance
    pub fn get_energy(&self) -> u32 {
        self.energy_current as u32
    }

    /// Get actual energy fill rate for monster instance
    pub fn get_fill_rate(&self) -> u32 {
        (self.fill_rate as f32 * self.fill_rate_modifier) as u32
    }

    /// energy for each action.
    pub fn try_spend_energy(&mut self, action_cost: ActionCost) -> bool {
        let mut cost: u16 = 0;
        match action_cost {
            ActionCost::Free => {}
            ActionCost::Lowered => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 0.5) as u16,
            ActionCost::Standart => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 1.0) as u16,
            ActionCost::Increased => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 1.4) as u16,
            ActionCost::Doubled => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 2.0) as u16,
            ActionCost::Tripled => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 3.0) as u16,
            ActionCost::Overwhelmed => cost = (Self::DEFAULT_ENERGY_SPENT as f32 * 100.0) as u16,
        }

        if self.energy_current >= cost {
            self.energy_current = self.energy_current.saturating_sub(cost);
            true
        } else {
            false
        }
    }

    /// runs every tick.
    pub fn on_tick(&mut self) {
        let nrg_per_tick = (self.fill_rate as f32 * self.fill_rate_modifier) as u16;
        self.energy_current = self.energy_current.saturating_add(nrg_per_tick).min(self.energy_cap);
    }
}
