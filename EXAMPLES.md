# Examples Gallery

This document showcases all examples included with the Daggerheart engine. Each example is fully functional and demonstrates specific features.

---

## 🎲 Dice Mechanics

### 1. Basic Dice (`basic_dice.rs`)

**What it shows:** Rolling standard dice (d4, d6, d8, d10, d12, d20)

**Run:**
```bash
cargo run --example basic_dice
```

**Features demonstrated:**
- Rolling different die types
- Multiple rolls
- Statistical analysis (min, max, average)

**Sample output:**
```
=== Basic Dice Rolling ===

Rolling d20:
  Result: 14

Rolling 10d6:
  Rolls: [3, 6, 1, 4, 5, 2, 6, 3, 4, 5]
  Total: 39
  Average: 3.9
```

---

### 2. Duality Dice (`duality_dice.rs`)

**What it shows:** Daggerheart's core mechanic - 2d12 Hope vs Fear

**Run:**
```bash
cargo run --example duality_dice
```

**Features demonstrated:**
- Basic duality rolls
- Hope vs Fear determination
- Critical success (doubles)
- Modifiers and advantage
- Success with Hope vs Fear

**Sample output:**
```
=== Duality Dice Examples ===

EXAMPLE 1: Basic Duality Roll
  Hope die: 8
  Fear die: 5
  Total: 13
  Controlling: Hope 🌟

EXAMPLE 2: Critical Success!
  Hope die: 7
  Fear die: 7
  🌟 CRITICAL! (Doubles)
  Total: 14

EXAMPLE 3: With Modifier (+3)
  Hope die: 4
  Fear die: 10
  Modifier: +3
  Total: 17
  Controlling: Fear 💀
```

---

### 3. Weapon Damage (`weapon_damage.rs`)

**What it shows:** Rolling damage for weapons

**Run:**
```bash
cargo run --example weapon_damage
```

**Features demonstrated:**
- Damage dice combinations
- Bonus damage
- Multiple dice types in one roll
- Weapon tier examples

**Sample output:**
```
=== Weapon Damage Examples ===

Longsword (Tier 1): d10+3
  Rolls: [7]
  Bonus: +3
  Total damage: 10

Greataxe (Tier 2): 2d8+5
  Rolls: [6, 3]
  Bonus: +5
  Total damage: 14

Dual Wielding: d8+d6+2
  Rolls: [5, 4]
  Bonus: +2
  Total damage: 11
```

---

## 🧙 Character System

### 4. Character Creation (`character_creation.rs`)

**What it shows:** Complete character building process

**Run:**
```bash
cargo run --example character_creation
```

**Features demonstrated:**
- Creating characters with attributes, class, ancestry
- Calculating HP and evasion
- Domain mappings
- Character backgrounds

**Sample output:**
```
=== Character Creation ===

CHARACTER 1: Elara the Inspiring

Name: Elara the Inspiring
Ancestry: Human
Class: Bard

Attributes:
  Presence:  +2 (primary: inspiring performances)
  Knowledge: +1 (secondary: vast lore)
  Instinct:  +1 (secondary: reading people)
  Agility:   +0
  Finesse:   +0
  Strength:  -1

Stats:
  HP: 6 (Class: 6, Ancestry: +0)
  Evasion: 11 (Class: 11, Ancestry: +0)
  Domains: Sage, Codex
```

---

### 5. Character Classes (`character_classes.rs`)

**What it shows:** All 9 classes with details

**Run:**
```bash
cargo run --example character_classes
```

**Features demonstrated:**
- Class starting stats
- Domain mappings
- Playstyle descriptions

**Sample output:**
```
=== Daggerheart Classes ===

CLASS: Warrior
  Starting HP: 6
  Starting Evasion: 10
  Primary Domains: Blade, Bone
  Playstyle: Front-line combatant, high HP, martial prowess

CLASS: Bard
  Starting HP: 6
  Starting Evasion: 11
  Primary Domains: Sage, Codex
  Playstyle: Support and inspiration, knowledge-based abilities
```

---

### 6. Character Ancestries (`character_ancestries.rs`)

**What it shows:** All 17 ancestries with modifiers

**Run:**
```bash
cargo run --example character_ancestries
```

**Features demonstrated:**
- Ancestry modifiers (HP, Evasion)
- Unique traits and lore
- Stat combinations

**Sample output:**
```
=== Daggerheart Ancestries ===

ANCESTRY: Orc
  HP Modifier: +0
  Evasion Modifier: -1
  Traits: Strong, resilient, fierce warriors

ANCESTRY: Faerie
  HP Modifier: -1
  Evasion Modifier: +2
  Traits: Magical, quick, connected to nature
```

---

### 7. Character Attributes (`character_attributes.rs`)

**What it shows:** The 6-attribute system

**Run:**
```bash
cargo run --example character_attributes
```

**Features demonstrated:**
- Standard modifier distribution [+2, +1, +1, +0, +0, -1]
- Attribute meanings
- Different character builds

**Sample output:**
```
=== Character Attributes ===

CONCEPT: Agile Rogue
  Agility:   +2  (lightning reflexes)
  Finesse:   +1  (precise strikes)
  Instinct:  +1  (danger sense)
  Strength:  +0
  Presence:  +0
  Knowledge: -1

CONCEPT: Strong Guardian
  Strength:  +2  (mighty defender)
  Presence:  +1  (inspiring leader)
  Agility:   +1  (combat awareness)
  Finesse:   +0
  Instinct:  +0
  Knowledge: -1
```

