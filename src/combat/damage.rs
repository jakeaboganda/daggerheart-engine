//! Damage calculation and resolution

use serde::{Deserialize, Serialize};

/// Result of applying damage to a character
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DamageResult {
    pub raw_damage: u16,
    pub after_armor: u16,
    pub hp_lost: u8,
    pub stress_gained: u8,
}

impl DamageResult {
    /// Calculate damage result from raw damage and armor
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::DamageResult;
    ///
    /// // 10 damage against 3 armor = 7 after armor
    /// let result = DamageResult::calculate(10, 3);
    /// assert_eq!(result.raw_damage, 10);
    /// assert_eq!(result.after_armor, 7);
    /// ```
    pub fn calculate(raw_damage: u16, armor_score: u8) -> Self {
        let after_armor = raw_damage.saturating_sub(armor_score as u16);

        // Damage threshold mechanics:
        // - Below threshold (< 5): 0 HP, 1 Stress
        // - At/Above threshold (>= 5): HP damage based on amount
        let threshold = 5;

        let (hp_lost, stress_gained) = if after_armor < threshold {
            // Below threshold = scratch (1 Stress)
            (0, 1)
        } else {
            // At/above threshold = real damage
            let excess = after_armor - threshold;
            let hp = match excess {
                0..=4 => 1, // 5-9 damage = 1 HP
                5..=9 => 2, // 10-14 damage = 2 HP
                _ => 3,     // 15+ damage = 3 HP
            };
            (hp, 0)
        };

        Self {
            raw_damage,
            after_armor,
            hp_lost,
            stress_gained,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_below_threshold_gives_stress() {
        // 7 raw damage - 5 armor = 2 (below threshold of 5)
        let result = DamageResult::calculate(7, 5);

        assert_eq!(result.raw_damage, 7);
        assert_eq!(result.after_armor, 2);
        assert_eq!(result.hp_lost, 0);
        assert_eq!(result.stress_gained, 1);
    }

    #[test]
    fn test_damage_at_threshold() {
        // Exactly at threshold = 1 HP
        let result = DamageResult::calculate(10, 5);

        assert_eq!(result.after_armor, 5);
        assert_eq!(result.hp_lost, 1);
        assert_eq!(result.stress_gained, 0);
    }

    #[test]
    fn test_moderate_damage() {
        // 12 damage - 2 armor = 10 (threshold + 5 = 2 HP)
        let result = DamageResult::calculate(12, 2);

        assert_eq!(result.after_armor, 10);
        assert_eq!(result.hp_lost, 2);
        assert_eq!(result.stress_gained, 0);
    }

    #[test]
    fn test_heavy_damage() {
        // 20 damage - 2 armor = 18 (threshold + 13 = 3 HP)
        let result = DamageResult::calculate(20, 2);

        assert_eq!(result.after_armor, 18);
        assert_eq!(result.hp_lost, 3);
        assert_eq!(result.stress_gained, 0);
    }

    #[test]
    fn test_armor_reduces_to_zero() {
        // All damage blocked
        let result = DamageResult::calculate(5, 10);

        assert_eq!(result.after_armor, 0);
        assert_eq!(result.hp_lost, 0);
        assert_eq!(result.stress_gained, 1); // Still 1 stress even if blocked
    }

    #[test]
    fn test_no_damage() {
        let result = DamageResult::calculate(0, 0);

        assert_eq!(result.raw_damage, 0);
        assert_eq!(result.after_armor, 0);
        assert_eq!(result.hp_lost, 0);
        assert_eq!(result.stress_gained, 1); // Even 0 damage = 1 stress
    }

    #[test]
    fn test_damage_serialization() {
        let result = DamageResult::calculate(10, 3);

        let json = serde_json::to_string(&result).unwrap();
        let loaded: DamageResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result, loaded);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_damage_is_deterministic(
            raw in 0u16..100,
            armor in 0u8..20,
        ) {
            let result1 = DamageResult::calculate(raw, armor);
            let result2 = DamageResult::calculate(raw, armor);

            prop_assert_eq!(result1, result2);
        }

        #[test]
        fn prop_armor_never_increases_damage(
            raw in 1u16..100,
            armor in 0u8..20,
        ) {
            let result = DamageResult::calculate(raw, armor);

            prop_assert!(result.after_armor <= raw);
        }

        #[test]
        fn prop_below_threshold_always_gives_stress(
            raw in 1u16..100,
            armor in 0u8..20,
        ) {
            let result = DamageResult::calculate(raw, armor);

            if result.after_armor < 5 {
                prop_assert_eq!(result.hp_lost, 0);
                prop_assert_eq!(result.stress_gained, 1);
            }
        }

        #[test]
        fn prop_at_or_above_threshold_gives_hp_damage(
            raw in 5u16..100,
            armor in 0u8..4,  // Ensure after_armor >= 5
        ) {
            let result = DamageResult::calculate(raw, armor);

            if result.after_armor >= 5 {
                prop_assert!(result.hp_lost > 0);
                prop_assert_eq!(result.stress_gained, 0);
            }
        }

        #[test]
        fn prop_hp_damage_increases_with_excess(
            excess in 0u16..50,
        ) {
            // Damage = threshold + excess
            let raw = 5 + excess + 3; // Add armor amount
            let result = DamageResult::calculate(raw, 3);

            let expected_hp = match excess {
                0..=4 => 1,
                5..=9 => 2,
                _ => 3,
            };

            prop_assert_eq!(result.hp_lost, expected_hp);
        }
    }
}
