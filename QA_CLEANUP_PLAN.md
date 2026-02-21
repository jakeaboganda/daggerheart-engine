# QA Cleanup & Enhancement Plan

## 🎯 Executive Summary

The Daggerheart engine is **functionally complete and well-tested** (218/218 tests passing). However, the documentation is heavily polluted with development process artifacts that confuse potential users. This plan focuses on:

1. **Removing development artifacts** (13 process docs)
2. **Restructuring user-facing documentation** (examples-driven)
3. **Adding quality-of-life features** (CLI, save states, better onboarding)

---

## 📊 Current State Analysis

### ✅ Strengths
- **Solid codebase**: 4,600+ lines, 100% test coverage
- **Good examples**: 8 working examples demonstrating features
- **Complete features**: All core Daggerheart mechanics implemented
- **CI/CD**: Working GitHub Actions pipeline

### ⚠️ Issues
- **Documentation clutter**: 13 phase/process docs (CI_CD_SETUP.md, PHASE_*.md, etc.)
- **Developer-centric README**: Talks about phases, not usage
- **Missing user guides**: No "Getting Started", "API Guide", or "Tutorial"
- **No CLI**: Users can't easily experiment without writing Rust code
- **Limited save/load examples**: JSON serialization exists but underdocumented

---

## 🗑️ Phase 1: Documentation Cleanup

### Files to DELETE (Development Artifacts)

```bash
# CI/CD process docs (move details to .github/workflows or delete)
CI_CD_SETUP.md
CI_READY_FOR_PHASE_2.md
CI_UNIFICATION.md
LOCAL_CI_SETUP.md

# Phase completion docs (archive or delete)
PHASE_1_COMPLETE.md
PHASE_1_REVIEW.md
PHASE_2_COMPLETE.md
PHASE_2_PROGRESS.md
PHASE_3_COMPLETE.md
PHASE_4_COMPLETE.md
PHASE_5_COMPLETE.md

# Meta docs (archive)
EXAMPLES_ADDED.md
PROJECT_COMPLETE.md
QA_REPORT_PRE_PHASE_5.md
MULTI_AGENT_RESEARCH.md
TDD_PLAN.md
IMPLEMENTATION_PLAN.md
```

**Total: 17 files to remove** ✂️

### Files to KEEP (but potentially refactor)

```bash
README.md              # Rewrite to be user-focused
QUICKSTART.md          # Good for onboarding
DEVELOPMENT.md         # Move to docs/CONTRIBUTING.md
MECHANICS_DEEP_DIVE.md # Rename to API_REFERENCE.md or GAME_MECHANICS.md
RESEARCH.md            # Move to docs/RESEARCH.md (for contributors)
```

---

## 📚 Phase 2: Documentation Restructure

### New Documentation Structure

```
daggerheart-engine/
├── README.md               # ⭐ User-focused overview
├── QUICKSTART.md           # 5-minute getting started
├── GAME_MECHANICS.md       # Reference for Daggerheart rules
├── EXAMPLES.md             # Gallery of all examples with outputs
│
├── docs/
│   ├── INSTALLATION.md     # Cargo, WASM, bindings
│   ├── API_GUIDE.md        # How to use each module
│   ├── TUTORIAL.md         # Build a simple game step-by-step
│   ├── CONTRIBUTING.md     # Dev guide (from DEVELOPMENT.md)
│   ├── CHANGELOG.md        # Version history
│   └── RESEARCH.md         # Background research (archive)
│
└── examples/               # Keep all 8 examples
```

---

## 📝 Phase 3: Improve User-Facing Docs

### 3.1 Rewrite README.md

**Current problems:**
- Starts with "Project Complete!" (developer mindset)
- Lists phases and test counts (irrelevant to users)
- Buries actual usage examples

**New structure:**
1. **Hero section**: What is this? Why use it?
2. **Quick example**: Show a complete character + dice roll (3 lines)
3. **Features**: Bullet list of what you can do
4. **Installation**: `cargo add daggerheart-engine`
5. **Examples**: Link to examples with brief descriptions
6. **Documentation**: Link to guides and API docs
7. **Contributing**: Link to CONTRIBUTING.md

