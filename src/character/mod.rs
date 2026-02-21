//! Character system
//!
//! This module handles:
//! - Character attributes (Agility, Strength, Finesse, etc.)
//! - Classes and ancestries
//! - Character progression and leveling
//! - Foundation abilities

pub mod ancestry;
pub mod attributes;
pub mod classes;
pub mod progression;

pub use ancestry::Ancestry;
pub use attributes::{AttributeType, Attributes};
pub use classes::{Class, Domain};
pub use progression::CharacterProgress;

// TODO: Add submodules
// pub mod sheet;
