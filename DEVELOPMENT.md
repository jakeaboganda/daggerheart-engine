# Daggerheart Development Plan

## Project Overview

Building a Rust rules engine for Daggerheart TTRPG that can compile to:
- Native binaries (CLI tools, game servers)
- WASM (web frontends)
- Future: Python bindings, C FFI

## Current Status: Phase 0 Complete ✅

- [x] Project initialized
- [x] Dependencies configured
- [x] Module structure created
- [x] Error handling foundation
- [x] Documentation framework

## Phase 1: Core Dice System (Week 1-2)

### Goals
Implement the foundation of Daggerheart's unique dice mechanics.

### Tasks

#### 1.1 Basic Dice Types
- [ ] Create `src/core/dice.rs`
- [ ] Implement `Die` enum (D4, D6, D8, D10, D12, D20)
- [ ] Implement `roll()` method using `rand` crate
- [ ] Add unit tests for each die type

#### 1.2 Duality Dice (2d12)
- [ ] Create `DualityDice` struct
- [ ] Implement roll returning (hope_die, fear_die)
- [ ] Detect critical success (12 + 12)
- [ ] Detect critical failure (1 + 1)
- [ ] Determine which die is higher (Hope vs Fear)
- [ ] Add comprehensive tests

#### 1.3 Roll Modifiers
- [ ] Implement advantage (roll extra, take best)
- [ ] Implement disadvantage (roll extra, take worst)
- [ ] Create `RollResult` type with metadata
- [ ] Test edge cases (all same values, etc.)

#### 1.4 Damage Dice
- [ ] Create `DamageDice` struct
- [ ] Support multiple dice (e.g., 2d6 + 1d4)
- [ ] Implement damage roll with modifiers
- [ ] Add tests for damage calculation

### Deliverables
- Fully tested dice rolling system
- Property-based tests with `proptest`
- Documentation with examples
- Benchmarks for performance validation

### Acceptance Criteria
```rust
// Should be able to do:
use daggerheart_engine::core::dice::*;

let duality = DualityDice::roll();
assert!(duality.hope >= 1 && duality.hope <= 12);
assert!(duality.fear >= 1 && duality.fear <= 12);

let damage = DamageDice::new(vec![Die::D6, Die::D6]).roll();
assert!(damage.total >= 2 && damage.total <= 12);
```

---

## Phase 2: Character Foundation (Week 3-4)

### Goals
Create the character data model and attribute system.

### Tasks

#### 2.1 Attributes
- [ ] Create `src/character/attributes.rs`
- [ ] Define `Attributes` struct (Agility, Strength, Finesse, Instinct, Presence, Knowledge)
- [ ] Implement attribute modifiers (-3 to +5 range)
- [ ] Add validation and bounds checking

#### 2.2 Classes & Ancestries
- [ ] Create `src/character/classes.rs`
- [ ] Define `Class` enum (all 9 classes)
- [ ] Create `src/character/ancestry.rs`
- [ ] Define `Ancestry` enum
- [ ] Add trait modifiers

#### 2.3 Character Sheet
- [ ] Create `src/character/sheet.rs`
- [ ] Define `Character` struct
- [ ] Implement builder pattern for creation
- [ ] Add serialization support (save/load)

### Deliverables
- Complete character data model
- Character creation API
- Serialization to/from JSON
- Validation rules

---

## Phase 3: Combat System (Week 5-7)

### Tasks
- [ ] Action economy (major, minor, reaction)
- [ ] Attack resolution using duality dice
- [ ] Damage calculation with armor
- [ ] Stress and HP tracking
- [ ] Conditions system

---

## Phase 4: Hope/Fear & Advanced Mechanics (Week 8-10)

### Tasks
- [ ] Hope pool management
- [ ] Fear accumulation
- [ ] GM move triggers
- [ ] Domain cards system
- [ ] Spell mechanics

---

## Phase 5: Polish & Testing (Week 11-12)

### Tasks
- [ ] Comprehensive integration tests
- [ ] Performance benchmarks
- [ ] Documentation completion
- [ ] Example usage scenarios
- [ ] WASM compilation validation

---

## Next Immediate Steps

1. **Start with dice system** - Create `src/core/dice.rs`
2. **Write failing tests first** (TDD approach)
3. **Implement basic Die enum**
4. **Add DualityDice struct**
5. **Validate with property tests**

Ready to start Phase 1? 🎲
