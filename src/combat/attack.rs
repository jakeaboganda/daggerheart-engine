//! Combat system - Attack resolution

use crate::core::dice::{DualityResult, DualityRoll};
use serde::{Deserialize, Serialize};

/// An attack action with modifiers
#[derive(Debug, Clone)]
pub struct Attack {
    pub modifier: i8,
    pub with_advantage: bool,
}

impl Attack {
    /// Create a new attack with the given modifier
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Attack;
    ///
    /// let attack = Attack::new(2);  // +2 to attack
    /// let result = attack.roll();
    /// ```
    pub fn new(modifier: i8) -> Self {
        Self {
            modifier,
            with_advantage: false,
        }
    }

    /// Add advantage to this attack
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Attack;
    ///
    /// let attack = Attack::new(2).with_advantage();
    /// ```
    pub fn with_advantage(mut self) -> Self {
        self.with_advantage = true;
        self
    }

    /// Roll the attack
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Attack;
    ///
    /// let attack = Attack::new(2);
    /// let result = attack.roll();
    ///
    /// if result.success {
    ///     println!("Hit! Total: {}", result.total);
    /// }
    /// ```
    pub fn roll(&self) -> AttackResult {
        let duality_roll = DualityRoll::roll();
        let duality_result = if self.with_advantage {
            duality_roll.with_advantage()
        } else {
            duality_roll.with_modifier(self.modifier)
        };
        AttackResult::from_duality_result(duality_result)
    }
}

/// Result of an attack roll
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttackResult {
    pub hope: u16,
    pub fear: u16,
    pub modifier: i8,
    pub success: bool,
    pub critical: bool,
    pub total: u16,
}

impl AttackResult {
    /// Create an AttackResult from a DualityResult
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::core::dice::DualityRoll;
    /// use daggerheart_engine::combat::AttackResult;
    ///
    /// let roll = DualityRoll::from_values(10, 5);
    /// let duality = roll.with_modifier(2);
    /// let result = AttackResult::from_duality_result(duality);
    ///
    /// assert!(result.success);  // Hope > Fear
    /// ```
    pub fn from_duality_result(duality: DualityResult) -> Self {
        let hope = duality.roll.hope as u16;
        let fear = duality.roll.fear as u16;
        let success = duality.roll.hope > duality.roll.fear;
        let critical = duality.is_critical;

        Self {
            hope,
            fear,
            modifier: duality.modifier,
            success,
            critical,
            total: duality.total,
        }
    }