### 3.2 Create EXAMPLES.md

Gallery format:

```markdown
# Examples

## 🎲 Basic Dice Rolling
**File:** `examples/basic_dice.rs`
**What it shows:** Roll different dice types, duality dice, damage dice
**Run:** `cargo run --example basic_dice`
**Output:**
[sample output here]

## 🧙 Character Creation
**File:** `examples/character_creation.rs`
...
```

### 3.3 Create TUTORIAL.md

Step-by-step guide:
1. Create your first character
2. Roll some dice
3. Simulate a simple attack
4. Run a full combat encounter
5. Save and load character state

---

## 🚀 Phase 4: Quality-of-Life Features

### 4.1 CLI Tool (`daggerheart-cli`)

**Purpose:** Let users experiment without writing code

**Features:**
```bash
# Character management
daggerheart char create "Elara" --class Bard --ancestry Human
daggerheart char show elara.json
daggerheart char level-up elara.json

# Dice rolling
daggerheart roll 2d12           # Basic roll
daggerheart roll duality +5     # Duality dice with modifier
daggerheart roll damage 2d6+3   # Damage roll

# Combat simulation
daggerheart combat start
daggerheart combat add-player "Warrior" 5 --hp 25 --evasion 12
daggerheart combat add-enemy "Goblin" 1 --hp 4 --evasion 13
daggerheart combat initiative
daggerheart combat attack "Warrior" "Goblin" +3

# Save states
daggerheart save session combat_encounter.json
daggerheart load session combat_encounter.json
```

**Implementation:**
- Create `src/bin/daggerheart.rs`
- Use `clap` for CLI parsing
- Leverage existing engine API
- Add to `Cargo.toml` with `[[bin]]` section

### 4.2 Better Save/Load System

**Current state:** JSON serialization exists but underdocumented

**Improvements:**
1. **Create save state examples**
   - `examples/save_character.rs`
   - `examples/save_combat.rs`
   - `examples/campaign_state.rs`

2. **Add convenience methods**
   ```rust
   impl Combatant {
       pub fn save_to_file(&self, path: &str) -> Result<()>;
       pub fn load_from_file(path: &str) -> Result<Self>;
   }
   
   impl CombatEncounter {
       pub fn save_session(&self, path: &str) -> Result<()>;
       pub fn load_session(path: &str) -> Result<Self>;
   }
   ```

3. **Document save formats**
   - Create `docs/SAVE_FORMAT.md`
   - Show example JSON structures
   - Explain versioning strategy

### 4.3 Interactive REPL

**Purpose:** Experiment with the engine interactively

**Example session:**
```
$ daggerheart repl
> create character Grom
> set class Warrior
> set ancestry Orc
> set strength +2
> roll duality +2
Hope: 8, Fear: 3, Total: 10 (Success with Hope!)
> save grom.json
Character saved!
```

**Implementation:**
- Use `rustyline` crate for REPL
- Parse commands with simple regex or `nom`
- Maintain session state

### 4.4 Web Playground (WASM)

**Purpose:** Try the engine in browser without installing Rust

**Features:**
- Character creator UI
- Dice roller with animations
- Combat simulator
- Export/import JSON states

**Tech:**
- Already has `wasm-bindgen` in Cargo.toml
- Create `www/` directory with simple HTML/JS
- Deploy to GitHub Pages

---

## 🎯 Phase 5: Additional Enhancements

### 5.1 More Examples

```bash
examples/
├── basic_dice.rs           # ✅ Exists
├── duality_dice.rs         # ✅ Exists
├── weapon_damage.rs        # ✅ Exists
├── character_creation.rs   # ✅ Exists
├── character_ancestries.rs # ✅ Exists
├── character_attributes.rs # ✅ Exists
├── character_classes.rs    # ✅ Exists
├── combat_scenario.rs      # ✅ Exists
├── save_and_load.rs        # ⭐ NEW
├── leveling_up.rs          # ⭐ NEW
├── custom_abilities.rs     # ⭐ NEW
└── full_session.rs         # ⭐ NEW (complete game session)
```

