# Daggerheart Engine Tutorial

Welcome! This tutorial will guide you step-by-step from installation to running your first combat encounter.

**Time required:** ~30 minutes  
**Prerequisites:** Basic command-line knowledge

---

## Part 1: Installation & Setup (5 minutes)

### Step 1.1: Install Rust

If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify:
```bash
cargo --version  # Should show: cargo 1.x.x
```

### Step 1.2: Clone the Repository

```bash
git clone https://github.com/jakeaboganda/daggerheart-engine.git
cd daggerheart-engine
```

### Step 1.3: Test the Installation

```bash
# Run tests to verify everything works
cargo test

# Should see: test result: ok. 218 passed
```

### Step 1.4: Install the CLI (Optional but Recommended)

```bash
cargo install --path .
```

Now you can run `daggerheart` from anywhere!

---

## Part 2: Your First Dice Roll (5 minutes)

### Step 2.1: Roll a Basic Die

```bash
daggerheart roll die d20
```

**Output:**
```
🎲 Rolling 1xd20:
  Roll 1: 14
```

### Step 2.2: Roll Multiple Dice

```bash
daggerheart roll die d6 --count 5
```

**Output:**
```
🎲 Rolling 5xd6:
  Roll 1: 3
  Roll 2: 6
  Roll 3: 2
  Roll 4: 4
  Roll 5: 5

Rolls: [3, 6, 2, 4, 5]
Total: 20
Average: 4.00
```

### Step 2.3: Roll Duality Dice (Hope vs Fear)

This is Daggerheart's signature mechanic!

```bash
daggerheart roll duality +3
```

**Output:**
```
🎲 Duality Roll:
  Hope die: 8
  Fear die: 5
  Modifier: +3
  Total: 16

✅ Hope controls! 🌟
```

**What does this mean?**
- You rolled 2d12: one Hope (8), one Fear (5)
- Hope won (8 > 5), so you succeed WITH HOPE
- Added your modifier (+3) for total of 16
- In a game, you'd gain 1 Hope point!

### Step 2.4: Roll with Advantage

```bash
daggerheart roll duality +2 --advantage
```

**Output:**
```
🎲 Duality Roll:
  Hope die: 7
  Fear die: 11
  Advantage die: 4
  Modifier: +2
  Total: 24

⚠️ Fear controls... 💀
```

With advantage, you roll an extra d6!

---

## Part 3: Create Your First Character (10 minutes)

### Step 3.1: List Available Options

```bash
# See all classes
daggerheart classes

# See all ancestries
daggerheart ancestries
```

### Step 3.2: Create a Character

Let's create a Human Warrior named Grom:

```bash
daggerheart char create "Grom the Brave" \
  --class Warrior \
  --ancestry Human \
  --level 1
```

**Output:**
```
✅ Character created!
  Name: Grom the Brave
  Class: Warrior
  Ancestry: Human
  Level: 1
  HP: 6/6
  Evasion: 10

📁 Files saved:
  Character: Grom the Brave_char.json
  Progress: Grom the Brave_progress.json
```

### Step 3.3: View Your Character

```bash
daggerheart char show "Grom the Brave_char.json"
```

**Output:**
```
=== Grom the Brave ===
  Class: Warrior
  Ancestry: Human
  Level: 1

  HP: 6/6
  Stress: 0/5
  Evasion: 10
  Armor: 0

Attributes:
  Agility:   +2
  Strength:  +1
  Finesse:   +1
  Instinct:  +0
  Presence:  +0
  Knowledge: -1

Progress:
  Level: 1
  XP: 0 / 100
  Cards: []
```

### Step 3.4: Understand Attributes

The default attributes `[+2, +1, +1, +0, +0, -1]` are distributed automatically. You can customize:

```bash
daggerheart char create "Mage" \
  --class Wizard \
  --ancestry Faerie \
  --attributes "1,1,2,0,0,-1"
```

**Attribute meanings:**
- **Agility:** Reflexes, dodging
- **Strength:** Physical power
- **Finesse:** Precision, accuracy
- **Instinct:** Gut feelings, perception
- **Presence:** Charisma, leadership
- **Knowledge:** Intelligence, lore

