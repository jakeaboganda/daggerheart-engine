# Daggerheart Engine

[![CI](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/ci.yml)
[![Documentation](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/docs.yml/badge.svg)](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/docs.yml)

**A complete Rust implementation of the Daggerheart TTRPG rules system.**

Use it as a **library** in your Rust projects, or as a **command-line tool** for quick dice rolls and character management—no coding required!

---

## ✨ What Can You Do?

### 🎲 Roll Dice
```bash
# Basic dice
daggerheart roll die d20 --count 3

# Duality dice (Hope vs Fear - Daggerheart's core mechanic)
daggerheart roll duality +5

# Damage dice
daggerheart roll damage 2d6+3
```

### 🧙 Create Characters
```bash
# Create a character (auto-saves as JSON)
daggerheart char create "Grom" --class Warrior --ancestry Orc --level 3

# View character
daggerheart char show Grom_char.json

# Add experience and level up
daggerheart char add-xp Grom_progress.json 150
daggerheart char level-up Grom_progress.json --card "blade_strike"
```

### ⚔️ Run Combat
```bash
# Create encounter
daggerheart combat new --hope 5 --output battle.json

# Add combatants
daggerheart combat add battle.json --character hero.json
daggerheart combat add battle.json --enemy "Goblin" --hp 4 --evasion 13

# Start combat (rolls initiative)
daggerheart combat start battle.json

# Check status
daggerheart combat status battle.json
```

### 📚 Use as a Library
```rust
use daggerheart_engine::prelude::*;

// Create a character
let warrior = Combatant::player(
    "Grom",
    5,
    Class::Warrior,
    Ancestry::Orc,
    Attributes::from_array([2, 1, 1, 0, 0, -1])?,
);

// Roll duality dice
let roll = DualityRoll::roll();
let result = roll.with_modifier(3);

if result.controlling == ControllingDie::Hope {
    println!("Success with Hope! 🌟");
}

// Save character
warrior.save_to_file("grom.json")?;
```

---

## 🚀 Quick Start

### Option 1: CLI Tool (No Coding)

```bash
# Clone the repository
git clone https://github.com/jakeaboganda/daggerheart-engine.git
cd daggerheart-engine

# Install the CLI
cargo install --path .

# Start using it!
daggerheart --help
daggerheart classes  # List all classes
daggerheart roll duality +3
```

### Option 2: Rust Library

Add to your `Cargo.toml`:
```toml
[dependencies]
daggerheart-engine = { git = "https://github.com/jakeaboganda/daggerheart-engine" }
```

Then use it:
```rust
use daggerheart_engine::prelude::*;

fn main() {
    let roll = DualityRoll::roll().with_modifier(2);
    println!("Total: {}", roll.total);
}
```

See [docs/TUTORIAL.md](docs/TUTORIAL.md) for a complete walkthrough.

---

## 📖 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - 5-minute getting started guide
- **[EXAMPLES.md](EXAMPLES.md)** - Gallery of all examples with sample output
- **[docs/TUTORIAL.md](docs/TUTORIAL.md)** - Step-by-step tutorial
- **[docs/API_GUIDE.md](docs/API_GUIDE.md)** - How to use the library
- **[docs/SAVE_FORMAT.md](docs/SAVE_FORMAT.md)** - JSON save file format
- **[docs/GAME_MECHANICS.md](docs/GAME_MECHANICS.md)** - Daggerheart rules reference
- **[API Documentation](https://jakeaboganda.github.io/daggerheart-engine/)** - Complete API reference

---

## 🎯 Features

### Core Mechanics ✅
- **Duality Dice** (2d12 Hope vs Fear) - The heart of Daggerheart
- **All standard dice** (d4, d6, d8, d10, d12, d20)
- **Damage dice** with bonuses
- **Critical detection** (doubles!)
- **Advantage system**

### Character System ✅
- **9 classes** - Bard, Druid, Guardian, Ranger, Rogue, Seraph, Sorcerer, Warrior, Wizard
- **17 ancestries** - Clank, Daemon, Drakona, Dwarf, Faerie, Faun, Fungril, Galapa, Giant, Goblin, Halfling, Human, Inferis, Katari, Orc, Ribbet, Simiah
- **6 attributes** with standard modifiers
- **HP, Stress, Evasion, Armor**
- **Progression** (levels 1-10, XP system)

### Combat System ✅
- **Turn-based encounters**
- **Automatic initiative**
- **Attack resolution**
- **Damage calculation**
- **Resource management** (HP, Stress, Hope, Fear)

### Abilities & Cards ✅
- **Domain cards framework**
- **Level requirements**
- **Action economy** (Major/Minor/Reaction)
- **Card effects** (Attack, Heal, Modifier, etc.)

### Save/Load ✅
- **JSON format** (human-readable, editable)
- **One-line save/load** methods
- **Full state persistence**
- **Git-friendly** files

---

## 🎮 Examples

We provide 10 complete examples:

```bash
# Dice mechanics
cargo run --example basic_dice
cargo run --example duality_dice
cargo run --example weapon_damage

# Character system
cargo run --example character_creation
cargo run --example character_classes
cargo run --example character_ancestries
cargo run --example character_attributes

# Combat
cargo run --example combat_scenario

# Progression
cargo run --example leveling_up
cargo run --example save_and_load
```

See [EXAMPLES.md](EXAMPLES.md) for detailed descriptions and sample output.

---

## 🛠️ Development

### Run Tests
```bash
# All tests (218 tests)
cargo test

# Specific module
cargo test character
cargo test combat
cargo test core::dice
```

### Build
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# CLI tool
cargo build --bin daggerheart --release
```

### Documentation
```bash
# Generate API docs
cargo doc --open

# Run examples
cargo run --example character_creation
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint with Clippy
cargo clippy -- -D warnings

# Quick CI check (local)
./scripts/ci-quick.sh
```

---

## 📊 Status

| Metric | Status |
|--------|--------|
| **Tests** | 218/218 passing (100%) |
| **Test Coverage** | Full (unit + property + doc tests) |
| **Clippy Warnings** | 0 |
| **Doc Coverage** | 100% public API |
| **Examples** | 10 complete examples |
| **Quality Score** | ⭐ 10/10 |

**Development:** Strict TDD methodology  
**CI/CD:** GitHub Actions (format, lint, test, docs)  
**Type Safety:** Full Rust guarantees  

---

## 🎯 Use Cases

Perfect for:

- 🎮 **Digital TTRPG Tools** - Character sheets, dice rollers, campaign managers
- 🤖 **Discord/Slack Bots** - Automated game management in chat
- 🌐 **Web Apps** - Browser-based gameplay (WASM ready)
- 📱 **Mobile Apps** - Companion apps for players
- 🧪 **Rules Testing** - Validate game balance and mechanics
- 🎓 **Learning** - Study Rust, TDD, and game design

---

## 🏗️ Project Structure

```
daggerheart-engine/
├── src/
│   ├── bin/
│   │   └── daggerheart.rs    # CLI tool (600+ lines)
│   ├── core/
│   │   └── dice/             # Dice mechanics (62 tests)
│   ├── character/            # Character system (59 tests)
│   ├── combat/               # Combat system (64 tests)
│   ├── cards/                # Card system (33 tests)
│   └── lib.rs                # Public API
├── examples/                 # 10 comprehensive examples
├── docs/                     # User guides and references
└── scripts/                  # CI/CD tooling
```

---

## 🤝 Contributing

Contributions welcome! See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for:
- Development setup
- Code style guide
- Testing requirements
- Pull request process

---

## 📜 License

MIT OR Apache-2.0

---

## 🙏 Acknowledgments

Built with **strict test-driven development** methodology. Every feature was test-first, ensuring production-ready quality from day one.

**Repository:** https://github.com/jakeaboganda/daggerheart-engine  
**Documentation:** https://jakeaboganda.github.io/daggerheart-engine/  

---

## 🔗 Links

- **[Getting Started](QUICKSTART.md)** - Quick installation and first steps
- **[Examples Gallery](EXAMPLES.md)** - See all examples in action
- **[Tutorial](docs/TUTORIAL.md)** - Complete step-by-step guide
- **[API Guide](docs/API_GUIDE.md)** - Library usage guide
- **[Game Mechanics](docs/GAME_MECHANICS.md)** - Daggerheart rules reference
- **[Contributing](docs/CONTRIBUTING.md)** - Development guide

---

**Built with ❤️ and 218 tests in Rust** 🦀
