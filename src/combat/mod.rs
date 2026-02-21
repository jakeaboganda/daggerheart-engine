//! Combat system
//!
//! This module handles:
//! - Attack rolls and resolution
//! - Damage calculation and application
//! - Combat resources (HP, Stress, Hope, Fear)
//! - Action economy

pub mod attack;
pub mod damage;
pub mod resources;

pub use attack::{Attack, AttackResult};
pub use damage::DamageResult;
pub use resources::{Fear, HitPoints, Hope, Stress};

// TODO: Add submodules
// pub mod actions;
