use super::HEALTH_MULTIPLIER;
use crate::world::map::WorldId;
use serde::{Deserialize, Serialize};

/// The struct that represent the player's id.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, ParialEq, Eq)]
pub struct PlayerId(pub u32);

/// The different classes that the users can pick.
/// 
/// Each classes will influence the gameplay.
/// For example, a warrior will focus on damage.
/// And a healer will do little to no damage but is able to heal.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerClass {
    /// The warrior focus on strengh damage.
    Warrior,
    /// The mage is for intelligence damage at distence.
    Mage,
    /// The archer deal agility damage at distence.
    Archer,
    /// The summoner deal intelligence damage and is able to attack with summons.
    Summoner,
    /// The healer is focused on healing allies.
    Healer,
    /// The tank is able to take enemies focus and deal stamina base damage.
    Tank,
    /// The thief deal agility based damage and is able to steal.
    Thief,
}

/// The player's statistic.
/// 
/// Each player will have stats based on its class.
/// The stats will evolve throughout the game.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, ParialEq, Eq)]
pub struct Stats {
    /// The strength that will be used by the warrior.
    pub strength: u32,
    /// The agility for the archer and the thief.
    pub agility: u32,
    /// The intelligence for the mage, summoner and healer.
    pub intelligence: u32,
    /// The stamina for the tank and to scale HP.
    pub stamina: u32,
    /// The luck for items.
    pub luck: u32,
}

impl From<PlayerClass> for Stats {
    /// The function that will set the player's base stats.
    /// 
    /// It takes the player chosen class as its parameter.
    /// The stats are implemented for the player.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // With from()
    /// let stats = Stats::from(PlayerClass::Warrior);
    /// assert_eq!(stats.strength, 10);
    ///
    /// // With into()
    /// let stats_into: Stats = PlayerClass::Archer.into();
    /// assert_eq!(stats_into.agility, 10);
    /// ```
    fn from(class: PlayerClass) -> Self {
        match class {
            PlayerClass::Warrior => Stats {
                strength: 10,
                agility: 5,
                intelligence: 3,
                stamina: 8,
                luck: 2,
            },
            PlayerClass::Mage => Stats {
                strength: 3,
                agility: 4,
                intelligence: 10,
                stamina: 5,
                luck: 6,
            },
            PlayerClass::Archer => Stats {
                strength: 5,
                agility: 10,
                intelligence: 4,
                stamina: 7,
                luck: 8,
            },
            PlayerClass::Summoner => Stats {
                strength: 4,
                agility: 6,
                intelligence: 9,
                stamina: 6,
                luck: 7,
            },
            PlayerClass::Healer => Stats {
                strength: 6,
                agility: 7,
                intelligence: 8,
                stamina: 9,
                luck: 5,
            },
            PlayerClass::Tank => Stats {
                strength: 9,
                agility: 3,
                intelligence: 4,
                stamina: 10,
                luck: 4,
            },
            PlayerClass::Thief => Stats {
                strength: 7,
                agility: 9,
                intelligence: 6,
                stamina: 8,
                luck: 10,
            },
        }
    }
}

/// The player build.
///
/// It holds the player's information.
/// With its stats, position and unique id.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Player {
    /// The player id is encapsulated so it can be identified without copying everything.
    pub id: PlayerId,
    /// The player level that increases throughout the game.
    pub level: u32,
    /// Its experience that is reset when the player levels up.
    pub experience: u32,
    /// The player's current health.
    pub health: u32,
    /// The maximum health, the health cannot exceed this calue.
    pub max_health: u32,
    /// The world where the player is.
    pub position: WorldId,
    /// The player chosen class.
    pub class: PlayerClass,
    /// The player stats.
    pub stats: Stats,
}

impl Player {
    /// Creates a new player from an ID and a chosen class.
    /// 
    /// The player starts at level 1 with full health based on their class stamina.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let player = Player::new(PlayerId(1), PlayerClass::Tank);
    /// assert_eq!(player.level, 1);
    /// assert!(player.is_alive());
    /// ```
    pub fn new(id: PlayerId, class: PlayerClass) -> Self {
        let stats: Stats = class.into();
        let max_health = stats.stamina * HEALTH_MULTIPLIER;
        Player {
            id,
            level: 1,
            experience: 0,
            health: max_health,
            max_health,
            position: WorldId(0),
            class,
            stats,
        }
    }

    /// Checks if the player is currently alive.
    ///
    /// Returns 'true' if the player's health is strictly greater than 0.
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Resets the player's progress to the starting state.
    ///
    /// This resets the level, experience, health, and position.
    /// but keeps the player's ID, class, and base stats.
    pub fn reset(&mut self) {
        self.level = 1;
        self.experience = 0;
        self.health = self.max_health;
        self.position = WorldId(0);
    }
}
