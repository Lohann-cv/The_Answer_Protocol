//! This module holds the EXP progression logic
//! The level up is exponentially hard to get
//! For now in this version the player leftover EXP isn't kept when leveling up

use super::Player;
use super::{BASE_EXP_REQUIRED, HEALTH_MULTIPLIER, MAX_LEVEL};

impl Player {
    /// Levels up the player.
    ///
    /// This function is called by 'gain_experience' when the needed EXP is reached.
    /// It also increases the player max health.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tap_core::{Player, PlayerId, PlayerClass};
    ///
    /// // We create a player that is level 1
    /// let mut player = Player::new(PlayerId(1), PlayerClass::Warrior);
    /// assert_eq!(player.level, 1);
    ///
    /// // We simulate damage
    /// player.health -= 10;
    ///
    /// // Then level up
    /// player.level_up();
    ///
    /// // The level increases and health is fully restored
    /// assert_eq!(player.level, 2);
    /// assert_eq!(player.health, player.max_health);
    /// ```
    pub fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
        self.max_health = self.stats.stamina * HEALTH_MULTIPLIER;
        self.health = self.max_health;
    }

    /// Make the player gain experience.
    ///
    /// It will be exponentially harder for the player to gain a level.
    /// The player cannot exceed the 'MAX_LEVEL' cap.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tap_core::{Player, PlayerId, PlayerClass, BASE_EXP_REQUIRED};
    ///
    /// // We create a player that is level 1
    /// let mut player = Player::new(PlayerId(1), PlayerClass::Warrior);
    /// assert_eq!(player.level, 1);
    ///
    /// // We give the player enough EXP to make it level up
    /// player.gain_experience(BASE_EXP_REQUIRED);
    ///
    /// // The player is indeed level 2
    /// assert_eq!(player.level, 2);
    /// assert_eq!(player.experience, 0);
    /// ```
    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount; // We could add different exp gain depending on the class
        if self.experience >= self.level * BASE_EXP_REQUIRED && self.level < MAX_LEVEL {
            self.level_up();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PlayerClass, PlayerId};

    /// A function to create a player
    fn create_warrior() -> Player {
        Player::new(PlayerId(1), PlayerClass::Warrior)
    }

    #[test]
    fn test_exp_gain_without_level_up() {
        let mut player = create_warrior();
        assert_eq!(player.level, 1);

        player.gain_experience(BASE_EXP_REQUIRED - 1);
        assert_eq!(player.level, 1);
        assert_eq!(player.experience, BASE_EXP_REQUIRED - 1);
    }

    #[test]
    fn test_exp_gain_and_level_up() {
        let mut player = create_warrior();
        assert_eq!(player.level, 1);

        player.gain_experience(BASE_EXP_REQUIRED);
        assert_eq!(player.level, 2);
    }

    #[test]
    fn test_exp_gain_above_needed() {
        let mut player = create_warrior();
        assert_eq!(player.level, 1);

        player.gain_experience(BASE_EXP_REQUIRED + 50);
        assert_eq!(player.level, 2);
        assert_eq!(player.experience, 0);
    }
}