---

## ⚔️ Combat System

### 8. Combat Scenario (`combat_scenario.rs`)

**What it shows:** Complete combat encounter

**Run:**
```bash
cargo run --example combat_scenario
```

**Features demonstrated:**
- Creating a combat encounter
- Adding players and enemies
- Rolling initiative
- Turn order
- Combat resolution
- Hope and Fear pools

**Sample output:**
```
=== Combat Scenario ===

Party:
  - Grom the Mighty (Orc Warrior, Level 5)
  - Elara Songweaver (Human Bard, Level 3)

Enemies:
  - Goblin Scout (HP: 4, Evasion: 13)
  - Goblin Archer (HP: 3, Evasion: 14)

Initiative Order:
  1. Elara Songweaver (Initiative: 18)
  2. Goblin Scout (Initiative: 13)
  3. Grom the Mighty (Initiative: 11)
  4. Goblin Archer (Initiative: 5)

Round 1 starts!
Current Hope: 5/5
Current Fear: 0/10
```

---

## 📈 Character Progression

### 9. Leveling Up (`leveling_up.rs`)

**What it shows:** XP system and character advancement

**Run:**
```bash
cargo run --example leveling_up
```

**Features demonstrated:**
- Gaining experience
- Level-up mechanics
- XP requirements per level
- Card/ability acquisition
- Multi-level progression

**Sample output:**
```
=== Character Progression ===

EXAMPLE 1: Basic Leveling
Starting: Level 1, 0 XP
📈 Gained 75 XP
📈 Gained 30 XP (Total: 105)
🎉 LEVEL UP!
  New level: 2
  Remaining XP: 5
  XP needed for next level: 200

EXAMPLE 2: Multi-Level Progression
🏆 Major quest completed! Gained 650 XP!
⬆️ Leveling up:
  Level 1 → 2 (cost: 100 XP, remaining: 550)
  Level 2 → 3 (cost: 200 XP, remaining: 350)
  Level 3 → 4 (cost: 300 XP, remaining: 50)
✅ Leveled up 3 times!
```

---

### 10. Save and Load (`save_and_load.rs`)

**What it shows:** Persistent game state using JSON

**Run:**
```bash
cargo run --example save_and_load
```

**Features demonstrated:**
- Saving characters to files
- Loading character state
- Combat encounter persistence
- Character progression tracking
- Multi-session campaign workflow

**Sample output:**
```
=== Save/Load System ===

EXAMPLE 1: Save and Load Character
Created character: Grom the Mighty
  Level: 5
  HP: 6/6
  Class: Warrior

✅ Character saved to: grom_character.json
✅ Character loaded from: grom_character.json

EXAMPLE 4: Persistence Across Sessions
📖 Session 1:
  Character created: Theron (Level 2)
  XP gained: 20
  Took damage, HP: 1/6
  ✅ Session saved!

📖 Session 2 (next week):
  Loaded character: Theron (Level 2)
  Current HP: 1/6
  Rested and healed to full HP
  🎉 LEVEL UP! Now level 3
  ✅ Session saved!
```

---

## 🎯 Running Multiple Examples

### Run all examples at once:
```bash
for example in basic_dice duality_dice weapon_damage \
               character_creation character_classes character_ancestries character_attributes \
               combat_scenario leveling_up save_and_load; do
    echo "=== Running $example ==="
    cargo run --example $example
    echo ""
done
```

### Run with timing:
```bash
time cargo run --example combat_scenario
```

### Run quietly (minimal output):
```bash
cargo run --example basic_dice 2>/dev/null
```

---

## 📚 Learning Path

### Beginner (Start Here)
1. **basic_dice.rs** - Understand dice rolling
2. **duality_dice.rs** - Learn Hope vs Fear mechanic
3. **character_creation.rs** - Create your first character

### Intermediate
4. **character_classes.rs** - Explore all classes
5. **character_ancestries.rs** - Explore all ancestries
6. **weapon_damage.rs** - Understand damage calculation

### Advanced
7. **combat_scenario.rs** - Run full combat
8. **leveling_up.rs** - Manage progression
9. **save_and_load.rs** - Persist game state

### Expert
10. Read the source code of examples
11. Build your own tools using the library
12. Create custom examples for your use cases

---

## 🛠️ Customizing Examples

All examples are in `examples/` directory. Feel free to:
- Modify values
- Add your own characters
- Test different scenarios
- Use as templates for your projects

**Example: Create a custom character**
```rust
// Copy examples/character_creation.rs and modify:
let my_character = Combatant::player(
    "YourName",
    5,  // Level
    Class::Wizard,
    Ancestry::Faerie,
    Attributes::from_array([1, -1, 2, 1, 1, 0]).unwrap(),
);
```

---

## 💡 Tips

- Use `cargo run --example <name> -- --help` if the example accepts arguments
- Examples are self-contained - read the source to learn!
- Output varies due to random dice rolls
- Some examples create temporary JSON files (auto-cleaned)

---

## 🔗 See Also

- **[QUICKSTART.md](QUICKSTART.md)** - Quick installation guide
- **[docs/TUTORIAL.md](docs/TUTORIAL.md)** - Step-by-step tutorial
- **[docs/API_GUIDE.md](docs/API_GUIDE.md)** - Library usage guide
- **[docs/SAVE_FORMAT.md](docs/SAVE_FORMAT.md)** - JSON save format

---

**Have fun exploring! 🎮**
