//! Character ancestries (races)

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// The 17 playable ancestries in Daggerheart
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display, Serialize, Deserialize)]
pub enum Ancestry {
    Clank,
    Daemon,
    Drakona,
    Dwarf,
    Faerie,
    Faun,
    Fungril,
    Galapa,
    Giant,
    Goblin,
    Halfling,
    Human,
    Inferis,
    Katari,
    Orc,
    Ribbet,
    Simiah,
}

impl Ancestry {
    /// Get the HP modifier for this ancestry
    ///
    /// Most ancestries use the standard 6 HP (modifier = 0).
    /// Giants start with 7 HP (modifier = +1).
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::Ancestry;
    ///
    /// assert_eq!(Ancestry::Human.hp_modifier(), 0);
    /// assert_eq!(Ancestry::Giant.hp_modifier(), 1);
    /// ```
    pub fn hp_modifier(&self) -> i8 {
        match self {
            Ancestry::Giant => 1, // Giants start with 7 HP
            _ => 0,               // All others: standard 6 HP
        }
    }

    /// Get the Evasion modifier for this ancestry
    ///
    /// Most ancestries have no evasion bonus (modifier = 0).
    /// Simiah get +1 Evasion from their nimble nature.
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::Ancestry;
    ///
    /// assert_eq!(Ancestry::Human.evasion_modifier(), 0);
    /// assert_eq!(Ancestry::Simiah.evasion_modifier(), 1);
    /// ```
    pub fn evasion_modifier(&self) -> i8 {
        match self {
            Ancestry::Simiah => 1, // Simiah are nimble (+1 Evasion)
            _ => 0,
        }
    }

    /// Check if this ancestry has natural flight ability
    ///
    /// Faeries have innate flight. Other ancestries may gain flight
    /// through magic items or abilities, but it's not innate.
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::Ancestry;
    ///
    /// assert!(Ancestry::Faerie.has_flight());
    /// assert!(!Ancestry::Human.has_flight());
    /// ```
    pub fn has_flight(&self) -> bool {
        matches!(self, Ancestry::Faerie)
    }

