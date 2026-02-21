//! Combat resources - HP, Stress, Hope, and Fear

use crate::error::EngineError;
use serde::{Deserialize, Serialize};

/// Hit Points pool
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HitPoints {
    pub current: u8,
    pub maximum: u8,
}

impl HitPoints {
    /// Create a new HP pool
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::HitPoints;
    ///
    /// let hp = HitPoints::new(6);
    /// assert_eq!(hp.current, 6);
    /// assert_eq!(hp.maximum, 6);
    /// ```
    pub fn new(maximum: u8) -> Self {
        Self {
            current: maximum,
            maximum,
        }
    }

    /// Take damage (reduce current HP)
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::HitPoints;
    ///
    /// let mut hp = HitPoints::new(6);
    /// hp.take_damage(2);
    /// assert_eq!(hp.current, 4);
    /// ```
    pub fn take_damage(&mut self, amount: u8) {
        self.current = self.current.saturating_sub(amount);
    }

    /// Heal damage (increase current HP, capped at maximum)
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::HitPoints;
    ///
    /// let mut hp = HitPoints::new(6);
    /// hp.take_damage(3);
    /// hp.heal(2);
    /// assert_eq!(hp.current, 5);
    /// ```
    pub fn heal(&mut self, amount: u8) {
        self.current = (self.current + amount).min(self.maximum);
    }

    /// Check if character is alive (HP > 0)
    pub fn is_alive(&self) -> bool {
        self.current > 0
    }

    /// Check if character is at full HP
    pub fn is_full(&self) -> bool {
        self.current == self.maximum
    }
}

/// Stress tracking
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stress {
    pub current: u8,
}

impl Stress {
    /// Create a new stress tracker (starts at 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Stress;
    ///
    /// let stress = Stress::new();
    /// assert_eq!(stress.current, 0);
    /// ```
    pub fn new() -> Self {
        Self { current: 0 }
    }

    /// Gain stress
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Stress;
    ///
    /// let mut stress = Stress::new();
    /// stress.gain(2);
    /// assert_eq!(stress.current, 2);
    /// ```
    pub fn gain(&mut self, amount: u8) {
        self.current = self.current.saturating_add(amount);
    }

    /// Clear all stress
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Stress;
    ///
    /// let mut stress = Stress::new();
    /// stress.gain(5);
    /// stress.clear();
    /// assert_eq!(stress.current, 0);
    /// ```
    pub fn clear(&mut self) {
        self.current = 0;
    }
}

impl Default for Stress {
    fn default() -> Self {
        Self::new()
    }
}

/// Hope resource pool
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hope {
    pub current: u8,
    pub maximum: u8,
}

impl Hope {
    /// Create a new Hope pool
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Hope;
    ///
    /// let hope = Hope::new(5);
    /// assert_eq!(hope.current, 5);
    /// assert_eq!(hope.maximum, 5);
    /// ```
    pub fn new(maximum: u8) -> Self {
        Self {
            current: maximum,
            maximum,
        }
    }

    /// Gain hope (capped at maximum)
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Hope;
    ///
    /// let mut hope = Hope::new(5);
    /// hope.spend(2).unwrap();
    /// hope.gain(1);
    /// assert_eq!(hope.current, 4);
    /// ```
    pub fn gain(&mut self, amount: u8) {
        self.current = (self.current + amount).min(self.maximum);
    }

    /// Spend hope
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Hope;
    ///
    /// let mut hope = Hope::new(5);
    /// assert!(hope.spend(2).is_ok());
    /// assert_eq!(hope.current, 3);
    ///
    /// // Can't spend more than available
    /// assert!(hope.spend(10).is_err());
    /// ```
    pub fn spend(&mut self, amount: u8) -> Result<(), EngineError> {
        if self.current >= amount {
            self.current -= amount;
            Ok(())
        } else {
            Err(EngineError::ResourceExceeded(format!(
                "Not enough Hope: have {}, need {}",
                self.current, amount
            )))
        }
    }

