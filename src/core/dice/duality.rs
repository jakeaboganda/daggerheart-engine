//! Duality dice system (2d12 Hope/Fear)
//!
//! The core mechanic of Daggerheart: rolling two d12s simultaneously,
//! one representing Hope and one representing Fear.

use rand::Rng;
use std::cmp::Ordering;

/// A roll of the duality dice (2d12: Hope and Fear)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DualityRoll {
    pub hope: u8,
    pub fear: u8,
}

/// Which die controls the outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllingDie {
    Hope,
    Fear,
    Tied,
}

/// The result of a duality roll with modifiers
#[derive(Debug, Clone, PartialEq)]
pub struct DualityResult {
    pub roll: DualityRoll,
    pub modifier: i8,
    pub advantage_die: Option<u8>, // d6 if advantage
    pub total: u16,
    pub controlling: ControllingDie,
    pub is_critical: bool,
}

/// Type of success based on the roll
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuccessType {
    Failure,
    SuccessWithHope,
    SuccessWithFear,
    CriticalSuccess,
}

impl DualityRoll {
    /// Roll both hope and fear dice
    pub fn roll() -> Self {
        let mut rng = rand::thread_rng();
        let hope = rng.gen_range(1..=12);
        let fear = rng.gen_range(1..=12);
        Self { hope, fear }
    }

    /// Create a duality roll from specific values (for testing)
    pub fn from_values(hope: u8, fear: u8) -> Self {
        Self { hope, fear }
    }

    /// Check if this roll is a critical (doubles)
    pub fn is_critical(&self) -> bool {
        self.hope == self.fear
    }

    /// Determine which die is controlling
    pub fn controlling_die(&self) -> ControllingDie {
        match self.hope.cmp(&self.fear) {
            Ordering::Greater => ControllingDie::Hope,
            Ordering::Less => ControllingDie::Fear,
            Ordering::Equal => ControllingDie::Tied,
        }
    }

    /// Apply a modifier to create a DualityResult
    pub fn with_modifier(self, modifier: i8) -> DualityResult {
        let total = (self.hope as i16 + self.fear as i16 + modifier as i16) as u16;

        DualityResult {
            roll: self,
            modifier,
            advantage_die: None,
            total,
            controlling: self.controlling_die(),
            is_critical: self.is_critical(),
        }
    }

    /// Apply advantage (roll extra d6) to create a DualityResult
    pub fn with_advantage(self) -> DualityResult {
        let mut rng = rand::thread_rng();
        let d6 = rng.gen_range(1..=6);

        let total = self.hope as u16 + self.fear as u16 + d6 as u16;

        DualityResult {
            roll: self,
            modifier: 0,
            advantage_die: Some(d6),
            total,
            controlling: self.controlling_die(),
            is_critical: self.is_critical(),
        }
    }
}

impl DualityResult {
    /// Check if this roll meets or exceeds the difficulty
    pub fn is_success(&self, difficulty: u16) -> bool {
        self.total >= difficulty
    }

