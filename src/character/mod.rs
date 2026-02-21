//! Character system
//!
//! This module handles:
//! - Character attributes (Agility, Strength, Finesse, etc.)
//! - Classes and ancestries
//! - Character progression and leveling
//! - Foundation abilities

pub mod attributes;
pub mod classes;

pub use attributes::{AttributeType, Attributes};
pub use classes::{Class, Domain};

// TODO: Add submodules
// pub mod ancestry;
// pub mod progression;
// pub mod sheet;