---

## Part 4: Character Progression (5 minutes)

### Step 4.1: Add Experience

After an adventure, Grom gains 120 XP:

```bash
daggerheart char add-xp "Grom the Brave_progress.json" 120
```

**Output:**
```
📈 Added 120 XP
  Total XP: 120
  Level: 1
  XP for next level: 100

💡 You can now level up! Run:
   daggerheart char level-up Grom the Brave_progress.json
```

### Step 4.2: Level Up

```bash
daggerheart char level-up "Grom the Brave_progress.json" \
  --card "blade_strike"
```

**Output:**
```
🎉 LEVEL UP!
  1 → 2
  Remaining XP: 20
  ✨ Learned: blade_strike

✅ Progress saved to Grom the Brave_progress.json
```

### Step 4.3: Check Progress Again

```bash
daggerheart char show "Grom the Brave_char.json"
```

Now the progress section shows:
```
Progress:
  Level: 2
  XP: 20 / 200
  Cards: ["blade_strike"]
```

---

## Part 5: Your First Combat (10 minutes)

### Step 5.1: Create a Combat Encounter

```bash
daggerheart combat new --hope 5 --output first_battle.json
```

**Output:**
```
✅ Combat encounter created!
  Hope pool: 5
  File: first_battle.json

💡 Add combatants with:
   daggerheart combat add first_battle.json --character <file>
   daggerheart combat add first_battle.json --enemy <name>
```

### Step 5.2: Add Your Hero

```bash
daggerheart combat add first_battle.json \
  --character "Grom the Brave_char.json"
```

**Output:**
```
➕ Adding player: Grom the Brave

✅ Combatant added! Total: 1
```

### Step 5.3: Add Enemies

```bash
# Add a goblin scout
daggerheart combat add first_battle.json \
  --enemy "Goblin Scout" \
  --hp 4 \
  --evasion 13 \
  --armor 0

# Add a goblin archer
daggerheart combat add first_battle.json \
  --enemy "Goblin Archer" \
  --hp 3 \
  --evasion 14 \
  --armor 0
```

### Step 5.4: Start Combat (Roll Initiative)

```bash
daggerheart combat start first_battle.json
```

**Output:**
```
⚔️ Combat started!

Initiative order:
  1. Grom the Brave (Initiative: 15)
  2. Goblin Archer (Initiative: 12)
  3. Goblin Scout (Initiative: 8)

✅ Encounter saved
```

### Step 5.5: Check Combat Status

```bash
daggerheart combat status first_battle.json
```

**Output:**
```
=== Combat Status ===
Round: 1
Hope: 5/5
Fear: 0

Current turn: Grom the Brave

Combatants:
  Grom the Brave [Alive] - HP: 6/6, Evasion: 10, Armor: 0
  Goblin Scout [Alive] - HP: 4/4, Evasion: 13, Armor: 0
  Goblin Archer [Alive] - HP: 3/3, Evasion: 14, Armor: 0
```

---

## Part 6: Using as a Library (5 minutes)

Now let's write some Rust code!

### Step 6.1: Create a New Project

```bash
cargo new my_daggerheart_game
cd my_daggerheart_game
```

### Step 6.2: Add Dependency

Edit `Cargo.toml`:
```toml
[dependencies]
daggerheart-engine = { path = "../daggerheart-engine" }
```

### Step 6.3: Write Your First Program

Edit `src/main.rs`:
```rust
use daggerheart_engine::core::dice::{DualityRoll, ControllingDie};

fn main() {
    println!("=== My First Daggerheart Program ===\n");

    // Roll duality dice
    let roll = DualityRoll::roll();
    let result = roll.with_modifier(3);

    println!("Hope: {}, Fear: {}", roll.hope, roll.fear);
    println!("Total: {} (+3 modifier)", result.total);

    match result.controlling {
        ControllingDie::Hope => println!("✅ Success with Hope!"),
        ControllingDie::Fear => println!("⚠️ Success with Fear..."),
        ControllingDie::Tied => println!("🔄 Tied!"),
    }

    if result.is_critical {
        println!("🌟 CRITICAL! (Doubles)");
    }
}
```

