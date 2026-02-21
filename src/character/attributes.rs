//! Character attribute system
//!
//! Characters have six core traits with modifiers that must sum correctly.

use crate::error::EngineError;
use serde::{Deserialize, Serialize};

/// Attribute types in Daggerheart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeType {
    Agility,
    Strength,
    Finesse,
    Instinct,
    Presence,
    Knowledge,
}

/// Character attributes (traits)
///
/// Daggerheart characters have six core traits that modify their actions.
/// The modifiers must be exactly: +2, +1, +1, +0, +0, -1 (in any order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attributes {
    pub agility: i8,
    pub strength: i8,
    pub finesse: i8,
    pub instinct: i8,
    pub presence: i8,
    pub knowledge: i8,
}

impl Attributes {
    /// Standard modifier distribution: +2, +1, +1, +0, +0, -1
    pub const STANDARD_MODIFIERS: [i8; 6] = [2, 1, 1, 0, 0, -1];

    /// Create attributes from an array of six modifiers
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::Attributes;
    ///
    /// let attrs = Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap();
    /// assert!(attrs.validate().is_ok());
    /// ```
    pub fn from_array(mods: [i8; 6]) -> Result<Self, EngineError> {
        let attrs = Self {
            agility: mods[0],
            strength: mods[1],
            finesse: mods[2],
            instinct: mods[3],
            presence: mods[4],
            knowledge: mods[5],
        };

        // Validate before returning
        attrs.validate()?;
        Ok(attrs)
    }

    /// Get the modifier for a specific attribute type
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::{Attributes, AttributeType};
    ///
    /// let attrs = Attributes {
    ///     agility: 2,
    ///     strength: 1,
    ///     finesse: 0,
    ///     instinct: 1,
    ///     presence: 0,
    ///     knowledge: -1,
    /// };
    ///
    /// assert_eq!(attrs.get_modifier(AttributeType::Agility), 2);
    /// assert_eq!(attrs.get_modifier(AttributeType::Knowledge), -1);
    /// ```
    pub fn get_modifier(&self, attr_type: AttributeType) -> i8 {
        match attr_type {
            AttributeType::Agility => self.agility,
            AttributeType::Strength => self.strength,
            AttributeType::Finesse => self.finesse,
            AttributeType::Instinct => self.instinct,
            AttributeType::Presence => self.presence,
            AttributeType::Knowledge => self.knowledge,
        }
    }

