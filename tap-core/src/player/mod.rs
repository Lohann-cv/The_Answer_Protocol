//! Player management module.
//!
//! This module groups everything related to players, including their stats,
//! classes, level progression rules, and combat skills.

mod class;
mod progression;
mod skill;

pub use class::{Player, PlayerClass, PlayerId, Stats};

/// The maximum level a player can achieve.
pub const MAX_LEVEL: u32 = 100;

/// The base experience required to level up from level 1.
pub const BASE_EXP_REQUIRED: u32 = 100;

/// The multiplier applied to stamina to calculate maximum health.
pub const HEALTH_MULTIPLIER: u32 = 10;
