//! Core game mechanics
//!
//! This module contains the fundamental systems:
//! - Dice rolling (duality dice, damage dice)
//! - Roll resolution
//! - Hope and Fear mechanics
//! - Action tokens and resources

pub mod dice;
// pub mod roll;
// pub mod resources;

pub use dice::{Die, DualityRoll, DualityResult, ControllingDie, SuccessType};
// pub use dice::{DamageDice, DamageRoll};