    /// Determine the type of success
    pub fn success_type(&self, difficulty: u16) -> SuccessType {
        if !self.is_success(difficulty) {
            return SuccessType::Failure;
        }

        if self.is_critical {
            return SuccessType::CriticalSuccess;
        }

        match self.controlling {
            ControllingDie::Hope => SuccessType::SuccessWithHope,
            ControllingDie::Fear | ControllingDie::Tied => SuccessType::SuccessWithFear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_values() {
        let roll = DualityRoll::from_values(7, 10);
        assert_eq!(roll.hope, 7);
        assert_eq!(roll.fear, 10);
    }

    #[test]
    fn test_critical_success_doubles() {
        // Any doubles are critical
        assert!(DualityRoll::from_values(1, 1).is_critical());
        assert!(DualityRoll::from_values(7, 7).is_critical());
        assert!(DualityRoll::from_values(12, 12).is_critical());
    }

    #[test]
    fn test_not_critical_when_different() {
        assert!(!DualityRoll::from_values(5, 7).is_critical());
        assert!(!DualityRoll::from_values(12, 11).is_critical());
    }

    #[test]
    fn test_hope_wins() {
        let roll = DualityRoll::from_values(10, 5);
        assert_eq!(roll.controlling_die(), ControllingDie::Hope);
    }

    #[test]
    fn test_fear_wins() {
        let roll = DualityRoll::from_values(3, 9);
        assert_eq!(roll.controlling_die(), ControllingDie::Fear);
    }

    #[test]
    fn test_tied() {
        let roll = DualityRoll::from_values(8, 8);
        assert_eq!(roll.controlling_die(), ControllingDie::Tied);
    }

    #[test]
    fn test_with_modifier_positive() {
        let roll = DualityRoll::from_values(6, 4);
        let result = roll.with_modifier(2);

        assert_eq!(result.modifier, 2);
        assert_eq!(result.total, 6 + 4 + 2);
        assert_eq!(result.controlling, ControllingDie::Hope);
        assert!(!result.is_critical);
    }

    #[test]
    fn test_with_modifier_negative() {
        let roll = DualityRoll::from_values(8, 5);
        let result = roll.with_modifier(-1);

        assert_eq!(result.modifier, -1);
        assert_eq!(result.total, 8 + 5 - 1);
    }

    #[test]
    fn test_with_advantage() {
        let roll = DualityRoll::from_values(5, 7);
        let result = roll.with_advantage();

        assert!(result.advantage_die.is_some());
        let d6 = result.advantage_die.unwrap();
        assert!((1..=6).contains(&d6));
        assert_eq!(result.total, 5 + 7 + d6 as u16);
    }

    #[test]
    fn test_critical_preserved_in_result() {
        let roll = DualityRoll::from_values(9, 9);
        let result = roll.with_modifier(3);

        assert!(result.is_critical);
    }

    #[test]
    fn test_is_success() {
        let roll = DualityRoll::from_values(8, 6); // Total 14
        let result = roll.with_modifier(0);

        assert!(result.is_success(12)); // 14 >= 12
        assert!(result.is_success(14)); // 14 >= 14
        assert!(!result.is_success(15)); // 14 < 15
    }

    #[test]
    fn test_success_with_hope() {
        let roll = DualityRoll::from_values(9, 5); // Hope wins, total 14
        let result = roll.with_modifier(0);

        assert_eq!(result.success_type(12), SuccessType::SuccessWithHope);
    }

    #[test]
    fn test_success_with_fear() {
        let roll = DualityRoll::from_values(4, 10); // Fear wins, total 14
        let result = roll.with_modifier(0);

        assert_eq!(result.success_type(12), SuccessType::SuccessWithFear);
    }

    #[test]
    fn test_critical_success() {
        let roll = DualityRoll::from_values(7, 7); // Critical, total 14
        let result = roll.with_modifier(0);

        assert_eq!(result.success_type(12), SuccessType::CriticalSuccess);
    }

    #[test]
    fn test_failure() {
        let roll = DualityRoll::from_values(3, 2); // Total 5
        let result = roll.with_modifier(0);

        assert_eq!(result.success_type(12), SuccessType::Failure);
    }

    #[test]
    fn test_roll_produces_valid_values() {
        for _ in 0..20 {
            let roll = DualityRoll::roll();
            assert!((1..=12).contains(&roll.hope));
            assert!((1..=12).contains(&roll.fear));
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Strategy to generate valid d12 rolls (1-12)
    fn d12_value() -> impl Strategy<Value = u8> {
        1u8..=12
    }

    proptest! {
        #[test]
        fn prop_from_values_creates_valid_roll(hope in d12_value(), fear in d12_value()) {
            let roll = DualityRoll::from_values(hope, fear);
            prop_assert_eq!(roll.hope, hope);
            prop_assert_eq!(roll.fear, fear);
        }

        #[test]
        fn prop_critical_detection_correct(value in d12_value()) {
            let roll = DualityRoll::from_values(value, value);
            prop_assert!(roll.is_critical(), "Doubles should always be critical");
        }

        #[test]
        fn prop_non_critical_when_different(hope in d12_value(), fear in d12_value()) {
            prop_assume!(hope != fear);  // Only test when different

            let roll = DualityRoll::from_values(hope, fear);
            prop_assert!(!roll.is_critical(), "Non-doubles should not be critical");
        }

        #[test]
        fn prop_controlling_die_correct(hope in d12_value(), fear in d12_value()) {
            let roll = DualityRoll::from_values(hope, fear);
            let controlling = roll.controlling_die();

            if hope > fear {
                prop_assert_eq!(controlling, ControllingDie::Hope);
            } else if fear > hope {
                prop_assert_eq!(controlling, ControllingDie::Fear);
            } else {
                prop_assert_eq!(controlling, ControllingDie::Tied);
            }
        }

        #[test]
        fn prop_modifier_adds_to_total(
            hope in d12_value(),
            fear in d12_value(),
            modifier in -10i8..=10
        ) {
            let roll = DualityRoll::from_values(hope, fear);
            let result = roll.with_modifier(modifier);

            let expected = (hope as i16 + fear as i16 + modifier as i16) as u16;
            prop_assert_eq!(result.total, expected);
        }

        #[test]
        fn prop_critical_preserved_through_modifier(
            value in d12_value(),
            modifier in -10i8..=10
        ) {
            let roll = DualityRoll::from_values(value, value);
            let result = roll.with_modifier(modifier);

            prop_assert!(result.is_critical, "Critical should be preserved");
        }

        #[test]
        fn prop_success_type_matches_total(
            hope in d12_value(),
            fear in d12_value(),
            difficulty in 1u16..=24
        ) {
            let roll = DualityRoll::from_values(hope, fear);
            let result = roll.with_modifier(0);
            let success_type = result.success_type(difficulty);

            if result.total < difficulty {
                prop_assert_eq!(success_type, SuccessType::Failure);
            } else if result.is_critical {
                prop_assert_eq!(success_type, SuccessType::CriticalSuccess);
            } else if result.controlling == ControllingDie::Hope {
                prop_assert_eq!(success_type, SuccessType::SuccessWithHope);
            } else {
                prop_assert_eq!(success_type, SuccessType::SuccessWithFear);
            }
        }

        #[test]
        fn prop_advantage_adds_d6(hope in d12_value(), fear in d12_value()) {
            let roll = DualityRoll::from_values(hope, fear);
            let result = roll.with_advantage();

            prop_assert!(result.advantage_die.is_some(), "Advantage should add d6");

            let d6 = result.advantage_die.unwrap();
            prop_assert!((1..=6).contains(&d6), "Advantage die should be valid d6");

            let expected = hope as u16 + fear as u16 + d6 as u16;
            prop_assert_eq!(result.total, expected);
        }

        #[test]
        fn prop_is_success_threshold_correct(
            hope in d12_value(),
            fear in d12_value(),
            modifier in -10i8..=10
        ) {
            let roll = DualityRoll::from_values(hope, fear);
            let result = roll.with_modifier(modifier);

            // Test difficulty at total - 1, total, and total + 1
            let total = result.total;

            // Always succeeds at exact total
            prop_assert!(result.is_success(total), "Should succeed at exact total");

            // If total > 0, should succeed at total - 1
            if total > 0 {
                prop_assert!(result.is_success(total - 1), "Should succeed below total");
            }

            // Should fail at total + 1 (unless total is u16::MAX)
            if total < u16::MAX {
                prop_assert!(!result.is_success(total + 1), "Should fail above total");
            }
        }
    }
}
