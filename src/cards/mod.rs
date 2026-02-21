//! Domain cards and abilities

use crate::character::Domain;
use serde::{Deserialize, Serialize};

/// Range categories for abilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Range {
    /// Very Close (melee, adjacent)
    VeryClose,
    /// Close (nearby, short range)
    Close,
    /// Far (long range, distant)
    Far,
    /// Any range (no range restriction)
    Any,
}

/// Target type for abilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Target {
    /// Self only
    SelfOnly,
    /// Single ally
    Ally,
    /// Single enemy
    Enemy,
    /// All allies
    AllAllies,
    /// All enemies
    AllEnemies,
    /// Any character (ally or enemy)
    Any,
}

/// Action cost type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionCost {
    /// Major action (one per turn)
    Major,
    /// Minor action (one per turn)
    Minor,
    /// Reaction (triggered by specific events)
    Reaction,
    /// Free action (no cost)
    Free,
}

/// Domain card representing an ability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCard {
    /// Unique identifier
    pub id: String,
    /// Card name
    pub name: String,
    /// Which domain this card belongs to
    pub domain: Domain,
    /// Minimum character level required
    pub level_requirement: u8,
    /// Card description
    pub description: String,
    /// Action cost to use this card
    pub action_cost: ActionCost,
}

impl DomainCard {
    /// Create a new domain card
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::cards::{DomainCard, ActionCost};
    /// use daggerheart_engine::character::Domain;
    ///
    /// let card = DomainCard::new(
    ///     "blade_strike",
    ///     "Blade Strike",
    ///     Domain::Blade,
    ///     1,
    ///     "A swift sword strike",
    ///     ActionCost::Major,
    /// );
    ///
    /// assert_eq!(card.name, "Blade Strike");
    /// assert_eq!(card.domain, Domain::Blade);
    /// ```
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        domain: Domain,
        level_requirement: u8,
        description: impl Into<String>,
        action_cost: ActionCost,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            domain,
            level_requirement,
            description: description.into(),
            action_cost,
        }
    }

    /// Check if a character of given level can use this card
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::cards::{DomainCard, ActionCost};
    /// use daggerheart_engine::character::Domain;
    ///
    /// let card = DomainCard::new(
    ///     "blade_strike",
    ///     "Blade Strike",
    ///     Domain::Blade,
    ///     3,
    ///     "A powerful strike",
    ///     ActionCost::Major,
    /// );
    ///
    /// assert!(!card.can_use(2)); // Level 2 can't use level 3 card
    /// assert!(card.can_use(3));  // Level 3 can use
    /// assert!(card.can_use(5));  // Higher level can use
    /// ```
    pub fn can_use(&self, character_level: u8) -> bool {
        character_level >= self.level_requirement
    }

    /// Check if this is a major action card
    pub fn is_major_action(&self) -> bool {
        matches!(self.action_cost, ActionCost::Major)
    }

    /// Check if this is a minor action card
    pub fn is_minor_action(&self) -> bool {
        matches!(self.action_cost, ActionCost::Minor)
    }

    /// Check if this is a reaction card
    pub fn is_reaction(&self) -> bool {
        matches!(self.action_cost, ActionCost::Reaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_variants() {
        let ranges = [Range::VeryClose, Range::Close, Range::Far, Range::Any];
        assert_eq!(ranges.len(), 4);
    }

    #[test]
    fn test_target_variants() {
        let targets = [
            Target::SelfOnly,
            Target::Ally,
            Target::Enemy,
            Target::AllAllies,
            Target::AllEnemies,
            Target::Any,
        ];
        assert_eq!(targets.len(), 6);
    }

    #[test]
    fn test_action_cost_variants() {
        let costs = [
            ActionCost::Major,
            ActionCost::Minor,
            ActionCost::Reaction,
            ActionCost::Free,
        ];
        assert_eq!(costs.len(), 4);
    }

    #[test]
    fn test_create_domain_card() {
        let card = DomainCard::new(
            "test_card",
            "Test Card",
            Domain::Blade,
            1,
            "A test card",
            ActionCost::Major,
        );

        assert_eq!(card.id, "test_card");
        assert_eq!(card.name, "Test Card");
        assert_eq!(card.domain, Domain::Blade);
        assert_eq!(card.level_requirement, 1);
        assert_eq!(card.description, "A test card");
        assert_eq!(card.action_cost, ActionCost::Major);
    }

    #[test]
    fn test_can_use_exact_level() {
        let card = DomainCard::new(
            "card",
            "Card",
            Domain::Arcana,
            3,
            "Level 3 card",
            ActionCost::Major,
        );

        assert!(card.can_use(3));
    }

    #[test]
    fn test_can_use_higher_level() {
        let card = DomainCard::new(
            "card",
            "Card",
            Domain::Arcana,
            2,
            "Level 2 card",
            ActionCost::Major,
        );

        assert!(card.can_use(5));
    }

    #[test]
    fn test_cannot_use_lower_level() {
        let card = DomainCard::new(
            "card",
            "Card",
            Domain::Arcana,
            5,
            "Level 5 card",
            ActionCost::Major,
        );

        assert!(!card.can_use(3));
    }

    #[test]
    fn test_is_major_action() {
        let card = DomainCard::new(
            "card",
            "Card",
            Domain::Blade,
            1,
            "Major action",
            ActionCost::Major,
        );

        assert!(card.is_major_action());
        assert!(!card.is_minor_action());
        assert!(!card.is_reaction());
    }

    #[test]
    fn test_is_minor_action() {
        let card = DomainCard::new(
            "card",
            "Card",
            Domain::Blade,
            1,
            "Minor action",
            ActionCost::Minor,
        );

        assert!(!card.is_major_action());
        assert!(card.is_minor_action());
        assert!(!card.is_reaction());
    }

    #[test]
    fn test_is_reaction() {
        let card = DomainCard::new(
            "card",
            "Card",
            Domain::Blade,
            1,
            "Reaction",
            ActionCost::Reaction,
        );

        assert!(!card.is_major_action());
        assert!(!card.is_minor_action());
        assert!(card.is_reaction());
    }

    #[test]
    fn test_card_serialization() {
        let card = DomainCard::new(
            "blade_strike",
            "Blade Strike",
            Domain::Blade,
            1,
            "A swift strike",
            ActionCost::Major,
        );

        let json = serde_json::to_string(&card).unwrap();
        let loaded: DomainCard = serde_json::from_str(&json).unwrap();

        assert_eq!(card, loaded);
    }

    #[test]
    fn test_range_serialization() {
        let range = Range::Close;
        let json = serde_json::to_string(&range).unwrap();
        let loaded: Range = serde_json::from_str(&json).unwrap();
        assert_eq!(range, loaded);
    }

    #[test]
    fn test_target_serialization() {
        let target = Target::Enemy;
        let json = serde_json::to_string(&target).unwrap();
        let loaded: Target = serde_json::from_str(&json).unwrap();
        assert_eq!(target, loaded);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_can_use_is_monotonic(
            req_level in 1u8..10,
            char_level in 1u8..10,
        ) {
            let card = DomainCard::new(
                "card",
                "Card",
                Domain::Arcana,
                req_level,
                "Test",
                ActionCost::Major,
            );

            let can_use = card.can_use(char_level);
            let expected = char_level >= req_level;

            prop_assert_eq!(can_use, expected);
        }

        #[test]
        fn prop_level_0_can_use_nothing(
            req_level in 1u8..10,
        ) {
            let card = DomainCard::new(
                "card",
                "Card",
                Domain::Arcana,
                req_level,
                "Test",
                ActionCost::Major,
            );

            // Level 0 character can't use any card requiring level >= 1
            prop_assert!(!card.can_use(0));
        }

        #[test]
        fn prop_action_type_methods_are_exclusive(
            action_cost in prop_oneof![
                Just(ActionCost::Major),
                Just(ActionCost::Minor),
                Just(ActionCost::Reaction),
                Just(ActionCost::Free),
            ],
        ) {
            let card = DomainCard::new(
                "card",
                "Card",
                Domain::Arcana,
                1,
                "Test",
                action_cost,
            );

            // Exactly one of the action type methods should be true
            // (or none for Free action)
            let count = [
                card.is_major_action(),
                card.is_minor_action(),
                card.is_reaction(),
            ].iter().filter(|&&x| x).count();

            prop_assert!(count <= 1);
        }
    }
}
