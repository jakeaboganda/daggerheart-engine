//! Error types for the Daggerheart engine

use thiserror::Error;

/// Main error type for the Daggerheart engine
#[derive(Error, Debug)]
pub enum EngineError {
    /// Invalid dice roll
    #[error("Invalid dice roll: {0}")]
    InvalidDiceRoll(String),

    /// Invalid character state
    #[error("Invalid character state: {0}")]
    InvalidCharacterState(String),

    /// Invalid combat action
    #[error("Invalid combat action: {0}")]
    InvalidCombatAction(String),

    /// Resource exceeded (e.g., action economy)
    #[error("Resource limit exceeded: {0}")]
    ResourceExceeded(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, EngineError>;