    /// Get the foundation abilities for this ancestry
    ///
    /// Each ancestry provides unique foundation abilities that grant
    /// special powers and traits. This is a simplified representation;
    /// full ability implementations will be added later.
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::character::Ancestry;
    ///
    /// let abilities = Ancestry::Human.foundation_abilities();
    /// assert!(abilities.contains(&"Adaptable"));
    ///
    /// let abilities = Ancestry::Giant.foundation_abilities();
    /// assert!(abilities.contains(&"Mighty Grip"));
    /// ```
    pub fn foundation_abilities(&self) -> Vec<&'static str> {
        match self {
            Ancestry::Clank => vec!["Constructed", "Repair Protocol"],
            Ancestry::Daemon => vec!["Demon Ancestry", "Otherworldly"],
            Ancestry::Drakona => vec!["Dragon Ancestry", "Breath Weapon"],
            Ancestry::Dwarf => vec!["Stonecunning", "Dwarven Resilience"],
            Ancestry::Faerie => vec!["Flight", "Fey Magic"],
            Ancestry::Faun => vec!["Natural Athlete", "Forest Step"],
            Ancestry::Fungril => vec!["Spore Cloud", "Fungal Network"],
            Ancestry::Galapa => vec!["Shell Defense", "Aquatic"],
            Ancestry::Giant => vec!["Mighty Grip", "Imposing Presence"],
            Ancestry::Goblin => vec!["Nimble Escape", "Sneaky"],
            Ancestry::Halfling => vec!["Lucky", "Brave"],
            Ancestry::Human => vec!["Adaptable", "Versatile"],
            Ancestry::Inferis => vec!["Fire Resistance", "Infernal Legacy"],
            Ancestry::Katari => vec!["Cat's Grace", "Nine Lives"],
            Ancestry::Orc => vec!["Relentless Endurance", "Savage Attacks"],
            Ancestry::Ribbet => vec!["Amphibious", "Leap"],
            Ancestry::Simiah => vec!["Prehensile Tail", "Climbing"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_ancestry_count() {
        assert_eq!(
            Ancestry::iter().count(),
            17,
            "Should have exactly 17 ancestries"
        );
    }

    #[test]
    fn test_ancestry_display() {
        assert_eq!(Ancestry::Human.to_string(), "Human");
        assert_eq!(Ancestry::Giant.to_string(), "Giant");
        assert_eq!(Ancestry::Simiah.to_string(), "Simiah");
    }

    #[test]
    fn test_giant_hp_modifier() {
        // Giants start with 7 HP instead of 6
        assert_eq!(Ancestry::Giant.hp_modifier(), 1);
    }

    #[test]
    fn test_most_ancestries_no_hp_modifier() {
        // Most ancestries use standard 6 HP (modifier = 0)
        assert_eq!(Ancestry::Human.hp_modifier(), 0);
        assert_eq!(Ancestry::Dwarf.hp_modifier(), 0);
        assert_eq!(Ancestry::Halfling.hp_modifier(), 0);
    }

    #[test]
    fn test_simiah_evasion_bonus() {
        // Simiah get +1 Evasion
        assert_eq!(Ancestry::Simiah.evasion_modifier(), 1);
    }

    #[test]
    fn test_most_ancestries_no_evasion_modifier() {
        // Most ancestries have no evasion modifier
        assert_eq!(Ancestry::Human.evasion_modifier(), 0);
        assert_eq!(Ancestry::Dwarf.evasion_modifier(), 0);
        assert_eq!(Ancestry::Giant.evasion_modifier(), 0);
    }

    #[test]
    fn test_faerie_flight_evasion_bonus() {
        // Faerie flight grants +2 Evasion (armor property, not ancestry trait)
        // But we can track if ancestry enables flight
        assert!(Ancestry::Faerie.has_flight());
        assert!(!Ancestry::Human.has_flight());
    }

    #[test]
    fn test_all_ancestries_have_foundation_ability() {
        // Every ancestry should have at least one foundation ability
        for ancestry in Ancestry::iter() {
            let abilities = ancestry.foundation_abilities();
            assert!(
                !abilities.is_empty(),
                "{} should have foundation abilities",
                ancestry
            );
        }
    }

    #[test]
    fn test_human_foundation_abilities() {
        let abilities = Ancestry::Human.foundation_abilities();
        assert!(
            abilities.contains(&"Adaptable"),
            "Humans should have Adaptable trait"
        );
    }

    #[test]
    fn test_giant_foundation_abilities() {
        let abilities = Ancestry::Giant.foundation_abilities();
        assert!(
            abilities.contains(&"Mighty Grip"),
            "Giants should have Mighty Grip"
        );
    }

    #[test]
    fn test_all_ancestries_serializable() {
        for ancestry in Ancestry::iter() {
            let json = serde_json::to_string(&ancestry).unwrap();
            let deserialized: Ancestry = serde_json::from_str(&json).unwrap();
            assert_eq!(ancestry, deserialized);
        }
    }

    #[test]
    fn test_hp_modifiers_are_reasonable() {
        // HP modifiers should be small (typically -1 to +2)
        for ancestry in Ancestry::iter() {
            let modifier = ancestry.hp_modifier();
            assert!(
                (-1..=2).contains(&modifier),
                "{} HP modifier {} should be between -1 and +2",
                ancestry,
                modifier
            );
        }
    }

    #[test]
    fn test_evasion_modifiers_are_reasonable() {
        // Evasion modifiers should be small (typically 0 to +2)
        for ancestry in Ancestry::iter() {
            let modifier = ancestry.evasion_modifier();
            assert!(
                (0..=2).contains(&modifier),
                "{} Evasion modifier {} should be between 0 and +2",
                ancestry,
                modifier
            );
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use strum::IntoEnumIterator;

    fn any_ancestry() -> impl Strategy<Value = Ancestry> {
        let ancestries: Vec<Ancestry> = Ancestry::iter().collect();
        let ancestry_indices = 0..ancestries.len();
        ancestry_indices.prop_map(move |i| ancestries[i])
    }

    proptest! {
        #[test]
        fn prop_hp_modifier_is_consistent(ancestry in any_ancestry()) {
            let hp1 = ancestry.hp_modifier();
            let hp2 = ancestry.hp_modifier();
            prop_assert_eq!(hp1, hp2, "hp_modifier() should be deterministic");
        }

        #[test]
        fn prop_evasion_modifier_is_consistent(ancestry in any_ancestry()) {
            let ev1 = ancestry.evasion_modifier();
            let ev2 = ancestry.evasion_modifier();
            prop_assert_eq!(ev1, ev2, "evasion_modifier() should be deterministic");
        }

        #[test]
        fn prop_foundation_abilities_is_consistent(ancestry in any_ancestry()) {
            let abilities1 = ancestry.foundation_abilities();
            let abilities2 = ancestry.foundation_abilities();
            prop_assert_eq!(abilities1, abilities2, "foundation_abilities() should be deterministic");
        }

        #[test]
        fn prop_has_flight_is_consistent(ancestry in any_ancestry()) {
            let flight1 = ancestry.has_flight();
            let flight2 = ancestry.has_flight();
            prop_assert_eq!(flight1, flight2, "has_flight() should be deterministic");
        }
    }
}
