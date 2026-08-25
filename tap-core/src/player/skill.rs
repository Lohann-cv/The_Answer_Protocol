//! This module handles the player's combat skills and class-specific abilities.
//!
//! The damage and effects scale dynamically based on the player's class and stats.

use super::{Player, PlayerClass};

impl Player {
    /// Performs a basic attack on a target.
    ///
    /// The damage dealt scales with the primary stat of the player's class
    /// (e.g., Strength for Warriors, Agility for Archers).
    ///
    /// **Note:** If the player is a `Healer`, this method will heal the player instead of damaging the target.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tap_core::{Player, PlayerId, PlayerClass};
    /// // We create an attacker and a target
    /// let mut warrior = Player::new(PlayerId(1), PlayerClass::Warrior);
    /// let mut target = Player::new(PlayerId(2), PlayerClass::Tank);
    ///
    /// let initial_health = target.health;
    ///
    /// // The warrior attacks the target
    /// warrior.attack(&mut target);
    ///
    /// // The target has lost health
    /// assert!(target.health < initial_health);
    /// ```
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

    /// Casts the player's special ability on a target.
    ///
    /// This is a much stronger version of the basic attack, utilizing higher stat multipliers.
    /// Just like the basic attack, the `Healer` class will use this to strongly self-heal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tap_core::{Player, PlayerId, PlayerClass};
    /// let mut mage = Player::new(PlayerId(1), PlayerClass::Mage);
    /// let mut target = Player::new(PlayerId(2), PlayerClass::Warrior);
    ///
    /// let initial_health = target.health;
    ///
    /// // The mage casts a powerful spell
    /// mage.special_ability(&mut target);
    ///
    /// assert!(target.health < initial_health);
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PlayerId;

    /// Utils function to create a target
    fn create_dummy_target() -> Player {
        Player::new(PlayerId(99), PlayerClass::Tank)
    }

    #[test]
    fn test_warrior_basic_attack() {
        let mut warrior = Player::new(PlayerId(1), PlayerClass::Warrior);
        let mut target = create_dummy_target();

        let initial_health = target.health;

        // We attack the target
        warrior.attack(&mut target);

        // The warrior has 10 of strenght. The damage are of 10 * 2 = 20.
        // The target should loose 20 HP.
        let expected_damage = warrior.stats.strength * 2;
        assert_eq!(target.health, initial_health - expected_damage);
    }

    #[test]
    fn test_mage_special_ability() {
        let mut mage = Player::new(PlayerId(1), PlayerClass::Mage);
        let mut target = create_dummy_target();

        let initial_health = target.health;

        mage.special_ability(&mut target);

        // The mage is the best in attack with the thief
        let expected_damage = mage.stats.intelligence * 6;
        assert_eq!(target.health, initial_health - expected_damage);
    }

    #[test]
    fn test_healer_self_heals_instead_of_attacking() {
        let mut healer = Player::new(PlayerId(1), PlayerClass::Healer);
        let mut target = create_dummy_target();

        let target_initial_health = target.health;

        // We harm the healer
        healer.health -= 25;
        let healer_wounded_health = healer.health;

        // It attack
        healer.attack(&mut target);

        // And the target is unharmed
        assert_eq!(target.health, target_initial_health);

        // But the healer is healed
        let expected_healing = healer.stats.intelligence * 2;
        assert_eq!(healer.health, healer_wounded_health + expected_healing);
    }
}
