# API Guide

This guide explains how to use the Daggerheart engine as a Rust library in your own projects.

**Audience:** Developers integrating the engine into apps, games, or tools  
**Prerequisites:** Basic Rust knowledge

---

## Table of Contents

1. [Setup](#setup)
2. [Core Dice Module](#core-dice-module)
3. [Character Module](#character-module)
4. [Combat Module](#combat-module)
5. [Cards Module](#cards-module)
6. [Error Handling](#error-handling)
7. [Serialization](#serialization)
8. [Common Patterns](#common-patterns)
9. [Advanced Usage](#advanced-usage)

---

## Setup

### Add as Dependency

**Cargo.toml:**
```toml
[dependencies]
daggerheart-engine = { git = "https://github.com/jakeaboganda/daggerheart-engine" }

# Or local path:
daggerheart-engine = { path = "../daggerheart-engine" }
```

### Import in Code

```rust
// Import everything
use daggerheart_engine::prelude::*;

// Or import specific modules
use daggerheart_engine::core::dice::{Die, DualityRoll};
use daggerheart_engine::character::{Class, Ancestry, Attributes};
use daggerheart_engine::combat::simulation::{Combatant, CombatEncounter};
```

---

## Core Dice Module

Located at: `daggerheart_engine::core::dice`

### Basic Dice

```rust
use daggerheart_engine::core::dice::Die;

// Create and roll a die
let d20 = Die::D20;
let result = d20.roll();  // 1-20

// Available dice: D4, D6, D8, D10, D12, D20
let d6 = Die::D6;
let d8 = Die::D8;
```

**API:**
```rust
impl Die {
    pub fn roll(&self) -> u8;
    pub fn max_value(&self) -> u8;
}
```

### Duality Dice (2d12 Hope vs Fear)

```rust
use daggerheart_engine::core::dice::{DualityRoll, ControllingDie};

// Basic roll
let roll = DualityRoll::roll();
println!("Hope: {}, Fear: {}", roll.hope, roll.fear);

// With modifier
let result = roll.with_modifier(3);
println!("Total: {}", result.total);

// Check critical (doubles)
if result.is_critical {
    println!("CRITICAL!");
}

// Check which die controls
match result.controlling {
    ControllingDie::Hope => println!("Hope wins!"),
    ControllingDie::Fear => println!("Fear wins..."),
    ControllingDie::Tied => println!("Tied"),
}
```

**API:**
```rust
pub struct DualityRoll {
    pub hope: u8,  // 1-12
    pub fear: u8,  // 1-12
}

impl DualityRoll {
    pub fn roll() -> Self;
    pub fn from_values(hope: u8, fear: u8) -> Self;
    pub fn is_critical(&self) -> bool;
    pub fn controlling_die(&self) -> ControllingDie;
    pub fn with_modifier(self, modifier: i8) -> DualityResult;
    pub fn with_advantage(self) -> DualityResult;
}

pub struct DualityResult {
    pub roll: DualityRoll,
    pub modifier: i8,
    pub advantage_die: Option<u8>,  // d6 if advantage
    pub total: u16,
    pub controlling: ControllingDie,
    pub is_critical: bool,
}
```

### Damage Dice

```rust
use daggerheart_engine::core::dice::{DamageDice, Die};

// Simple damage
let damage = DamageDice::d8(2)  // 2d8
    .with_bonus(3);

let result = damage.roll();
println!("Rolls: {:?}", result.rolls);
println!("Total damage: {}", result.total);

// Complex damage
let damage = DamageDice::new(vec![Die::D8, Die::D8, Die::D6])
    .with_bonus(5);
```

**API:**
```rust
impl DamageDice {
    pub fn new(dice: Vec<Die>) -> Self;
    pub fn with_bonus(self, bonus: i16) -> Self;
    pub fn roll(&self) -> DamageRoll;
    
    // Convenience constructors
    pub fn d4(count: usize) -> Self;
    pub fn d6(count: usize) -> Self;
    pub fn d8(count: usize) -> Self;
    pub fn d10(count: usize) -> Self;
    pub fn d12(count: usize) -> Self;
    pub fn d20(count: usize) -> Self;
}

pub struct DamageRoll {
    pub rolls: Vec<u8>,
    pub bonus: i16,
    pub total: u16,
}
```

---

## Character Module

Located at: `daggerheart_engine::character`

### Attributes

```rust
use daggerheart_engine::character::Attributes;

// Create with standard modifiers [+2, +1, +1, +0, +0, -1]
let attrs = Attributes::from_array([2, 1, 1, 0, 0, -1])?;

// Access individual attributes
println!("Agility: {:+}", attrs.agility);
println!("Strength: {:+}", attrs.strength);
```

**API:**
```rust
pub struct Attributes {
    pub agility: i8,
    pub strength: i8,
    pub finesse: i8,
    pub instinct: i8,
    pub presence: i8,
    pub knowledge: i8,
}

impl Attributes {
    pub fn from_array(values: [i8; 6]) -> Result<Self, EngineError>;
}
```

**Validation:** Attributes must use exactly `[-1, 0, 0, 1, 1, 2]` in any order.

### Classes

```rust
use daggerheart_engine::character::Class;

let class = Class::Warrior;

// Get starting stats
let hp = class.starting_hp();       // 6
let evasion = class.starting_evasion();  // 10

// Get domains
let (d1, d2) = class.domains();
// Warrior: (Blade, Bone)
```

**Available Classes:**
```rust
pub enum Class {
    Bard,      // Sage, Codex
    Druid,     // Arcana, Sage
    Guardian,  // Valor, Blade
    Ranger,    // Sage, Grace
    Rogue,     // Grace, Codex
    Seraph,    // Splendor, Valor
    Sorcerer,  // Arcana, Midnight
    Warrior,   // Blade, Bone
    Wizard,    // Codex, Splendor
}
```

### Ancestries

```rust
use daggerheart_engine::character::Ancestry;

let ancestry = Ancestry::Orc;

// Get modifiers
let hp_mod = ancestry.hp_modifier();       // 0
let evasion_mod = ancestry.evasion_modifier();  // -1
```

**Available Ancestries:**
```rust
pub enum Ancestry {
    Clank, Daemon, Drakona, Dwarf, Faerie, Faun,
    Fungril, Galapa, Giant, Goblin, Halfling, Human,
    Inferis, Katari, Orc, Ribbet, Simiah,
}
```

### Character Progression

```rust
use daggerheart_engine::character::CharacterProgress;

let mut progress = CharacterProgress::new();

// Add experience
progress.add_experience(120);

// Check if can level up
if progress.can_level_up() {
    let old_level = progress.level;
    progress.level_up()?;
    println!("Level up: {} → {}", old_level, progress.level);
    
    // Learn a card
    progress.add_card("blade_strike");
}

// XP formula: level × 100
let xp_needed = progress.xp_for_next_level();

// Check for cards
if progress.has_card("blade_strike") {
    println!("Has blade strike!");
}
```

**API:**
```rust
pub struct CharacterProgress {
    pub level: u8,           // 1-10
    pub experience: u32,
    pub available_cards: Vec<String>,
}

impl CharacterProgress {
    pub fn new() -> Self;
    pub fn xp_for_next_level(&self) -> u32;
    pub fn add_experience(&mut self, amount: u32);
    pub fn can_level_up(&self) -> bool;
    pub fn level_up(&mut self) -> Result<(), EngineError>;
    pub fn add_card(&mut self, card_id: impl Into<String>);
    pub fn has_card(&self, card_id: &str) -> bool;
    
    // Save/Load
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error>;
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error>;
}
```

---

## Combat Module

Located at: `daggerheart_engine::combat`

### Creating Combatants

```rust
use daggerheart_engine::combat::simulation::Combatant;
use daggerheart_engine::character::{Class, Ancestry, Attributes};

// Create a player character
let warrior = Combatant::player(
    "Grom",
    5,  // level
    Class::Warrior,
    Ancestry::Orc,
    Attributes::from_array([2, 1, 1, 0, 0, -1])?,
).with_armor(3);

// Create an enemy
let goblin = Combatant::enemy(
    "Goblin Scout",
    1,   // level
    4,   // hp
    13,  // evasion
    1,   // armor
);

// Check if alive
if warrior.is_alive() {
    println!("Warrior is alive!");
}

// Take damage
let mut warrior = warrior;
warrior.take_damage(3);
println!("HP: {}/{}", warrior.hp.current, warrior.hp.maximum);
```

**API:**
```rust
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
    pub fn player(...) -> Self;
    pub fn enemy(...) -> Self;
    pub fn with_armor(self, armor: u8) -> Self;
    pub fn roll_initiative(&mut self);
    pub fn is_alive(&self) -> bool;
    pub fn take_damage(&mut self, amount: u8);
    pub fn gain_stress(&mut self, amount: u8);
    
    // Save/Load
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error>;
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error>;
}
```

### Combat Encounters

```rust
use daggerheart_engine::combat::simulation::CombatEncounter;

// Create encounter
let mut encounter = CombatEncounter::new(5);  // Hope pool max = 5

// Add combatants
encounter.add_combatant(warrior);
encounter.add_combatant(goblin1);
encounter.add_combatant(goblin2);

// Start combat (roll initiative)
encounter.start();

println!("Round: {}", encounter.round);
println!("Turn order: {:?}", encounter.turn_order);

// Combat loop
while !encounter.is_over() {
    // Get current combatant
    if let Some(current) = encounter.current_combatant() {
        println!("Current turn: {}", current.name);
        
        // Take actions...
        
        // Next turn
        encounter.next_turn();
    }
}

// Check victory
if let Some(victory) = encounter.player_victory() {
    if victory {
        println!("Players won!");
    } else {
        println!("Enemies won!");
    }
}
```

**API:**
```rust
pub struct CombatEncounter {
    pub combatants: Vec<Combatant>,
    pub round: u32,
    pub turn_order: Vec<usize>,  // Indices into combatants
    pub current_turn: usize,
    pub hope: Hope,
    pub fear: Fear,
}

impl CombatEncounter {
    pub fn new(hope_max: u8) -> Self;
    pub fn add_combatant(&mut self, combatant: Combatant);
    pub fn start(&mut self);  // Roll initiative
    pub fn current_combatant(&self) -> Option<&Combatant>;
    pub fn current_combatant_mut(&mut self) -> Option<&mut Combatant>;
    pub fn next_turn(&mut self);
    pub fn is_over(&self) -> bool;
    pub fn player_victory(&self) -> Option<bool>;
    
    // Save/Load
    pub fn save_session(&self, path: &str) -> Result<(), std::io::Error>;
    pub fn load_session(path: &str) -> Result<Self, std::io::Error>;
}
```

---

## Error Handling

```rust
use daggerheart_engine::error::EngineError;

fn example() -> Result<(), EngineError> {
    // Operations that can fail
    let attrs = Attributes::from_array([3, 1, 1, 0, 0, -1])?;  // Error: invalid
    
    let mut progress = CharacterProgress::new();
    progress.level_up()?;  // Error: not enough XP
    
    Ok(())
}

// Handle errors
match example() {
    Ok(_) => println!("Success!"),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Error Types:**
```rust
pub enum EngineError {
    InvalidDiceRoll(String),
    InvalidAttributeArray(String),
    InvalidCharacterState(String),
    InvalidCardRequirement(String),
}
```

---

## Serialization

All main types implement `Serialize` and `Deserialize`:

```rust
use serde_json;

// Save to JSON
let json = serde_json::to_string_pretty(&warrior)?;
std::fs::write("warrior.json", json)?;

// Load from JSON
let json = std::fs::read_to_string("warrior.json")?;
let warrior: Combatant = serde_json::from_str(&json)?;

// Or use convenience methods
warrior.save_to_file("warrior.json")?;
let warrior = Combatant::load_from_file("warrior.json")?;
```

See [SAVE_FORMAT.md](SAVE_FORMAT.md) for JSON schema details.

---

## Common Patterns

### Pattern 1: Attack Resolution

```rust
use daggerheart_engine::combat::attack::Attack;

let attack = Attack::new(warrior.attributes.strength)
    .with_proficiency(1);

let attack_result = attack.roll();

if attack_result.total >= goblin.evasion {
    // Hit! Roll damage
    let damage = DamageDice::d10(1).with_bonus(3).roll();
    goblin.take_damage(damage.total as u8);
    
    println!("Hit for {} damage!", damage.total);
} else {
    println!("Miss!");
}
```

### Pattern 2: Character Builder

```rust
fn create_warrior(name: &str, level: u8) -> Combatant {
    Combatant::player(
        name,
        level,
        Class::Warrior,
        Ancestry::Human,
        Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
    ).with_armor(3)
}

let hero = create_warrior("Grom", 5);
```

### Pattern 3: Save Game State

```rust
struct GameState {
    party: Vec<Combatant>,
    session: u32,
    location: String,
}

impl GameState {
    fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    fn load(path: &str) -> Result<Self, std::io::Error> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
```

---

## Advanced Usage

### Custom Dice Distributions

```rust
use rand::Rng;

fn roll_custom(sides: u8, count: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(1..=sides)).collect()
}

// Roll 5d100
let rolls = roll_custom(100, 5);
```

### Property-Based Testing

The engine uses `proptest` for testing. You can too:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_damage_never_negative(bonus in -100i16..100i16) {
        let damage = DamageDice::d6(1).with_bonus(bonus).roll();
        prop_assert!(damage.total >= 0);
    }
}
```

### WASM Compilation

The engine supports WebAssembly:

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
```

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

## Full Example: Discord Bot

```rust
use daggerheart_engine::core::dice::DualityRoll;
use serenity::prelude::*;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!roll" {
            let roll = DualityRoll::roll();
            let result = roll.with_modifier(0);
            
            let response = format!(
                "🎲 Hope: {}, Fear: {}\nTotal: {}",
                roll.hope, roll.fear, result.total
            );
            
            msg.channel_id.say(&ctx.http, response).await.ok();
        }
    }
}
```

---

## 📚 Additional Resources

- **[Full API Docs](https://jakeaboganda.github.io/daggerheart-engine/)** - Complete rustdoc
- **[Examples](../EXAMPLES.md)** - 10 working examples
- **[Game Mechanics](GAME_MECHANICS.md)** - Rules reference
- **[Save Format](SAVE_FORMAT.md)** - JSON schema

---

**Questions?** Open an issue on GitHub or check the full API documentation.
