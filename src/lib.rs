//! # Daggerheart Rules Engine
//!
//! A Rust implementation of the Daggerheart TTRPG rules system.
//!
//! This library provides the core game mechanics including:
//! - Dice rolling system (duality dice, damage dice)
//! - Character creation and progression
//! - Combat resolution
//! - Domain card mechanics
//! - Item and equipment management

// Public modules
pub mod core;
pub mod character;
pub mod combat;
pub mod cards;
pub mod items;
pub mod error;

// Re-export commonly used types
pub use error::EngineError;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }
}
