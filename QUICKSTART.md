# Quick Start Guide

Get started with the Daggerheart engine in 5 minutes!

---

## Choose Your Path

### 🎮 **CLI Tool** (No coding required)
Perfect for: Quick dice rolls, character management, testing

### 📚 **Rust Library** (For developers)
Perfect for: Building apps, bots, games, tools

---

## 🎮 CLI Quick Start

### Step 1: Install

```bash
git clone https://github.com/jakeaboganda/daggerheart-engine.git
cd daggerheart-engine
cargo install --path .
```

### Step 2: Use It!

```bash
# Roll dice
daggerheart roll die d20
daggerheart roll duality +3
daggerheart roll damage 2d6+3

# Create a character
daggerheart char create "Grom" --class Warrior --ancestry Orc

# View character
daggerheart char show Grom_char.json

# Get help
daggerheart --help
daggerheart roll --help
```

**That's it!** You're ready to roll.

---

## 📚 Library Quick Start

### Step 1: Add Dependency

**Cargo.toml:**
```toml
[dependencies]
daggerheart-engine = { git = "https://github.com/jakeaboganda/daggerheart-engine" }
```

### Step 2: Use It!

```rust
use daggerheart_engine::core::dice::DualityRoll;

fn main() {
    let roll = DualityRoll::roll();
    let result = roll.with_modifier(3);
    
    println!("Hope: {}, Fear: {}", roll.hope, roll.fear);
    println!("Total: {}", result.total);
}
```

**That's it!** You're coding with Daggerheart.

---

## 🎯 What Can You Do?

### Roll Dice
```bash
daggerheart roll die d20                    # Basic die
daggerheart roll duality +5 --advantage     # Hope vs Fear
daggerheart roll damage 2d8+3               # Weapon damage
```

### Manage Characters
```bash
daggerheart char create "Hero" --class Bard --ancestry Human
daggerheart char add-xp Hero_progress.json 150
daggerheart char level-up Hero_progress.json --card "inspire"
daggerheart char show Hero_char.json
```

### Run Combat
```bash
daggerheart combat new -o battle.json
daggerheart combat add battle.json --character Hero_char.json
daggerheart combat add battle.json --enemy "Goblin" --hp 4
daggerheart combat start battle.json
daggerheart combat status battle.json
```

### See Available Options
```bash
daggerheart classes       # List all 9 classes
daggerheart ancestries    # List all 17 ancestries
```

---

## 📖 Learn More

**Next steps:**
- **[Full Tutorial](docs/TUTORIAL.md)** - 30-minute walkthrough
- **[Examples Gallery](EXAMPLES.md)** - See 10 working examples
- **[API Guide](docs/API_GUIDE.md)** - For developers using the library

**References:**
- **[CLI Help](README.md#-what-can-you-do)** - All commands explained
- **[Save Format](docs/SAVE_FORMAT.md)** - JSON file format
- **[Game Mechanics](docs/GAME_MECHANICS.md)** - Daggerheart rules
- **[API Docs](https://jakeaboganda.github.io/daggerheart-engine/)** - Complete API

---

## 🧪 Try Examples

```bash
# Character creation
cargo run --example character_creation

# Dice rolling
cargo run --example duality_dice

# Combat simulation
cargo run --example combat_scenario

# Leveling up
cargo run --example leveling_up

# Save and load
cargo run --example save_and_load
```

See [EXAMPLES.md](EXAMPLES.md) for all examples.

---

## ⚡ Common Commands Reference

### Dice Rolling
```bash
# Roll a d20
daggerheart roll die d20

# Roll with advantage
daggerheart roll duality +3 --advantage

# Roll damage
daggerheart roll damage 2d6+1d4+3
```

### Character Management
```bash
# Create
daggerheart char create NAME --class CLASS --ancestry ANCESTRY

# View
daggerheart char show FILE.json

# Progression
daggerheart char add-xp FILE.json AMOUNT
daggerheart char level-up FILE.json --card CARD_ID
```

### Combat
```bash
# Setup
daggerheart combat new -o FILE.json
daggerheart combat add FILE.json --character CHAR.json
daggerheart combat add FILE.json --enemy NAME --hp N

# Run
daggerheart combat start FILE.json
daggerheart combat status FILE.json
```

---

## 🛠️ Development Setup

```bash
# Clone
git clone https://github.com/jakeaboganda/daggerheart-engine.git
cd daggerheart-engine

# Test
cargo test          # Run all 218 tests

# Run examples
cargo run --example combat_scenario

# Build
cargo build --release

# Generate docs
cargo doc --open
```

---

## 🆘 Troubleshooting

**Command not found?**
```bash
# Make sure cargo bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Or reinstall
cargo install --path /path/to/daggerheart-engine
```

**Tests failing?**
```bash
cargo clean
cargo build
cargo test
```

**Need help?**
- Check [Tutorial](docs/TUTORIAL.md) for detailed walkthrough
- Read [API Guide](docs/API_GUIDE.md) for library usage
- Open an issue on GitHub

---

## 🎉 You're Ready!

**CLI users:** Start rolling dice with `daggerheart roll die d20`  
**Developers:** Check out [docs/API_GUIDE.md](docs/API_GUIDE.md)  
**Learners:** Follow [docs/TUTORIAL.md](docs/TUTORIAL.md)

**Have fun!** 🎲