    /// Validate that attributes use the correct modifier distribution
    ///
    /// Checks that the modifiers are exactly: +2, +1, +1, +0, +0, -1 (in any order)
    pub fn validate(&self) -> Result<(), EngineError> {
        let mut mods = vec![
            self.agility,
            self.strength,
            self.finesse,
            self.instinct,
            self.presence,
            self.knowledge,
        ];
        mods.sort();

        let mut expected = Self::STANDARD_MODIFIERS.to_vec();
        expected.sort();

        if mods == expected {
            Ok(())
        } else {
            Err(EngineError::InvalidCharacterState(format!(
                "Attributes must use standard modifiers {:?}, got {:?}",
                expected, mods
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_attributes() {
        // Standard distribution: +2, +1, +1, +0, +0, -1
        let attrs = Attributes {
            agility: 2,
            strength: 1,
            finesse: 0,
            instinct: 1,
            presence: 0,
            knowledge: -1,
        };

        // Should be valid
        assert!(attrs.validate().is_ok());
    }

    #[test]
    fn test_all_valid_permutations() {
        // Different valid arrangements
        let valid_sets = vec![
            Attributes {
                agility: 2,
                strength: 1,
                finesse: 1,
                instinct: 0,
                presence: 0,
                knowledge: -1,
            },
            Attributes {
                agility: -1,
                strength: 0,
                finesse: 0,
                instinct: 1,
                presence: 1,
                knowledge: 2,
            },
            Attributes {
                agility: 0,
                strength: 2,
                finesse: -1,
                instinct: 1,
                presence: 1,
                knowledge: 0,
            },
        ];

        for attrs in valid_sets {
            assert!(attrs.validate().is_ok(), "Failed for: {:?}", attrs);
        }
    }

    #[test]
    fn test_invalid_too_high() {
        // Invalid: all modifiers too high
        let attrs = Attributes {
            agility: 3,
            strength: 2,
            finesse: 1,
            instinct: 0,
            presence: 0,
            knowledge: 0,
        };

        assert!(attrs.validate().is_err());
    }

    #[test]
    fn test_invalid_wrong_distribution() {
        // Invalid: wrong distribution (missing -1)
        let attrs = Attributes {
            agility: 2,
            strength: 1,
            finesse: 1,
            instinct: 0,
            presence: 0,
            knowledge: 0, // Should be -1
        };

        assert!(attrs.validate().is_err());
    }

    #[test]
    fn test_invalid_duplicate_high() {
        // Invalid: two +2 modifiers
        let attrs = Attributes {
            agility: 2,
            strength: 2,
            finesse: 0,
            instinct: 0,
            presence: -1,
            knowledge: 0,
        };

        assert!(attrs.validate().is_err());
    }

    #[test]
    fn test_get_modifier() {
        let attrs = Attributes {
            agility: 2,
            strength: 1,
            finesse: 0,
            instinct: 1,
            presence: 0,
            knowledge: -1,
        };

        assert_eq!(attrs.get_modifier(AttributeType::Agility), 2);
        assert_eq!(attrs.get_modifier(AttributeType::Strength), 1);
        assert_eq!(attrs.get_modifier(AttributeType::Finesse), 0);
        assert_eq!(attrs.get_modifier(AttributeType::Instinct), 1);
        assert_eq!(attrs.get_modifier(AttributeType::Presence), 0);
        assert_eq!(attrs.get_modifier(AttributeType::Knowledge), -1);
    }

    #[test]
    fn test_from_array() {
        // Create from array of modifiers
        let mods = [2, 1, 1, 0, 0, -1];
        let result = Attributes::from_array(mods);

        assert!(result.is_ok());
        let attrs = result.unwrap();
        assert!(attrs.validate().is_ok());
    }

    #[test]
    fn test_from_array_invalid() {
        // Wrong modifiers
        let mods = [3, 2, 1, 0, 0, 0];
        let result = Attributes::from_array(mods);

        assert!(result.is_err());
    }

    #[test]
    fn test_standard_modifiers_constant() {
        let expected = vec![-1, 0, 0, 1, 1, 2];
        let mut actual = Attributes::STANDARD_MODIFIERS.to_vec();
        actual.sort();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_validation_is_consistent(
            ag in -5i8..=5,
            st in -5i8..=5,
            fi in -5i8..=5,
            ins in -5i8..=5,
            pr in -5i8..=5,
            kn in -5i8..=5,
        ) {
            let attrs = Attributes {
                agility: ag,
                strength: st,
                finesse: fi,
                instinct: ins,
                presence: pr,
                knowledge: kn,
            };

            let result1 = attrs.validate();
            let result2 = attrs.validate();

            // Validation should be deterministic
            prop_assert_eq!(result1.is_ok(), result2.is_ok());
        }

        #[test]
        fn prop_valid_attributes_always_validate(
            positions in proptest::sample::subsequence((0..6).collect::<Vec<_>>(), 6)
        ) {
            let standard = Attributes::STANDARD_MODIFIERS;
            let mut mods = [0i8; 6];
            for (i, &pos) in positions.iter().enumerate() {
                mods[pos] = standard[i];
            }

            let attrs = Attributes {
                agility: mods[0],
                strength: mods[1],
                finesse: mods[2],
                instinct: mods[3],
                presence: mods[4],
                knowledge: mods[5],
            };

            prop_assert!(attrs.validate().is_ok(), "Permutation failed: {:?}", mods);
        }
    }
}