    /// Reduce maximum Hope (permanent cost)
    pub fn reduce_maximum(&mut self, amount: u8) {
        self.maximum = self.maximum.saturating_sub(amount);
        self.current = self.current.min(self.maximum);
    }
}

/// Fear resource pool (GM resource)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fear {
    pub current: u8,
}

impl Fear {
    /// Create a new Fear pool (starts at 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Fear;
    ///
    /// let fear = Fear::new();
    /// assert_eq!(fear.current, 0);
    /// ```
    pub fn new() -> Self {
        Self { current: 0 }
    }

    /// Gain fear
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Fear;
    ///
    /// let mut fear = Fear::new();
    /// fear.gain(3);
    /// assert_eq!(fear.current, 3);
    /// ```
    pub fn gain(&mut self, amount: u8) {
        self.current = self.current.saturating_add(amount);
    }

    /// Spend fear
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::Fear;
    ///
    /// let mut fear = Fear::new();
    /// fear.gain(5);
    /// assert!(fear.spend(2).is_ok());
    /// assert_eq!(fear.current, 3);
    ///
    /// // Can't spend more than available
    /// assert!(fear.spend(10).is_err());
    /// ```
    pub fn spend(&mut self, amount: u8) -> Result<(), EngineError> {
        if self.current >= amount {
            self.current -= amount;
            Ok(())
        } else {
            Err(EngineError::ResourceExceeded(format!(
                "Not enough Fear: have {}, need {}",
                self.current, amount
            )))
        }
    }
}

impl Default for Fear {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // HitPoints tests
    #[test]
    fn test_hp_creation() {
        let hp = HitPoints::new(6);
        assert_eq!(hp.current, 6);
        assert_eq!(hp.maximum, 6);
        assert!(hp.is_alive());
        assert!(hp.is_full());
    }

    #[test]
    fn test_hp_take_damage() {
        let mut hp = HitPoints::new(6);
        hp.take_damage(2);
        assert_eq!(hp.current, 4);
        assert!(hp.is_alive());
        assert!(!hp.is_full());
    }

    #[test]
    fn test_hp_death() {
        let mut hp = HitPoints::new(6);
        hp.take_damage(10); // Overkill
        assert_eq!(hp.current, 0);
        assert!(!hp.is_alive());
    }

    #[test]
    fn test_hp_healing() {
        let mut hp = HitPoints::new(6);
        hp.take_damage(4);
        hp.heal(2);
        assert_eq!(hp.current, 4);
    }

    #[test]
    fn test_hp_healing_capped() {
        let mut hp = HitPoints::new(6);
        hp.take_damage(2);
        hp.heal(10); // Overheal
        assert_eq!(hp.current, 6); // Capped at max
        assert!(hp.is_full());
    }

    // Stress tests
    #[test]
    fn test_stress_creation() {
        let stress = Stress::new();
        assert_eq!(stress.current, 0);
    }

    #[test]
    fn test_stress_gain() {
        let mut stress = Stress::new();
        stress.gain(3);
        assert_eq!(stress.current, 3);
        stress.gain(2);
        assert_eq!(stress.current, 5);
    }

    #[test]
    fn test_stress_clear() {
        let mut stress = Stress::new();
        stress.gain(5);
        stress.clear();
        assert_eq!(stress.current, 0);
    }

    // Hope tests
    #[test]
    fn test_hope_creation() {
        let hope = Hope::new(5);
        assert_eq!(hope.current, 5);
        assert_eq!(hope.maximum, 5);
    }

    #[test]
    fn test_hope_spend() {
        let mut hope = Hope::new(5);
        assert!(hope.spend(2).is_ok());
        assert_eq!(hope.current, 3);
    }

    #[test]
    fn test_hope_spend_too_much() {
        let mut hope = Hope::new(5);
        assert!(hope.spend(10).is_err());
        assert_eq!(hope.current, 5); // Unchanged
    }

