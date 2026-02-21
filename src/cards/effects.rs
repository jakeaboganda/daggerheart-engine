//! Card effects and abilities

use crate::core::dice::DamageDice;
use serde::{Deserialize, Serialize};

use super::{Range, Target};

/// Duration of an effect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Duration {
    /// Effect lasts until end of current turn
    EndOfTurn,
    /// Effect lasts until end of next turn
    EndOfNextTurn,
    /// Effect lasts for a specific number of rounds
    Rounds(u8),
    /// Effect is permanent/until removed
    Permanent,
    /// Instantaneous effect (no duration)
    Instant,
}

/// Type of card effect
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardEffect {
    /// Deal damage to target(s)
    Attack {
        /// Damage dice to roll
        damage: DamageDice,
        /// Range of the attack
        range: Range,
        /// Who can be targeted
        target: Target,
    },
    /// Heal hit points
    Heal {
        /// Amount to heal (0 = full heal)
        amount: u8,
        /// Who can be targeted
        target: Target,
    },
    /// Apply a modifier buff/debuff
    Modifier {
        /// Bonus/penalty to apply
        bonus: i8,
        /// Who gets the modifier
        target: Target,
        /// How long it lasts
        duration: Duration,
        /// Description of what it modifies
        applies_to: String,
    },
    /// Clear stress
    ClearStress {
        /// Who to clear stress from
        target: Target,
    },
    /// Move a character
    Move {
        /// Range of movement
        distance: Range,
        /// Who can be moved
        target: Target,
    },
    /// Complex effect (description only for now)
    Special {
        /// Description of the special effect
        description: String,
    },
}

impl CardEffect {
    /// Create an attack effect
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::cards::{Range, Target};
    /// use daggerheart_engine::cards::effects::CardEffect;
    /// use daggerheart_engine::core::dice::{DamageDice, Die};
    ///
    /// let effect = CardEffect::attack(
    ///     DamageDice::new(vec![Die::D6, Die::D6]),
    ///     Range::Close,
    ///     Target::Enemy,
    /// );
    ///
    /// assert!(effect.is_attack());
    /// ```
    pub fn attack(damage: DamageDice, range: Range, target: Target) -> Self {
        Self::Attack {
            damage,
            range,
            target,
        }
    }

    /// Create a heal effect
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::cards::Target;
    /// use daggerheart_engine::cards::effects::CardEffect;
    ///
    /// let effect = CardEffect::heal(5, Target::Ally);
    /// assert!(effect.is_heal());
    /// ```
    pub fn heal(amount: u8, target: Target) -> Self {
        Self::Heal { amount, target }
    }

    /// Create a modifier effect
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::cards::Target;
    /// use daggerheart_engine::cards::effects::{CardEffect, Duration};
    ///
    /// let effect = CardEffect::modifier(
    ///     2,
    ///     Target::SelfOnly,
    ///     Duration::EndOfTurn,
    ///     "attack rolls",
    /// );
    ///
    /// assert!(effect.is_modifier());
    /// ```
    pub fn modifier(
        bonus: i8,
        target: Target,
        duration: Duration,
        applies_to: impl Into<String>,
    ) -> Self {
        Self::Modifier {
            bonus,
            target,
            duration,
            applies_to: applies_to.into(),
        }
    }

    /// Check if this is an attack effect
    pub fn is_attack(&self) -> bool {
        matches!(self, Self::Attack { .. })
    }

    /// Check if this is a heal effect
    pub fn is_heal(&self) -> bool {
        matches!(self, Self::Heal { .. })
    }

    /// Check if this is a modifier effect
    pub fn is_modifier(&self) -> bool {
        matches!(self, Self::Modifier { .. })
    }

    /// Check if this is a clear stress effect
    pub fn is_clear_stress(&self) -> bool {
        matches!(self, Self::ClearStress { .. })
    }