### 5.2 Testing Improvements

**Property-based tests** (already using `proptest`):
- Add more property tests for edge cases
- Fuzz combat scenarios
- Test serialization round-trips

**Integration tests:**
```
tests/
├── integration_basic.rs
├── integration_combat.rs
└── integration_progression.rs
```

### 5.3 Performance

**Benchmarks:**
- Use `criterion` crate
- Benchmark dice rolling (1M rolls)
- Benchmark combat simulation
- Optimize hot paths if needed

### 5.4 Documentation Site

**Use `mdBook`:**
```bash
mdbook/
├── src/
│   ├── SUMMARY.md
│   ├── introduction.md
│   ├── quickstart.md
│   ├── guide/
│   │   ├── characters.md
│   │   ├── dice.md
│   │   ├── combat.md
│   │   └── abilities.md
│   └── reference/
│       ├── api.md
│       └── mechanics.md
```

Deploy to GitHub Pages alongside rustdoc.

---

## 📋 Implementation Checklist

### Sprint 1: Cleanup (1-2 hours)
- [ ] Delete 17 process documentation files
- [ ] Move DEVELOPMENT.md → docs/CONTRIBUTING.md
- [ ] Move RESEARCH.md → docs/RESEARCH.md
- [ ] Rename MECHANICS_DEEP_DIVE.md → GAME_MECHANICS.md
- [ ] Create `docs/` directory structure

### Sprint 2: Documentation Rewrite (3-4 hours)
- [ ] Rewrite README.md (user-focused)
- [ ] Create EXAMPLES.md (gallery)
- [ ] Create docs/TUTORIAL.md (step-by-step)
- [ ] Create docs/API_GUIDE.md (module usage)
- [ ] Update QUICKSTART.md

### Sprint 3: Save/Load Examples (2-3 hours)
- [ ] Add `examples/save_and_load.rs`
- [ ] Add convenience save/load methods
- [ ] Document JSON format in docs/SAVE_FORMAT.md
- [ ] Update README with save/load example

### Sprint 4: CLI Tool (4-6 hours)
- [ ] Create `src/bin/daggerheart.rs`
- [ ] Implement `char` subcommand
- [ ] Implement `roll` subcommand
- [ ] Implement `combat` subcommand
- [ ] Add CLI documentation to README

### Sprint 5: REPL (3-4 hours)
- [ ] Add `rustyline` dependency
- [ ] Implement basic REPL loop
- [ ] Add character creation commands
- [ ] Add dice rolling commands
- [ ] Document REPL usage

### Sprint 6: Web Playground (6-8 hours)
- [ ] Set up WASM build
- [ ] Create basic HTML/CSS/JS frontend
- [ ] Integrate character creator
- [ ] Add dice roller UI
- [ ] Deploy to GitHub Pages

---

## 🎖️ Success Metrics

**After cleanup:**
- ✅ Repository has <10 documentation files at root
- ✅ All docs are user-facing (no process artifacts)
- ✅ New users can get started in <5 minutes

**After enhancements:**
- ✅ Users can experiment without writing code (CLI/REPL)
- ✅ Clear examples for every major feature
- ✅ Complete tutorial exists
- ✅ Web playground available

---

## 🗺️ Suggested Timeline

**Week 1:** Cleanup + Documentation Rewrite  
**Week 2:** Save/Load + CLI Tool  
**Week 3:** REPL + Web Playground  
**Week 4:** Polish, testing, final review

**Total effort:** ~20-30 hours

---

## 💡 Additional Ideas

### Community Features
- **Discord bot integration**: Run Daggerheart sessions in Discord
- **Roll20/Foundry VTT modules**: Integrate with existing platforms
- **Character sheet templates**: Printable PDFs generated from engine

### Game Master Tools
- **Encounter generator**: Auto-balance combat encounters
- **NPC generator**: Random NPCs with stats
- **Loot tables**: Random item/treasure generation

### Educational Content
- **Video tutorials**: YouTube series on using the engine
- **Blog posts**: Deep dives into implementation
- **Livestream**: Build a game using the engine

---

**Ready to execute! Which phase should we start with?** 🚀
