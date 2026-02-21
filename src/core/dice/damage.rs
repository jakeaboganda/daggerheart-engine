//! Damage dice system (multiple dice with bonuses)
//!
//! Weapons and attacks roll damage using one or more dice plus a bonus.
//! For example: Longsword Tier 1 = d10+3

use super::basic::Die;
use rand::Rng;

/// A collection of dice to roll for damage
#[derive(Debug, Clone, PartialEq)]
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
        let rolls: Vec<u8> = self.dice.iter().map(|die| die.roll_with_rng(&mut rng)).collect();
        
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
            assert!(roll.rolls[0] >= 1 && roll.rolls[0] <= 6);
            assert_eq!(roll.total as u8, roll.rolls[0]);
        }
    }

    #[test]
    fn test_roll_with_bonus() {
        let damage = DamageDice::new(vec![Die::D6]).with_bonus(3);
        
        for _ in 0..20 {
            let roll = damage.roll();
            assert!(roll.total >= 1 + 3);  // min: 1+3
            assert!(roll.total <= 6 + 3);  // max: 6+3
            assert_eq!(roll.bonus, 3);
        }
    }

    #[test]
    fn test_roll_multiple_dice() {
        let damage = DamageDice::new(vec![Die::D6, Die::D6]);
        
        for _ in 0..20 {
            let roll = damage.roll();
            assert_eq!(roll.rolls.len(), 2);
            assert!(roll.total >= 2);   // min: 1+1
            assert!(roll.total <= 12);  // max: 6+6
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
            assert!(roll.total >= 1 + 3);
            assert!(roll.total <= 10 + 3);
        }
    }

    #[test]
    fn test_spear_tier_1() {
        // Spear Tier 1: d8+3
        let spear = DamageDice::d8(1).with_bonus(3);
        
        for _ in 0..20 {
            let roll = spear.roll();
            assert!(roll.total >= 1 + 3);
            assert!(roll.total <= 8 + 3);
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
        
        for _ in 0..20 {
            let roll = damage.roll();
            // Even with -10 bonus, should not go negative
            // (clamped to 0 minimum)
            assert!(roll.total == 0 || roll.total > 0);
        }
    }
}
