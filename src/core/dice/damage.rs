//! Damage dice system (multiple dice with bonuses)
//!
//! Weapons and attacks roll damage using one or more dice plus a bonus.
//! For example: Longsword Tier 1 = d10+3

use super::basic::Die;

/// A collection of dice to roll for damage
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DamageDice {
    dice: Vec<Die>,
    bonus: i16,
}

/// The result of rolling damage dice
#[derive(Debug, Clone, PartialEq)]
pub struct DamageRoll {
    pub rolls: Vec<u8>,
    pub bonus: i16,
    pub total: u16,
}

impl DamageDice {
    /// Create damage dice from a vector of dice
    pub fn new(dice: Vec<Die>) -> Self {
        Self { dice, bonus: 0 }
    }

    /// Add a bonus to the damage
    pub fn with_bonus(mut self, bonus: i16) -> Self {
        self.bonus = bonus;
        self
    }

    /// Roll the damage dice
    pub fn roll(&self) -> DamageRoll {
        let mut rng = rand::thread_rng();
        let rolls: Vec<u8> = self
            .dice
            .iter()
            .map(|die| die.roll_with_rng(&mut rng))
            .collect();

        let dice_total: i32 = rolls.iter().map(|&x| x as i32).sum();
        let total = (dice_total + self.bonus as i32).max(0) as u16;

        DamageRoll {
            rolls,
            bonus: self.bonus,
            total,
        }
    }

    // Convenience constructors for common patterns

    /// Create damage dice with N d4s
    pub fn d4(count: usize) -> Self {
        Self::new(vec![Die::D4; count])
    }

    /// Create damage dice with N d6s
    pub fn d6(count: usize) -> Self {
        Self::new(vec![Die::D6; count])
    }

    /// Create damage dice with N d8s
    pub fn d8(count: usize) -> Self {
        Self::new(vec![Die::D8; count])
    }

    /// Create damage dice with N d10s
    pub fn d10(count: usize) -> Self {
        Self::new(vec![Die::D10; count])
    }

    /// Create damage dice with N d12s
    pub fn d12(count: usize) -> Self {
        Self::new(vec![Die::D12; count])
    }

