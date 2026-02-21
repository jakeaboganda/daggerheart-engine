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
pub mod cards;
pub mod character;
pub mod combat;
pub mod core;
pub mod error;
pub mod items;

// Re-export commonly used types
pub use error::EngineError;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        // VERSION is always non-empty (from CARGO_PKG_VERSION)
        assert_eq!(
            VERSION.split('.').count(),
            3,
            "Version should be semver format"
        );
    }
}
