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
