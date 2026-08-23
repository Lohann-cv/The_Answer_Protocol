use crate::class::{Player, PlayerClass, Stats};

impl Player {
    pub fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
        self.max_health = self.stats.stamina * 10;
        self.health = self.max_health;
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount; // We could add different exp gain depending on the class
        if self.experience >= self.level * 100 {
            self.level_up();
        }
    }
}