    /// Check if this effect targets enemies
    pub fn targets_enemies(&self) -> bool {
        match self {
            Self::Attack { target, .. } => {
                matches!(target, Target::Enemy | Target::AllEnemies | Target::Any)
            }
            Self::Heal { target, .. } => matches!(target, Target::Enemy | Target::Any),
            Self::Modifier { target, .. } => {
                matches!(target, Target::Enemy | Target::AllEnemies | Target::Any)
            }
            Self::ClearStress { target } => {
                matches!(target, Target::Enemy | Target::AllEnemies | Target::Any)
            }
            Self::Move { target, .. } => {
                matches!(target, Target::Enemy | Target::AllEnemies | Target::Any)
            }
            Self::Special { .. } => false, // Unknown
        }
    }

    /// Check if this effect targets allies
    pub fn targets_allies(&self) -> bool {
        match self {
            Self::Attack { target, .. } => {
                matches!(target, Target::Ally | Target::AllAllies | Target::Any)
            }
            Self::Heal { target, .. } => {
                matches!(
                    target,
                    Target::SelfOnly | Target::Ally | Target::AllAllies | Target::Any
                )
            }
            Self::Modifier { target, .. } => {
                matches!(
                    target,
                    Target::SelfOnly | Target::Ally | Target::AllAllies | Target::Any
                )
            }
            Self::ClearStress { target } => {
                matches!(
                    target,
                    Target::SelfOnly | Target::Ally | Target::AllAllies | Target::Any
                )
            }
            Self::Move { target, .. } => {
                matches!(
                    target,
                    Target::SelfOnly | Target::Ally | Target::AllAllies | Target::Any
                )
            }
            Self::Special { .. } => false, // Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dice::Die;

    #[test]
    fn test_duration_variants() {
        let durations = [
            Duration::EndOfTurn,
            Duration::EndOfNextTurn,
            Duration::Rounds(3),
            Duration::Permanent,
            Duration::Instant,
        ];
        assert_eq!(durations.len(), 5);
    }

    #[test]
    fn test_create_attack_effect() {
        let effect = CardEffect::attack(
            DamageDice::new(vec![Die::D6, Die::D6]),
            Range::Close,
            Target::Enemy,
        );

        assert!(effect.is_attack());
        assert!(!effect.is_heal());
        assert!(!effect.is_modifier());
        assert!(effect.targets_enemies());
        assert!(!effect.targets_allies());
    }

    #[test]
    fn test_create_heal_effect() {
        let effect = CardEffect::heal(5, Target::Ally);

        assert!(!effect.is_attack());
        assert!(effect.is_heal());
        assert!(!effect.is_modifier());
        assert!(!effect.targets_enemies());
        assert!(effect.targets_allies());
    }

    #[test]
    fn test_create_modifier_effect() {
        let effect = CardEffect::modifier(2, Target::SelfOnly, Duration::EndOfTurn, "attacks");

        assert!(!effect.is_attack());
        assert!(!effect.is_heal());
        assert!(effect.is_modifier());
        assert!(!effect.targets_enemies());
        assert!(effect.targets_allies());
    }

    #[test]
    fn test_clear_stress_effect() {
        let effect = CardEffect::ClearStress {
            target: Target::SelfOnly,
        };

        assert!(effect.is_clear_stress());
        assert!(!effect.targets_enemies());
        assert!(effect.targets_allies());
    }

    #[test]
    fn test_attack_with_very_close_range() {
        let effect = CardEffect::attack(
            DamageDice::new(vec![Die::D8]),
            Range::VeryClose,
            Target::Enemy,
        );

        if let CardEffect::Attack { range, .. } = effect {
            assert_eq!(range, Range::VeryClose);
        } else {
            panic!("Expected Attack effect");
        }
    }

    #[test]
    fn test_heal_full() {
        let effect = CardEffect::heal(0, Target::SelfOnly);

        if let CardEffect::Heal { amount, .. } = effect {
            assert_eq!(amount, 0); // 0 = full heal
        } else {
            panic!("Expected Heal effect");
        }
    }

    #[test]
    fn test_negative_modifier() {
        let effect = CardEffect::modifier(-2, Target::Enemy, Duration::Rounds(2), "defense");

        if let CardEffect::Modifier { bonus, .. } = effect {
            assert_eq!(bonus, -2);
        } else {
            panic!("Expected Modifier effect");
        }
    }

    #[test]
    fn test_permanent_duration() {
        let effect = CardEffect::modifier(1, Target::SelfOnly, Duration::Permanent, "hp");

        if let CardEffect::Modifier { duration, .. } = effect {
            assert_eq!(duration, Duration::Permanent);
        } else {
            panic!("Expected Modifier effect");
        }
    }

    #[test]
    fn test_multi_target_attack() {
        let effect = CardEffect::attack(
            DamageDice::new(vec![Die::D6, Die::D6]),
            Range::Far,
            Target::AllEnemies,
        );

        if let CardEffect::Attack { target, .. } = effect {
            assert_eq!(target, Target::AllEnemies);
        } else {
            panic!("Expected Attack effect");
        }
    }

    #[test]
    fn test_effect_serialization() {
        let effect = CardEffect::attack(
            DamageDice::new(vec![Die::D6, Die::D6]),
            Range::Close,
            Target::Enemy,
        );

        let json = serde_json::to_string(&effect).unwrap();
        let loaded: CardEffect = serde_json::from_str(&json).unwrap();

        assert_eq!(effect, loaded);
    }

    #[test]
    fn test_duration_serialization() {
        let duration = Duration::Rounds(5);
        let json = serde_json::to_string(&duration).unwrap();
        let loaded: Duration = serde_json::from_str(&json).unwrap();
        assert_eq!(duration, loaded);
    }

    #[test]
    fn test_target_any_targets_both() {
        let effect = CardEffect::attack(DamageDice::new(vec![Die::D6]), Range::Any, Target::Any);

        assert!(effect.targets_enemies());
        assert!(effect.targets_allies());
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use crate::core::dice::Die;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_attack_always_targets_someone(
            range in prop_oneof![
                Just(Range::VeryClose),
                Just(Range::Close),
                Just(Range::Far),
                Just(Range::Any),
            ],
            target in prop_oneof![
                Just(Target::Enemy),
                Just(Target::AllEnemies),
                Just(Target::Any),
            ],
        ) {
            let effect = CardEffect::attack(
                DamageDice::new(vec![Die::D6]),
                range,
                target,
            );

            // Attacks should target someone
            prop_assert!(effect.targets_enemies() || effect.targets_allies());
        }

        #[test]
        fn prop_heal_amount_is_preserved(
            amount in 0u8..100,
        ) {
            let effect = CardEffect::heal(amount, Target::SelfOnly);

            if let CardEffect::Heal { amount: heal_amount, .. } = effect {
                prop_assert_eq!(heal_amount, amount);
            }
        }

        #[test]
        fn prop_modifier_bonus_can_be_negative(
            bonus in -10i8..10,
        ) {
            let effect = CardEffect::modifier(
                bonus,
                Target::SelfOnly,
                Duration::EndOfTurn,
                "test",
            );

            if let CardEffect::Modifier { bonus: mod_bonus, .. } = effect {
                prop_assert_eq!(mod_bonus, bonus);
            }
        }

        #[test]
        fn prop_self_only_targets_allies(
            effect in prop_oneof![
                Just(CardEffect::heal(5, Target::SelfOnly)),
                Just(CardEffect::modifier(2, Target::SelfOnly, Duration::EndOfTurn, "test")),
                Just(CardEffect::ClearStress { target: Target::SelfOnly }),
            ],
        ) {
            // SelfOnly should count as targeting allies
            prop_assert!(effect.targets_allies());
            prop_assert!(!effect.targets_enemies());
        }
    }
}
