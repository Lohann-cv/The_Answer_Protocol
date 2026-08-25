use super::{Player, PlayerClass};

impl Player {
    pub fn attack(&mut self, target: &mut Player) {
        match self.class {
            PlayerClass::Warrior => {
                let damage = self.stats.strength * 2;
                target.take_damage(damage);
            }
            PlayerClass::Mage => {
                let damage = self.stats.intelligence * 3;
                target.take_damage(damage);
            }
            PlayerClass::Archer => {
                let damage = self.stats.agility * 2;
                target.take_damage(damage);
            }
            PlayerClass::Summoner => {
                let damage = self.stats.intelligence * 2;
                target.take_damage(damage);
            }
            PlayerClass::Healer => {
                let healing = self.stats.intelligence * 2;
                self.heal(healing);
            }
            PlayerClass::Tank => {
                let damage = self.stats.stamina * 2;
                target.take_damage(damage);
            }
            PlayerClass::Thief => {
                let damage = self.stats.agility * 3;
                target.take_damage(damage);
            }
        }
    }

    pub fn special_ability(&mut self, target: &mut Player) {
        match self.class {
            PlayerClass::Warrior => {
                let damage = self.stats.strength * 5;
                target.take_damage(damage);
            }
            PlayerClass::Mage => {
                let damage = self.stats.intelligence * 6;
                target.take_damage(damage);
            }
            PlayerClass::Archer => {
                let damage = self.stats.agility * 4;
                target.take_damage(damage);
            }
            PlayerClass::Summoner => {
                let damage = self.stats.intelligence * 5;
                target.take_damage(damage);
            }
            PlayerClass::Healer => {
                let healing = self.stats.intelligence * 5;
                self.heal(healing);
            }
            PlayerClass::Tank => {
                let damage = self.stats.stamina * 5;
                target.take_damage(damage);
            }
            PlayerClass::Thief => {
                let damage = self.stats.agility * 6;
                target.take_damage(damage);
            }
        }
    }
}