### Step 6.4: Run It

```bash
cargo run
```

**Output:**
```
=== My First Daggerheart Program ===

Hope: 9, Fear: 4
Total: 16 (+3 modifier)
✅ Success with Hope!
```

---

## Part 7: Save & Load Game State

### Step 7.1: Inspect the JSON Files

```bash
cat "Grom the Brave_char.json" | head -20
```

**You'll see:**
```json
{
  "name": "Grom the Brave",
  "level": 1,
  "class": "Warrior",
  "ancestry": "Human",
  "attributes": {
    "agility": 2,
    "strength": 1,
    "finesse": 1,
    "instinct": 0,
    "presence": 0,
    "knowledge": -1
  },
  ...
}
```

### Step 7.2: Manually Edit (Optional)

You can edit these files! For example, heal your character:

```bash
# Use any text editor
nano "Grom the Brave_char.json"

# Find "hp" and set current = maximum
"hp": {
  "current": 6,
  "maximum": 6
}
```

### Step 7.3: Load in Code

```rust
use daggerheart_engine::combat::simulation::Combatant;

fn main() {
    // Load character
    let warrior = Combatant::load_from_file("Grom the Brave_char.json")
        .expect("Failed to load character");

    println!("Loaded: {} (Level {})", warrior.name, warrior.level);
    println!("HP: {}/{}", warrior.hp.current, warrior.hp.maximum);

    // Modify and save
    let mut warrior = warrior;
    warrior.hp.current = warrior.hp.maximum;  // Full heal

    warrior.save_to_file("Grom the Brave_char.json")
        .expect("Failed to save");

    println!("Character healed and saved!");
}
```

---

## 🎓 What's Next?

### Explore More Features
- **[EXAMPLES.md](../EXAMPLES.md)** - See all 10 examples
- **[docs/API_GUIDE.md](API_GUIDE.md)** - Deep dive into the API
- **[docs/GAME_MECHANICS.md](GAME_MECHANICS.md)** - Learn Daggerheart rules

### Build Something Cool
- Discord bot for rolling dice
- Character generator with random names
- Combat simulator for testing builds
- Campaign manager with Git versioning

### Contribute
- **[docs/CONTRIBUTING.md](CONTRIBUTING.md)** - Development guide
- Report bugs on GitHub Issues
- Add new examples
- Improve documentation

---

## 💡 Pro Tips

1. **Use Git for campaigns:**
   ```bash
   git init my_campaign
   cd my_campaign
   daggerheart char create "Hero" --class Ranger --ancestry Faerie
   git add .
   git commit -m "Session 1: Character creation"
   ```

2. **Backup often:**
   ```bash
   cp *.json backups/session_$(date +%Y%m%d)/
   ```

3. **Share characters:**
   JSON files are portable - email them to other players!

4. **Automate rolls:**
   ```bash
   # Roll 100 times to see average
   for i in {1..100}; do
     daggerheart roll die d20
   done | grep "Roll 1:" | awk '{sum+=$3} END {print sum/NR}'
   ```

---

## 🆘 Troubleshooting

### "Command not found: daggerheart"
```bash
# Install the CLI
cargo install --path /path/to/daggerheart-engine

# Or add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### "Invalid JSON" when loading
```bash
# Validate JSON
cat character.json | python3 -m json.tool

# Or use jq
jq . character.json
```

### Tests failing
```bash
# Clean and rebuild
cargo clean
cargo build
cargo test
```

---

## 🎉 Congratulations!

You've completed the tutorial! You now know how to:
- ✅ Roll dice (basic, duality, damage)
- ✅ Create and manage characters
- ✅ Track progression (XP, leveling, cards)
- ✅ Set up combat encounters
- ✅ Save and load game state
- ✅ Use the library in your own code

**Go forth and build amazing things!** 🚀

---

**Questions?** Check the [API Documentation](https://jakeaboganda.github.io/daggerheart-engine/) or open an issue on GitHub.
