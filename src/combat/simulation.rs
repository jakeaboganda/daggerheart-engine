//! Combat simulation - full combat encounter management

use crate::character::{Ancestry, Attributes, Class};
use crate::combat::{Fear, HitPoints, Hope, Stress};
use crate::core::dice::DualityRoll;
use serde::{Deserialize, Serialize};

/// A combatant in an encounter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combatant {
    pub name: String,
    pub level: u8,
    pub class: Class,
    pub ancestry: Ancestry,
    pub attributes: Attributes,
    pub hp: HitPoints,
    pub stress: Stress,
    pub evasion: u8,
    pub armor: u8,
    pub initiative: u8,
    pub is_player: bool,
}

impl Combatant {
    /// Save combatant to a JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use daggerheart_engine::combat::simulation::Combatant;
    /// use daggerheart_engine::character::{Class, Ancestry, Attributes};
    ///
    /// let warrior = Combatant::player(
    ///     "Grom",
    ///     1,
    ///     Class::Warrior,
    ///     Ancestry::Orc,
    ///     Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
    /// );
    ///
    /// warrior.save_to_file("grom.json").unwrap();
    /// ```
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load combatant from a JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use daggerheart_engine::combat::simulation::Combatant;
    ///
    /// let warrior = Combatant::load_from_file("grom.json").unwrap();
    /// println!("Loaded: {}", warrior.name);
    /// ```
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Create a new player character combatant
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::simulation::Combatant;
    /// use daggerheart_engine::character::{Class, Ancestry, Attributes};
    ///
    /// let warrior = Combatant::player(
    ///     "Grom",
    ///     1,
    ///     Class::Warrior,
    ///     Ancestry::Orc,
    ///     Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
    /// );
    ///
    /// assert_eq!(warrior.name, "Grom");
    /// assert!(warrior.is_player);
    /// ```
    pub fn player(
        name: impl Into<String>,
        level: u8,
        class: Class,
        ancestry: Ancestry,
        attributes: Attributes,
    ) -> Self {
        let base_hp = class.starting_hp();
        let hp_total = (base_hp as i16 + ancestry.hp_modifier() as i16).max(1) as u8;

        let base_evasion = class.starting_evasion();
        let evasion_total = (base_evasion as i16 + ancestry.evasion_modifier() as i16).max(1) as u8;

        Self {
            name: name.into(),
            level,
            class,
            ancestry,
            attributes,
            hp: HitPoints::new(hp_total),
            stress: Stress::new(),
            evasion: evasion_total,
            armor: 0, // Can be set later with equipment
            initiative: 0,
            is_player: true,
        }
    }

    /// Create a new enemy combatant
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::simulation::Combatant;
    ///
    /// let goblin = Combatant::enemy("Goblin Scout", 1, 4, 13, 1);
    ///
    /// assert_eq!(goblin.name, "Goblin Scout");
    /// assert!(!goblin.is_player);
    /// ```
    pub fn enemy(name: impl Into<String>, level: u8, hp: u8, evasion: u8, armor: u8) -> Self {
        // For enemies, use valid placeholder stats
        Self {
            name: name.into(),
            level,
            class: Class::Rogue, // Placeholder - enemies don't need real classes
            ancestry: Ancestry::Goblin, // Placeholder
            attributes: Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
            hp: HitPoints::new(hp),
            stress: Stress::new(),
            evasion,
            armor,
            initiative: 0,
            is_player: false,
        }
    }

    /// Set armor value
    pub fn with_armor(mut self, armor: u8) -> Self {
        self.armor = armor;
        self
    }

    /// Roll initiative
    pub fn roll_initiative(&mut self) {
        let roll = DualityRoll::roll();
        let result = roll.with_modifier(0); // Could add Agility modifier
        self.initiative = (result.total % 20) as u8; // Cap at 20
    }

    /// Check if combatant is alive
    pub fn is_alive(&self) -> bool {
        self.hp.is_alive()
    }

    /// Take damage
    pub fn take_damage(&mut self, amount: u8) {
        self.hp.take_damage(amount);
    }

    /// Gain stress
    pub fn gain_stress(&mut self, amount: u8) {
        self.stress.gain(amount);
    }
}

/// Combat encounter state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEncounter {
    pub combatants: Vec<Combatant>,
    pub round: u32,
    pub turn_order: Vec<usize>,
    pub current_turn: usize,
    pub hope: Hope,
    pub fear: Fear,
}

