//! Character progression - leveling and advancement

use crate::error::EngineError;
use serde::{Deserialize, Serialize};

/// Character progression tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProgress {
    pub level: u8,
    pub experience: u32,
    pub available_cards: Vec<String>,
}

impl CharacterProgress {
    /// Create a new character at level 1
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::progression::CharacterProgress;
    ///
    /// let progress = CharacterProgress::new();
    /// assert_eq!(progress.level, 1);
    /// assert_eq!(progress.experience, 0);
    /// ```
    pub fn new() -> Self {
        Self {
            level: 1,
            experience: 0,
            available_cards: Vec::new(),
        }
    }

    /// Get XP required for next level
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::progression::CharacterProgress;
    ///
    /// let progress = CharacterProgress::new();
    /// assert_eq!(progress.xp_for_next_level(), 100);
    /// ```
    pub fn xp_for_next_level(&self) -> u32 {
        // Simple formula: level * 100
        (self.level as u32) * 100
    }

    /// Add experience points
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::progression::CharacterProgress;
    ///
    /// let mut progress = CharacterProgress::new();
    /// progress.add_experience(50);
    /// assert_eq!(progress.experience, 50);
    /// ```
    pub fn add_experience(&mut self, amount: u32) {
        self.experience += amount;
    }

    /// Check if character can level up
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::progression::CharacterProgress;
    ///
    /// let mut progress = CharacterProgress::new();
    /// assert!(!progress.can_level_up());
    ///
    /// progress.add_experience(100);
    /// assert!(progress.can_level_up());
    /// ```
    pub fn can_level_up(&self) -> bool {
        self.level < 10 && self.experience >= self.xp_for_next_level()
    }

    /// Level up (consumes XP)
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::progression::CharacterProgress;
    ///
    /// let mut progress = CharacterProgress::new();
    /// progress.add_experience(150);
    ///
    /// assert!(progress.level_up().is_ok());
    /// assert_eq!(progress.level, 2);
    /// assert_eq!(progress.experience, 50); // 150 - 100 = 50 left over
    /// ```
    pub fn level_up(&mut self) -> Result<(), EngineError> {
        if !self.can_level_up() {
            return Err(EngineError::InvalidCharacterState(
                "Not enough XP to level up".to_string(),
            ));
        }

        let cost = self.xp_for_next_level();
        self.experience -= cost;
        self.level += 1;

        Ok(())
    }

    /// Add a card to available cards
    pub fn add_card(&mut self, card_id: impl Into<String>) {
        self.available_cards.push(card_id.into());
    }

    /// Check if character has a specific card
    pub fn has_card(&self, card_id: &str) -> bool {
        self.available_cards.iter().any(|c| c == card_id)
    }
}

impl Default for CharacterProgress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_progression() {
        let progress = CharacterProgress::new();

        assert_eq!(progress.level, 1);
        assert_eq!(progress.experience, 0);
        assert_eq!(progress.available_cards.len(), 0);
    }

    #[test]
    fn test_xp_for_next_level() {
        let mut progress = CharacterProgress::new();

        assert_eq!(progress.xp_for_next_level(), 100); // Level 1 -> 2

        progress.level = 2;
        assert_eq!(progress.xp_for_next_level(), 200); // Level 2 -> 3

        progress.level = 5;
        assert_eq!(progress.xp_for_next_level(), 500); // Level 5 -> 6
    }

    #[test]
    fn test_add_experience() {
        let mut progress = CharacterProgress::new();

        progress.add_experience(50);
        assert_eq!(progress.experience, 50);

        progress.add_experience(30);
        assert_eq!(progress.experience, 80);
    }

    #[test]
    fn test_can_level_up() {
        let mut progress = CharacterProgress::new();

        assert!(!progress.can_level_up());

        progress.add_experience(50);
        assert!(!progress.can_level_up());

        progress.add_experience(50);
        assert!(progress.can_level_up()); // Exactly 100
    }

    #[test]
    fn test_level_up_success() {
        let mut progress = CharacterProgress::new();
        progress.add_experience(150);

        assert!(progress.level_up().is_ok());
        assert_eq!(progress.level, 2);
        assert_eq!(progress.experience, 50); // 150 - 100
    }

    #[test]
    fn test_level_up_failure_not_enough_xp() {
        let mut progress = CharacterProgress::new();
        progress.add_experience(50);

        assert!(progress.level_up().is_err());
        assert_eq!(progress.level, 1); // Unchanged
    }

    #[test]
    fn test_level_up_max_level() {
        let mut progress = CharacterProgress::new();
        progress.level = 10;
        progress.add_experience(1000);

        assert!(!progress.can_level_up());
        assert!(progress.level_up().is_err());
    }

    #[test]
    fn test_multiple_level_ups() {
        let mut progress = CharacterProgress::new();
        progress.add_experience(600); // Enough for 3 levels (100 + 200 + 300)

        assert!(progress.level_up().is_ok());
        assert_eq!(progress.level, 2);
        assert_eq!(progress.experience, 500);

        assert!(progress.level_up().is_ok());
        assert_eq!(progress.level, 3);
        assert_eq!(progress.experience, 300);

        assert!(progress.level_up().is_ok());
        assert_eq!(progress.level, 4);
        assert_eq!(progress.experience, 0);
    }

    #[test]
    fn test_add_card() {
        let mut progress = CharacterProgress::new();

        progress.add_card("blade_strike");
        assert_eq!(progress.available_cards.len(), 1);
        assert!(progress.has_card("blade_strike"));
    }

    #[test]
    fn test_has_card() {
        let mut progress = CharacterProgress::new();

        assert!(!progress.has_card("blade_strike"));

        progress.add_card("blade_strike");
        assert!(progress.has_card("blade_strike"));
        assert!(!progress.has_card("fireball"));
    }

    #[test]
    fn test_progression_serialization() {
        let mut progress = CharacterProgress::new();
        progress.add_experience(50);
        progress.add_card("test_card");

        let json = serde_json::to_string(&progress).unwrap();
        let loaded: CharacterProgress = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.level, progress.level);
        assert_eq!(loaded.experience, progress.experience);
        assert_eq!(loaded.available_cards, progress.available_cards);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_xp_increases_with_level(level in 1u8..10) {
            let mut progress = CharacterProgress::new();
            progress.level = level;

            let xp_needed = progress.xp_for_next_level();

            prop_assert_eq!(xp_needed, level as u32 * 100);
        }

        #[test]
        fn prop_adding_xp_never_decreases(
            initial_xp in 0u32..1000,
            add_xp in 0u32..500,
        ) {
            let mut progress = CharacterProgress::new();
            progress.experience = initial_xp;

            progress.add_experience(add_xp);

            prop_assert!(progress.experience >= initial_xp);
            prop_assert_eq!(progress.experience, initial_xp + add_xp);
        }

        #[test]
        fn prop_level_up_requires_sufficient_xp(
            level in 1u8..9,
            xp in 0u32..2000,
        ) {
            let mut progress = CharacterProgress::new();
            progress.level = level;
            progress.experience = xp;

            let can_level = progress.can_level_up();
            let xp_needed = level as u32 * 100;

            prop_assert_eq!(can_level, xp >= xp_needed);
        }

        #[test]
        fn prop_cards_persist_after_adding(
            card_count in 0usize..10,
        ) {
            let mut progress = CharacterProgress::new();

            for i in 0..card_count {
                progress.add_card(format!("card_{}", i));
            }

            prop_assert_eq!(progress.available_cards.len(), card_count);
        }
    }
}
