mod class;
mod progression;
mod skill;

pub use class::{Player, PlayerClass, PlayerId, Stats}; // might want to expose Stats

pub const MAX_LEVEL: u32 = 100;
pub const BASE_EXP_REQUIRED: u32 = 100;
pub const HEALTH_MULTIPLIER: u32 = 10;