impl CombatEncounter {
    /// Save combat encounter to a JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use daggerheart_engine::combat::simulation::CombatEncounter;
    ///
    /// let encounter = CombatEncounter::new(5);
    /// encounter.save_session("encounter.json").unwrap();
    /// ```
    pub fn save_session(&self, path: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load combat encounter from a JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use daggerheart_engine::combat::simulation::CombatEncounter;
    ///
    /// let encounter = CombatEncounter::load_session("encounter.json").unwrap();
    /// println!("Round: {}", encounter.round);
    /// ```
    pub fn load_session(path: &str) -> Result<Self, std::io::Error> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Create a new combat encounter
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::simulation::CombatEncounter;
    ///
    /// let encounter = CombatEncounter::new(5); // Party Hope max = 5
    /// assert_eq!(encounter.round, 0);
    /// assert_eq!(encounter.combatants.len(), 0);
    /// ```
    pub fn new(hope_max: u8) -> Self {
        Self {
            combatants: Vec::new(),
            round: 0,
            turn_order: Vec::new(),
            current_turn: 0,
            hope: Hope::new(hope_max),
            fear: Fear::new(),
        }
    }

    /// Add a combatant to the encounter
    ///
    /// # Examples
    ///
    /// ```
    /// use daggerheart_engine::combat::simulation::{CombatEncounter, Combatant};
    /// use daggerheart_engine::character::{Class, Ancestry, Attributes};
    ///
    /// let mut encounter = CombatEncounter::new(5);
    /// let warrior = Combatant::player(
    ///     "Grom",
    ///     1,
    ///     Class::Warrior,
    ///     Ancestry::Orc,
    ///     Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
    /// );
    ///
    /// encounter.add_combatant(warrior);
    /// assert_eq!(encounter.combatants.len(), 1);
    /// ```
    pub fn add_combatant(&mut self, combatant: Combatant) {
        self.combatants.push(combatant);
    }

    /// Start the encounter (roll initiative for all combatants)
    pub fn start(&mut self) {
        // Roll initiative for all combatants
        for combatant in &mut self.combatants {
            combatant.roll_initiative();
        }

        // Sort by initiative (highest first)
        let mut indices: Vec<usize> = (0..self.combatants.len()).collect();
        indices.sort_by(|&a, &b| {
            self.combatants[b]
                .initiative
                .cmp(&self.combatants[a].initiative)
        });
        self.turn_order = indices;

        // Start round 1
        self.round = 1;
        self.current_turn = 0;
    }

    /// Get the current combatant's index
    pub fn current_combatant_index(&self) -> Option<usize> {
        if self.current_turn < self.turn_order.len() {
            Some(self.turn_order[self.current_turn])
        } else {
            None
        }
    }

    /// Get the current combatant
    pub fn current_combatant(&self) -> Option<&Combatant> {
        self.current_combatant_index()
            .and_then(|idx| self.combatants.get(idx))
    }

    /// Get a mutable reference to the current combatant
    pub fn current_combatant_mut(&mut self) -> Option<&mut Combatant> {
        self.current_combatant_index()
            .and_then(|idx| self.combatants.get_mut(idx))
    }

    /// Advance to the next turn
    pub fn next_turn(&mut self) {
        self.current_turn += 1;

        // If we've gone through all combatants, start new round
        if self.current_turn >= self.turn_order.len() {
            self.round += 1;
            self.current_turn = 0;

            // Remove dead combatants from turn order
            self.turn_order
                .retain(|&idx| self.combatants[idx].is_alive());
        }
    }

    /// Check if combat is over
    pub fn is_over(&self) -> bool {
        // Combat is over if all players are dead or all enemies are dead
        let players_alive = self.combatants.iter().any(|c| c.is_player && c.is_alive());
        let enemies_alive = self.combatants.iter().any(|c| !c.is_player && c.is_alive());

        !players_alive || !enemies_alive
    }

    /// Get victory status (true = players won, false = enemies won)
    pub fn player_victory(&self) -> Option<bool> {
        if !self.is_over() {
            return None;
        }

        let players_alive = self.combatants.iter().any(|c| c.is_player && c.is_alive());

        Some(players_alive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_warrior() -> Combatant {
        Combatant::player(
            "Test Warrior",
            1,
            Class::Warrior,
            Ancestry::Human,
            Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
        )
        .with_armor(3)
    }

    fn create_test_goblin() -> Combatant {
        Combatant::enemy("Goblin", 1, 4, 13, 1)
    }

    #[test]
    fn test_create_combatant() {
        let warrior = create_test_warrior();

        assert_eq!(warrior.name, "Test Warrior");
        assert_eq!(warrior.level, 1);
        assert_eq!(warrior.class, Class::Warrior);
        assert!(warrior.is_player);
        assert!(warrior.is_alive());
    }

    #[test]
    fn test_combatant_take_damage() {
        let mut warrior = create_test_warrior();

        warrior.take_damage(2);
        assert_eq!(warrior.hp.current, 4);
        assert!(warrior.is_alive());

        warrior.take_damage(10);
        assert_eq!(warrior.hp.current, 0);
        assert!(!warrior.is_alive());
    }

    #[test]
    fn test_combatant_gain_stress() {
        let mut warrior = create_test_warrior();

        warrior.gain_stress(3);
        assert_eq!(warrior.stress.current, 3);
    }

    #[test]
    fn test_create_encounter() {
        let encounter = CombatEncounter::new(5);

        assert_eq!(encounter.round, 0);
        assert_eq!(encounter.combatants.len(), 0);
        assert_eq!(encounter.hope.maximum, 5);
    }

    #[test]
    fn test_add_combatant() {
        let mut encounter = CombatEncounter::new(5);
        let warrior = create_test_warrior();

        encounter.add_combatant(warrior);
        assert_eq!(encounter.combatants.len(), 1);
    }

    #[test]
    fn test_start_encounter() {
        let mut encounter = CombatEncounter::new(5);
        encounter.add_combatant(create_test_warrior());
        encounter.add_combatant(create_test_goblin());

        encounter.start();

        assert_eq!(encounter.round, 1);
        assert_eq!(encounter.turn_order.len(), 2);
        assert_eq!(encounter.current_turn, 0);

        // Initiative should be rolled (non-zero)
        assert!(encounter.combatants[0].initiative < 20);
        assert!(encounter.combatants[1].initiative < 20);
    }

    #[test]
    fn test_current_combatant() {
        let mut encounter = CombatEncounter::new(5);
        encounter.add_combatant(create_test_warrior());
        encounter.add_combatant(create_test_goblin());
        encounter.start();

        let current = encounter.current_combatant();
        assert!(current.is_some());
    }

    #[test]
    fn test_next_turn() {
        let mut encounter = CombatEncounter::new(5);
        encounter.add_combatant(create_test_warrior());
        encounter.add_combatant(create_test_goblin());
        encounter.start();

        assert_eq!(encounter.current_turn, 0);

        encounter.next_turn();
        assert_eq!(encounter.current_turn, 1);

        encounter.next_turn();
        assert_eq!(encounter.current_turn, 0); // Back to first combatant
        assert_eq!(encounter.round, 2); // New round
    }

    #[test]
    fn test_combat_not_over_initially() {
        let mut encounter = CombatEncounter::new(5);
        encounter.add_combatant(create_test_warrior());
        encounter.add_combatant(create_test_goblin());
        encounter.start();

        assert!(!encounter.is_over());
        assert!(encounter.player_victory().is_none());
    }

    #[test]
    fn test_combat_over_when_all_enemies_dead() {
        let mut encounter = CombatEncounter::new(5);
        encounter.add_combatant(create_test_warrior());
        let mut goblin = create_test_goblin();
        goblin.take_damage(10); // Kill the goblin
        encounter.add_combatant(goblin);
        encounter.start();

        assert!(encounter.is_over());
        assert_eq!(encounter.player_victory(), Some(true));
    }

    #[test]
    fn test_combat_over_when_all_players_dead() {
        let mut encounter = CombatEncounter::new(5);
        let mut warrior = create_test_warrior();
        warrior.take_damage(10); // Kill the warrior
        encounter.add_combatant(warrior);
        encounter.add_combatant(create_test_goblin());
        encounter.start();

        assert!(encounter.is_over());
        assert_eq!(encounter.player_victory(), Some(false));
    }

    #[test]
    fn test_encounter_serialization() {
        let mut encounter = CombatEncounter::new(5);
        encounter.add_combatant(create_test_warrior());

        let json = serde_json::to_string(&encounter).unwrap();
        let loaded: CombatEncounter = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.combatants.len(), encounter.combatants.len());
        assert_eq!(loaded.hope.maximum, encounter.hope.maximum);
    }
}
