//! This module provides the core health manipulation logic for a player.
//!
//! It ensures that health values stay within safe bounds (0 to max_health)
//! during combat and healing events.

use crate::player::Player;

impl Player {
    /// Inflicts damage to the player, reducing their current health.
    ///
    /// If the damage amount exceeds or equals the player's current health,
    /// the health is safely set to 0 to prevent underflow panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tap_core::{Player, PlayerId, PlayerClass};
    /// let mut player = Player::new(PlayerId(1), PlayerClass::Warrior);
    /// let initial_health = player.health;
    ///
    /// // The player takes 5 damage
    /// player.take_damage(5);
    /// assert_eq!(player.health, initial_health - 5);
    /// ```
    pub fn take_damage(&mut self, amount: u32) {
        if amount >= self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }

    /// Restores health points to the player.
    ///
    /// The player's health cannot exceed their `max_health` capacity.
    /// Any excess healing is automatically ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tap_core::{Player, PlayerId, PlayerClass};
    /// let mut player = Player::new(PlayerId(1), PlayerClass::Warrior);
    ///
    /// // We damage the player first
    /// player.health -= 10;
    ///
    /// // We heal the player by 5
    /// player.heal(5);
    /// assert_eq!(player.health, player.max_health - 5);
    /// ```
    pub fn heal(&mut self, amount: u32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}
