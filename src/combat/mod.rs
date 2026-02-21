//! Combat system
//!
//! This module handles:
//! - Attack rolls and resolution
//! - Damage calculation and application
//! - Combat resources (HP, Stress, Hope, Fear)
//! - Action economy

pub mod attack;

pub use attack::{Attack, AttackResult};

// TODO: Add submodules
// pub mod damage;
// pub mod resources;
// pub mod actions;
