# Quick Start Guide

## Project Location
```
📁 /home/jake/.openclaw/workspace/daggerheart-engine
```

## Current Status

✅ **Completed**
- [x] Rust project initialized
- [x] Dependencies configured (rand, serde, thiserror, strum, proptest)
- [x] Module structure created (core, character, combat, cards, items)
- [x] Error handling foundation
- [x] Git repository initialized
- [x] Research document created
- [x] TDD plan documented

📋 **Git Status**
```bash
Commits: 2
Branch: master
Latest: "docs: add research findings and TDD development plan"
```

## Next Steps (In Order)

### 1. Get Official Rules 📚
**Priority: CRITICAL**

You need the official Daggerheart playtest materials to verify exact mechanics.

**Options:**
- Visit https://www.daggerheart.com/ and download PDFs
- Check DriveThruRPG: https://drivethrurpg.com/ (search "Daggerheart")
- Use Demiplane: https://app.demiplane.com/

**What to look for:**
- Duality dice mechanics (critical success/failure rules)
- Attribute system
- Character classes list
- Combat action economy
- Hope/Fear pool mechanics

---

### 2. Watch Videos (Optional but Recommended) 🎥
- **Daggerheart 101** - Mechanics overview with Spenser Starke & Matt Mercer
- **Character Creation Demo** - Mercer & Willingham building Bertrand Bell

Take notes while watching!

---

### 3. Document Core Mechanics ✍️
Once you have the rules, we'll update `RESEARCH.md` with:
- Verified duality dice rules
- Exact attribute names and ranges
- Complete class list
- Action economy details
- Critical hit/miss conditions

---

### 4. Write First Tests 🧪
Start with `src/core/dice.rs`:
```bash
# I'll help you write tests like:
- test_d6_rolls_valid_range()
- test_duality_dice_structure()
- test_critical_success()
- test_hope_wins_scenario()
```

---

### 5. Implement TDD Style 🔨
```
Write test → Run test (fails) → Implement code → Run test (passes) → Refactor → Repeat
```

---

## Commands Reference

### Development
```bash
cd daggerheart-engine

# Check compilation
cargo check

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Build release
cargo build --release

# Watch for changes (install cargo-watch first)
cargo watch -x test
```

### Git
```bash
# Check status
git status

# Stage changes
git add .

# Commit
git commit -m "type: message"

# View history
git log --oneline

# Create remote and push
git remote add origin <url>
git push -u origin master
```

### Documentation
```bash
# Generate docs
cargo doc --open

# Run examples (once we create them)
cargo run --example character_creation
```

---

## File Structure Overview

```
daggerheart-engine/
├── README.md           # Project overview
├── DEVELOPMENT.md      # 12-week development roadmap
├── RESEARCH.md         # Rules research findings ⭐ UPDATE THIS FIRST
├── TDD_PLAN.md         # Test-driven development breakdown
├── Cargo.toml          # Rust dependencies
├── .gitignore          # Git exclusions
│
└── src/
    ├── lib.rs              # Main library entry
    ├── error.rs            # Error types (✅ done)
    │
    ├── core/               # 🎲 IMPLEMENT FIRST
    │   └── mod.rs          # Dice & core mechanics
    │
    ├── character/          # 🧙 Implement second
    │   └── mod.rs          # Character system
    │
    ├── combat/             # ⚔️ Implement third
    │   └── mod.rs          # Combat resolution
    │
    ├── cards/              # 🃏 Implement fourth
    │   └── mod.rs          # Domain cards
    │
    └── items/              # ⚔️ Implement fifth
        └── mod.rs          # Equipment
```

---

## How I Can Help

Once you have the rules, I can:

1. **Extract mechanics** - Read PDFs and document exact rules
2. **Write tests** - Create comprehensive test suite
3. **Implement code** - Build the Rust implementation
4. **Review rules** - Validate against official materials
5. **Debug** - Fix issues and edge cases
6. **Optimize** - Profile and improve performance

---

## Questions?

- "Can you help me parse the Daggerheart PDF?" → Yes!
- "Write the first test for duality dice" → Ready!
- "Implement the Die enum" → Let's do it!
- "Should we use property testing?" → Absolutely!
- "How do I push this to GitHub?" → I'll guide you!

---

**Ready to continue when you are! 🚀**

Next: Get those official rules and we'll build this thing properly!
