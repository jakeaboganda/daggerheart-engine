//! Dice rolling system
//!
//! Core dice mechanics including:
//! - Basic dice (d4, d6, d8, d10, d12, d20)
//! - Duality dice (2d12 Hope/Fear system)
//! - Damage dice (multiple dice with bonuses)

pub mod basic;
pub mod damage;
pub mod duality;

pub use basic::Die;
pub use damage::{DamageDice, DamageRoll};
pub use duality::{ControllingDie, DualityResult, DualityRoll, SuccessType};