    /// Check if this attack beats the target's evasion
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::core::dice::DualityRoll;
    /// use daggerheart_engine::combat::AttackResult;
    ///
    /// let roll = DualityRoll::from_values(10, 5);
    /// let duality = roll.with_modifier(2);
    /// let result = AttackResult::from_duality_result(duality);
    ///
    /// assert!(result.beats_evasion(12));  // Total is Hope+Fear+mod
    /// ```
    pub fn beats_evasion(&self, evasion: u8) -> bool {
        self.success && self.total >= evasion as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_with_positive_modifier() {
        // Attack with +2 modifier
        let attack = Attack::new(2);

        // We can't test exact rolls (random), but we can test the API
        let result = attack.roll();

        // Result should have valid dice values
        assert!(result.hope >= 1 && result.hope <= 12);
        assert!(result.fear >= 1 && result.fear <= 12);
        assert_eq!(result.modifier, 2);
    }

    #[test]
    fn test_attack_success_when_hope_higher() {
        // Create attack result where Hope wins
        let roll = DualityRoll::from_values(10, 5);
        let duality = roll.with_modifier(2);
        let result = AttackResult::from_duality_result(duality);

        assert!(result.success, "Attack should succeed when Hope > Fear");
        assert_eq!(result.hope, 10);
        assert_eq!(result.fear, 5);
    }

    #[test]
    fn test_attack_failure_when_fear_higher() {
        // Create attack result where Fear wins
        let roll = DualityRoll::from_values(5, 10);
        let duality = roll.with_modifier(2);
        let result = AttackResult::from_duality_result(duality);

        assert!(!result.success, "Attack should fail when Fear > Hope");
    }

    #[test]
    fn test_critical_hit_on_doubles_success() {
        // Doubles with Hope winning = critical success
        let roll = DualityRoll::from_values(10, 10);
        let duality = roll.with_modifier(0);
        let result = AttackResult::from_duality_result(duality);

        // When doubles, Hope and Fear are equal, so neither "wins"
        // But doubles are still critical
        assert!(result.critical, "Doubles should be critical");
    }

    #[test]
    fn test_critical_on_doubles() {
        // Any doubles should be critical
        let roll = DualityRoll::from_values(7, 7);
        let duality = roll.with_modifier(0);
        let result = AttackResult::from_duality_result(duality);

        assert!(result.critical, "Doubles should be critical");
    }

    #[test]
    fn test_not_critical_when_not_doubles() {
        let roll = DualityRoll::from_values(12, 8);
        let duality = roll.with_modifier(0);
        let result = AttackResult::from_duality_result(duality);

        assert!(!result.critical, "Non-doubles should not be critical");
    }

    #[test]
    fn test_attack_with_advantage() {
        // Attack with advantage should work
        let attack = Attack::new(2).with_advantage();
        let result = attack.roll();

        // Should have valid rolls
        assert!(result.hope >= 1 && result.hope <= 12);
        assert!(result.fear >= 1 && result.fear <= 12);
    }

    #[test]
    fn test_attack_against_evasion() {
        let roll = DualityRoll::from_values(10, 5);
        let duality = roll.with_modifier(2);
        let result = AttackResult::from_duality_result(duality);

        // Total = hope + fear + modifier = 10 + 5 + 2 = 17
        assert_eq!(result.total, 17);

        // Should hit lower evasions
        assert!(result.beats_evasion(15));
        assert!(result.beats_evasion(17));

        // Should not hit higher evasion
        assert!(!result.beats_evasion(18));
    }

    #[test]
    fn test_negative_modifier() {
        let roll = DualityRoll::from_values(10, 8);
        let duality = roll.with_modifier(-3);
        let result = AttackResult::from_duality_result(duality);

        // Total = 10 + 8 - 3 = 15
        assert_eq!(result.total, 15);
    }

    #[test]
    fn test_attack_result_serialization() {
        let roll = DualityRoll::from_values(10, 5);
        let duality = roll.with_modifier(2);
        let result = AttackResult::from_duality_result(duality);

        let json = serde_json::to_string(&result).unwrap();
        let loaded: AttackResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result, loaded);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_attack_result_is_deterministic(
            hope in 1u8..=12,
            fear in 1u8..=12,
            modifier in -10i8..=10,
        ) {
            let roll = DualityRoll::from_values(hope, fear);
            let duality = roll.with_modifier(modifier);

            let result1 = AttackResult::from_duality_result(duality.clone());
            let result2 = AttackResult::from_duality_result(duality);

            prop_assert_eq!(result1, result2);
        }

        #[test]
        fn prop_success_matches_controlling_die(
            hope in 1u8..=12,
            fear in 1u8..=12,
            modifier in -10i8..=10,
        ) {
            let roll = DualityRoll::from_values(hope, fear);
            let duality = roll.with_modifier(modifier);
            let result = AttackResult::from_duality_result(duality);

            let expected_success = hope > fear;

            prop_assert_eq!(result.success, expected_success);
        }

        #[test]
        fn prop_critical_on_doubles(
            value in 1u8..=12,
            modifier in -10i8..=10,
        ) {
            // Test with doubles
            let roll = DualityRoll::from_values(value, value);
            let duality = roll.with_modifier(modifier);
            let result = AttackResult::from_duality_result(duality);

            prop_assert!(result.critical, "Doubles should always be critical");
        }

        #[test]
        fn prop_not_critical_when_not_doubles(
            hope in 1u8..=12,
            fear in 1u8..=12,
            modifier in -10i8..=10,
        ) {
            prop_assume!(hope != fear); // Only test non-doubles

            let roll = DualityRoll::from_values(hope, fear);
            let duality = roll.with_modifier(modifier);
            let result = AttackResult::from_duality_result(duality);

            prop_assert!(!result.critical, "Non-doubles should not be critical");
        }

        #[test]
        fn prop_total_is_sum_plus_modifier(
            hope in 1u8..=12,
            fear in 1u8..=12,
            modifier in -10i8..=10,
        ) {
            let roll = DualityRoll::from_values(hope, fear);
            let duality = roll.with_modifier(modifier);
            let result = AttackResult::from_duality_result(duality);

            let expected_total = (hope as i16 + fear as i16 + modifier as i16) as u16;

            prop_assert_eq!(result.total, expected_total);
        }
    }
}