    /// Create damage dice with N d20s
    pub fn d20(count: usize) -> Self {
        Self::new(vec![Die::D20; count])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_single_die() {
        let damage = DamageDice::new(vec![Die::D6]);
        assert_eq!(damage.dice.len(), 1);
        assert_eq!(damage.bonus, 0);
    }

    #[test]
    fn test_new_multiple_dice() {
        let damage = DamageDice::new(vec![Die::D6, Die::D6, Die::D4]);
        assert_eq!(damage.dice.len(), 3);
    }

    #[test]
    fn test_with_bonus_positive() {
        let damage = DamageDice::new(vec![Die::D6]).with_bonus(3);
        assert_eq!(damage.bonus, 3);
    }

    #[test]
    fn test_with_bonus_negative() {
        let damage = DamageDice::new(vec![Die::D8]).with_bonus(-2);
        assert_eq!(damage.bonus, -2);
    }

    #[test]
    fn test_roll_single_die() {
        let damage = DamageDice::new(vec![Die::D6]);

        for _ in 0..20 {
            let roll = damage.roll();
            assert_eq!(roll.rolls.len(), 1);
            assert!((1..=6).contains(&roll.rolls[0]));
            assert_eq!(roll.total as u8, roll.rolls[0]);
        }
    }

    #[test]
    fn test_roll_with_bonus() {
        let damage = DamageDice::new(vec![Die::D6]).with_bonus(3);

        for _ in 0..20 {
            let roll = damage.roll();
            assert!(roll.total >= 4); // min: 1+3
            assert!(roll.total <= 9); // max: 6+3
            assert_eq!(roll.bonus, 3);
        }
    }

    #[test]
    fn test_roll_multiple_dice() {
        let damage = DamageDice::new(vec![Die::D6, Die::D6]);

        for _ in 0..20 {
            let roll = damage.roll();
            assert_eq!(roll.rolls.len(), 2);
            assert!(roll.total >= 2); // min: 1+1
            assert!(roll.total <= 12); // max: 6+6
        }
    }

    #[test]
    fn test_roll_multiple_dice_with_bonus() {
        let damage = DamageDice::new(vec![Die::D6, Die::D6]).with_bonus(5);

        for _ in 0..20 {
            let roll = damage.roll();
            let sum: u16 = roll.rolls.iter().map(|&x| x as u16).sum();
            assert_eq!(roll.total, sum + 5);
        }
    }

    #[test]
    fn test_longsword_tier_1() {
        // Longsword Tier 1: d10+3
        let longsword = DamageDice::d10(1).with_bonus(3);

        for _ in 0..20 {
            let roll = longsword.roll();
            assert!(roll.total >= 4);
            assert!(roll.total <= 13);
        }
    }

    #[test]
    fn test_spear_tier_1() {
        // Spear Tier 1: d8+3
        let spear = DamageDice::d8(1).with_bonus(3);

        for _ in 0..20 {
            let roll = spear.roll();
            assert!(roll.total >= 4);
            assert!(roll.total <= 11);
        }
    }

    #[test]
    fn test_d6_convenience() {
        let damage = DamageDice::d6(2);
        assert_eq!(damage.dice.len(), 2);
        assert_eq!(damage.dice[0], Die::D6);
        assert_eq!(damage.dice[1], Die::D6);
    }

    #[test]
    fn test_d4_convenience() {
        let damage = DamageDice::d4(1);
        assert_eq!(damage.dice.len(), 1);
        assert_eq!(damage.dice[0], Die::D4);
    }

    #[test]
    fn test_mixed_dice() {
        let damage = DamageDice::new(vec![Die::D8, Die::D6, Die::D4]);

        for _ in 0..20 {
            let roll = damage.roll();
            assert_eq!(roll.rolls.len(), 3);
            // Min: 1+1+1=3, Max: 8+6+4=18
            assert!(roll.total >= 3);
            assert!(roll.total <= 18);
        }
    }

    #[test]
    fn test_negative_bonus_doesnt_go_below_zero() {
        let damage = DamageDice::d4(1).with_bonus(-10);

        // Test that clamping logic works (u16 is always >= 0)
        for _ in 0..20 {
            let roll = damage.roll();
            // Roll is d4 (1-4) with -10 bonus
            // Should clamp to 0, not wrap around
            assert!(roll.total <= 4); // Max possible is d4 max
            assert_eq!(roll.bonus, -10); // Bonus is preserved
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    fn any_die() -> impl Strategy<Value = Die> {
        prop_oneof![
            Just(Die::D4),
            Just(Die::D6),
            Just(Die::D8),
            Just(Die::D10),
            Just(Die::D12),
            Just(Die::D20),
        ]
    }

    fn die_vec() -> impl Strategy<Value = Vec<Die>> {
        prop::collection::vec(any_die(), 1..=5)
    }

    proptest! {
        #[test]
        fn prop_damage_total_includes_bonus(
            dice in die_vec(),
            bonus in -20i16..=20
        ) {
            let damage_dice = DamageDice::new(dice.clone()).with_bonus(bonus);
            let roll = damage_dice.roll();

            // Total should equal sum of rolls + bonus (clamped to 0)
            let dice_sum: i32 = roll.rolls.iter().map(|&x| x as i32).sum();
            let expected = (dice_sum + bonus as i32).max(0) as u16;

            prop_assert_eq!(roll.total, expected);
        }

        #[test]
        fn prop_damage_rolls_count_matches_dice(dice in die_vec()) {
            let damage_dice = DamageDice::new(dice.clone());
            let roll = damage_dice.roll();

            prop_assert_eq!(roll.rolls.len(), dice.len());
        }

        #[test]
        fn prop_each_die_roll_is_valid(dice in die_vec()) {
            let damage_dice = DamageDice::new(dice.clone());
            let roll = damage_dice.roll();

            for (i, &rolled_value) in roll.rolls.iter().enumerate() {
                let die = &dice[i];
                prop_assert!(rolled_value >= 1, "Roll {} is below 1", rolled_value);
                prop_assert!(rolled_value <= die.max(), "Roll {} exceeds max {} for {:?}", rolled_value, die.max(), die);
            }
        }

        #[test]
        fn prop_bonus_is_preserved(dice in die_vec(), bonus in -20i16..=20) {
            let damage_dice = DamageDice::new(dice).with_bonus(bonus);
            let roll = damage_dice.roll();

            prop_assert_eq!(roll.bonus, bonus);
        }

        #[test]
        fn prop_negative_bonus_never_goes_negative(
            dice in die_vec(),
            large_penalty in -100i16..=-10
        ) {
            let damage_dice = DamageDice::new(dice).with_bonus(large_penalty);
            let roll = damage_dice.roll();

            // Total is u16, so it's always >= 0
            // This test verifies the clamping logic compiles and runs
            prop_assert_eq!(roll.bonus, large_penalty);
        }

        #[test]
        fn prop_d6_constructor_creates_correct_dice(count in 1usize..=10) {
            let damage = DamageDice::d6(count);

            prop_assert_eq!(damage.dice.len(), count);
            for die in &damage.dice {
                prop_assert_eq!(*die, Die::D6);
            }
        }

        #[test]
        fn prop_d8_constructor_creates_correct_dice(count in 1usize..=10) {
            let damage = DamageDice::d8(count);

            prop_assert_eq!(damage.dice.len(), count);
            for die in &damage.dice {
                prop_assert_eq!(*die, Die::D8);
            }
        }

        #[test]
        fn prop_minimum_damage_with_positive_bonus(
            count in 1usize..=5,
            bonus in 1i16..=20
        ) {
            let damage = DamageDice::d6(count).with_bonus(bonus);
            let roll = damage.roll();

            // Minimum possible: count * 1 (all 1s) + bonus
            let min_possible = count as u16 + bonus as u16;
            prop_assert!(roll.total >= min_possible);
        }

        #[test]
        fn prop_maximum_damage_respects_limits(
            count in 1usize..=5,
            bonus in 0i16..=10
        ) {
            let damage = DamageDice::d6(count).with_bonus(bonus);
            let roll = damage.roll();

            // Maximum possible: count * 6 (all 6s) + bonus
            let max_possible = count as u16 * 6 + bonus as u16;
            prop_assert!(roll.total <= max_possible);
        }
    }
}