    #[test]
    fn test_hope_gain() {
        let mut hope = Hope::new(5);
        hope.spend(3).unwrap();
        hope.gain(2);
        assert_eq!(hope.current, 4);
    }

    #[test]
    fn test_hope_gain_capped() {
        let mut hope = Hope::new(5);
        hope.gain(10); // Try to gain too much
        assert_eq!(hope.current, 5); // Capped at max
    }

    #[test]
    fn test_hope_reduce_maximum() {
        let mut hope = Hope::new(5);
        hope.reduce_maximum(1);
        assert_eq!(hope.maximum, 4);
        assert_eq!(hope.current, 4); // Current adjusted down
    }

    // Fear tests
    #[test]
    fn test_fear_creation() {
        let fear = Fear::new();
        assert_eq!(fear.current, 0);
    }

    #[test]
    fn test_fear_gain() {
        let mut fear = Fear::new();
        fear.gain(3);
        assert_eq!(fear.current, 3);
    }

    #[test]
    fn test_fear_spend() {
        let mut fear = Fear::new();
        fear.gain(5);
        assert!(fear.spend(2).is_ok());
        assert_eq!(fear.current, 3);
    }

    #[test]
    fn test_fear_spend_too_much() {
        let mut fear = Fear::new();
        fear.gain(3);
        assert!(fear.spend(10).is_err());
        assert_eq!(fear.current, 3); // Unchanged
    }

    // Serialization tests
    #[test]
    fn test_hp_serialization() {
        let hp = HitPoints::new(6);
        let json = serde_json::to_string(&hp).unwrap();
        let loaded: HitPoints = serde_json::from_str(&json).unwrap();
        assert_eq!(hp, loaded);
    }

    #[test]
    fn test_hope_serialization() {
        let hope = Hope::new(5);
        let json = serde_json::to_string(&hope).unwrap();
        let loaded: Hope = serde_json::from_str(&json).unwrap();
        assert_eq!(hope, loaded);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_hp_never_exceeds_maximum(
            max in 1u8..20,
            damage in 0u8..10,
            heal in 0u8..20,
        ) {
            let mut hp = HitPoints::new(max);
            hp.take_damage(damage);
            hp.heal(heal);

            prop_assert!(hp.current <= hp.maximum);
        }

        #[test]
        fn prop_hp_never_negative(
            max in 1u8..20,
            damage in 0u8..50,
        ) {
            let mut hp = HitPoints::new(max);
            hp.take_damage(damage);

            // current is u8, so can't be negative, but should be 0
            prop_assert!(hp.current <= max);
        }

        #[test]
        fn prop_stress_always_increases_or_stays_same(
            gain1 in 0u8..10,
            gain2 in 0u8..10,
        ) {
            let mut stress = Stress::new();
            let before = stress.current;
            stress.gain(gain1);
            let after1 = stress.current;
            stress.gain(gain2);
            let after2 = stress.current;

            prop_assert!(after1 >= before);
            prop_assert!(after2 >= after1);
        }

        #[test]
        fn prop_hope_spend_is_reversible_by_gain(
            max in 1u8..20,
            spend in 0u8..10,
        ) {
            let mut hope = Hope::new(max);

            if spend <= max {
                hope.spend(spend).unwrap();
                hope.gain(spend);
                prop_assert_eq!(hope.current, max);
            }
        }

        #[test]
        fn prop_fear_accumulates(
            gains in prop::collection::vec(0u8..10, 1..10),
        ) {
            let mut fear = Fear::new();
            let total: u8 = gains.iter().map(|&x| x as u32).sum::<u32>().min(255) as u8;

            for &gain in &gains {
                fear.gain(gain);
            }

            // May saturate at u8::MAX
            prop_assert!(fear.current <= total || fear.current == 255);
        }
    }
}
