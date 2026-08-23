use super::{BASE_EXP_REQUIRED, HEALTH_MULTIPLIER, MAX_LEVEL};
use super::{Player, PlayerClass, Stats};

impl Player {
    pub fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
        self.max_health = self.stats.stamina * HEALTH_MULTIPLIER;
        self.health = self.max_health;
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount; // We could add different exp gain depending on the class
        if self.experience >= self.level * BASE_EXP_REQUIRED && self.level < MAX_LEVEL {
            self.level_up();
        }
    }
}
