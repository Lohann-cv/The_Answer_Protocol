use crate::player::Player;

impl Player {
    pub fn take_damage(&mut self, amount: u32) {
        if amount >= self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}
