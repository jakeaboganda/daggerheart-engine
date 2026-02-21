//! Character classes and domains

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// The nine playable classes in Daggerheart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display, Serialize, Deserialize)]
pub enum Class {
    Bard,
    Druid,
    Guardian,
    Ranger,
    Rogue,
    Seraph,
    Sorcerer,
    Warrior,
    Wizard,
}

/// The nine domains that grant special abilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display, Serialize, Deserialize)]
pub enum Domain {
    Arcana,
    Blade,
    Bone,
    Codex,
    Grace,
    Midnight,
    Sage,
    Splendor,
    Valor,
}

impl Class {
    /// Get the two domains associated with this class
    ///
    /// Each class has access to exactly two domains that determine
    /// which abilities they can learn.
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::{Class, Domain};
    ///
    /// assert_eq!(Class::Bard.domains(), (Domain::Codex, Domain::Grace));
    /// assert_eq!(Class::Warrior.domains(), (Domain::Blade, Domain::Bone));
    /// ```
    pub fn domains(&self) -> (Domain, Domain) {
        match self {
            Class::Bard => (Domain::Codex, Domain::Grace),
            Class::Druid => (Domain::Arcana, Domain::Sage),
            Class::Guardian => (Domain::Blade, Domain::Valor),
            Class::Ranger => (Domain::Bone, Domain::Sage),
            Class::Rogue => (Domain::Midnight, Domain::Grace),
            Class::Seraph => (Domain::Codex, Domain::Splendor),
            Class::Sorcerer => (Domain::Arcana, Domain::Midnight),
            Class::Warrior => (Domain::Blade, Domain::Bone),
            Class::Wizard => (Domain::Codex, Domain::Arcana),
        }
    }

    /// Get the starting HP for this class
    ///
    /// All classes start with 6 HP in Daggerheart.
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::Class;
    ///
    /// assert_eq!(Class::Bard.starting_hp(), 6);
    /// assert_eq!(Class::Warrior.starting_hp(), 6);
    /// ```
    pub fn starting_hp(&self) -> u8 {
        6 // All classes start with 6 HP
    }

    /// Get the starting evasion for this class
    ///
    /// Different classes have different base evasion values.
    /// These are placeholder values until exact SRD values are available.
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::Class;
    ///
    /// let evasion = Class::Rogue.starting_evasion();
    /// assert!(evasion > 0);
    /// ```
    pub fn starting_evasion(&self) -> u8 {
        // TODO: Replace with exact SRD values when available
        // These are reasonable placeholder values based on class archetype
        match self {
            Class::Rogue => 14,    // High evasion (agile, dodgy)
            Class::Ranger => 13,   // High evasion (agile)
            Class::Bard => 12,     // Medium-high (nimble)
            Class::Druid => 11,    // Medium (nature magic)
            Class::Wizard => 11,   // Medium (unarmored mage)
            Class::Sorcerer => 11, // Medium (unarmored mage)
            Class::Seraph => 11,   // Medium (divine protection)
            Class::Warrior => 10,  // Medium-low (armored fighter)
            Class::Guardian => 10, // Lower (heavy armor, tank)
        }
    }

    /// Check if this class can use abilities from a specific domain
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::{Class, Domain};
    ///
    /// assert!(Class::Bard.can_use_domain(Domain::Codex));
    /// assert!(Class::Bard.can_use_domain(Domain::Grace));
    /// assert!(!Class::Bard.can_use_domain(Domain::Blade));
    /// ```
    pub fn can_use_domain(&self, domain: Domain) -> bool {
        let (d1, d2) = self.domains();
        domain == d1 || domain == d2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_class_count() {
        assert_eq!(Class::iter().count(), 9, "Should have exactly 9 classes");
    }

    #[test]
    fn test_domain_count() {
        assert_eq!(Domain::iter().count(), 9, "Should have exactly 9 domains");
    }

    #[test]
    fn test_all_classes_have_starting_hp_of_6() {
        for class in Class::iter() {
            assert_eq!(class.starting_hp(), 6, "{} should start with 6 HP", class);
        }
    }

    #[test]
    fn test_class_domains_are_unique() {
        for class in Class::iter() {
            let (domain1, domain2) = class.domains();
            assert_ne!(
                domain1, domain2,
                "{} should have two different domains",
                class
            );
        }
    }

    #[test]
    fn test_bard_domains() {
        assert_eq!(Class::Bard.domains(), (Domain::Codex, Domain::Grace));
    }

    #[test]
    fn test_warrior_domains() {
        assert_eq!(Class::Warrior.domains(), (Domain::Blade, Domain::Bone));
    }

    #[test]
    fn test_class_evasion_is_nonzero() {
        for class in Class::iter() {
            let evasion = class.starting_evasion();
            assert!(
                evasion > 0 && evasion <= 20,
                "{} evasion {} should be between 1-20",
                class,
                evasion
            );
        }
    }

    #[test]
    fn test_class_display() {
        assert_eq!(Class::Bard.to_string(), "Bard");
        assert_eq!(Class::Guardian.to_string(), "Guardian");
    }

    #[test]
    fn test_domain_display() {
        assert_eq!(Domain::Arcana.to_string(), "Arcana");
        assert_eq!(Domain::Valor.to_string(), "Valor");
    }

    #[test]
    fn test_class_can_use_domain() {
        let class = Class::Bard;
        assert!(class.can_use_domain(Domain::Codex));
        assert!(class.can_use_domain(Domain::Grace));
        assert!(!class.can_use_domain(Domain::Blade));
    }

    #[test]
    fn test_all_classes_serializable() {
        for class in Class::iter() {
            let json = serde_json::to_string(&class).unwrap();
            let deserialized: Class = serde_json::from_str(&json).unwrap();
            assert_eq!(class, deserialized);
        }
    }

    #[test]
    fn test_all_domains_serializable() {
        for domain in Domain::iter() {
            let json = serde_json::to_string(&domain).unwrap();
            let deserialized: Domain = serde_json::from_str(&json).unwrap();
            assert_eq!(domain, deserialized);
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use strum::IntoEnumIterator;

    fn any_class() -> impl Strategy<Value = Class> {
        let classes: Vec<Class> = Class::iter().collect();
        let class_indices = 0..classes.len();
        class_indices.prop_map(move |i| classes[i])
    }

    fn any_domain() -> impl Strategy<Value = Domain> {
        let domains: Vec<Domain> = Domain::iter().collect();
        let domain_indices = 0..domains.len();
        domain_indices.prop_map(move |i| domains[i])
    }

    proptest! {
        #[test]
        fn prop_class_hp_always_6(class in any_class()) {
            prop_assert_eq!(class.starting_hp(), 6);
        }

        #[test]
        fn prop_class_domains_are_consistent(class in any_class()) {
            let domains1 = class.domains();
            let domains2 = class.domains();
            prop_assert_eq!(domains1, domains2, "domains() should be deterministic");
        }

        #[test]
        fn prop_can_use_domain_matches_domains(class in any_class(), domain in any_domain()) {
            let (d1, d2) = class.domains();
            let should_be_able = domain == d1 || domain == d2;
            prop_assert_eq!(class.can_use_domain(domain), should_be_able);
        }

        #[test]
        fn prop_evasion_is_consistent(class in any_class()) {
            let evasion1 = class.starting_evasion();
            let evasion2 = class.starting_evasion();
            prop_assert_eq!(evasion1, evasion2, "starting_evasion() should be deterministic");
        }
    }
}
