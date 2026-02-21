# Test-Driven Development Plan

## TDD Philosophy for Daggerheart Engine

We'll follow strict TDD:
1. **Write failing test first** - Based on Daggerheart rules
2. **Implement minimal code** - Make test pass
3. **Refactor** - Clean up without breaking tests
4. **Repeat** - Build incrementally

---

## Phase 1: Dice System (TDD Breakdown)

### Sprint 1.1: Basic Die Type

#### Test 1: Die enum exists with correct variants
```rust
#[test]
fn die_types_exist() {
    let d4 = Die::D4;
    let d6 = Die::D6;
    let d8 = Die::D8;
    let d10 = Die::D10;
    let d12 = Die::D12;
    let d20 = Die::D20;
}
```
**Status**: ❌ Not implemented
**Next**: Create `src/core/dice.rs` with `Die` enum

---

#### Test 2: Die can be rolled with valid range
```rust
#[test]
fn d6_rolls_between_1_and_6() {
    let die = Die::D6;
    for _ in 0..100 {
        let result = die.roll();
        assert!(result >= 1 && result <= 6);
    }
}
```
**Status**: ❌ Not implemented  
**Next**: Implement `roll()` method using `rand`

---

#### Test 3: Die maximum value
```rust
#[test]
fn die_max_values() {
    assert_eq!(Die::D4.max(), 4);
    assert_eq!(Die::D6.max(), 6);
    assert_eq!(Die::D8.max(), 8);
    assert_eq!(Die::D10.max(), 10);
    assert_eq!(Die::D12.max(), 12);
    assert_eq!(Die::D20.max(), 20);
}
```
**Status**: ❌ Not implemented
**Next**: Add `max()` method

---

### Sprint 1.2: Duality Dice (2d12 Hope/Fear)

#### Test 4: DualityDice structure
```rust
#[test]
fn duality_dice_has_hope_and_fear() {
    let roll = DualityDice::roll();
    assert!(roll.hope >= 1 && roll.hope <= 12);
    assert!(roll.fear >= 1 && roll.fear <= 12);
}
```
**Status**: ❌ Not implemented
**Next**: Create `DualityDice` struct

---

#### Test 5: Critical success (verify exact rule)
```rust
#[test]
fn critical_success_both_max() {
    // Assuming 12+12 is critical success
    let roll = DualityDice::from_values(12, 12);
    assert!(roll.is_critical_success());
}
```
**Status**: ⏸️ **BLOCKED** - Need to verify exact rule
**Next**: Research what counts as critical success

---

#### Test 6: Critical failure (verify exact rule)
```rust
#[test]
fn critical_failure_both_min() {
    // Assuming 1+1 is critical failure
    let roll = DualityDice::from_values(1, 1);
    assert!(roll.is_critical_failure());
}
```
**Status**: ⏸️ **BLOCKED** - Need to verify exact rule

---

#### Test 7: Hope wins scenario
```rust
#[test]
fn hope_wins_when_higher() {
    let roll = DualityDice::from_values(10, 5);
    assert_eq!(roll.controlling_die(), ControllingDie::Hope);
    assert_eq!(roll.value(), 10);
}
```
**Status**: ❌ Not implemented
**Next**: Add logic to determine which die wins

---

#### Test 8: Fear wins scenario
```rust
#[test]
fn fear_wins_when_higher() {
    let roll = DualityDice::from_values(4, 9);
    assert_eq!(roll.controlling_die(), ControllingDie::Fear);
    assert_eq!(roll.value(), 9);
}
```
**Status**: ❌ Not implemented

---

#### Test 9: Tie handling (verify rule)
```rust
#[test]
fn tied_dice_resolution() {
    let roll = DualityDice::from_values(7, 7);
    // TODO: Verify what happens on a tie
    // Does Hope win? Fear? Player chooses?
}
```
**Status**: ⏸️ **BLOCKED** - Need rule clarification

---

### Sprint 1.3: Roll Modifiers

#### Test 10: Adding modifier to roll
```rust
#[test]
fn roll_with_positive_modifier() {
    let roll = DualityDice::from_values(8, 5);
    let modified = roll.with_modifier(2);
    // Does modifier apply to both dice? Just the higher? Just Hope?
    // TODO: Verify modifier application rules
}
```
**Status**: ⏸️ **BLOCKED** - Need modifier rules

---

#### Test 11: Advantage (verify exact mechanic)
```rust
#[test]
fn advantage_rolls_extra_die() {
    // Assuming advantage = roll 3 dice, take best 2
    let rolls = DualityDice::roll_with_advantage();
    // How exactly does advantage work?
}
```
**Status**: ⏸️ **BLOCKED** - Need advantage rules

---

### Sprint 1.4: Property-Based Testing

#### Test 12: Property - all rolls are valid
```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn all_d12_rolls_are_valid(seed in 0u64..1000) {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let roll = Die::D12.roll_with_rng(&mut rng);
            prop_assert!(roll >= 1 && roll <= 12);
        }
    }
}
```
**Status**: ❌ Not implemented
**Next**: Add after basic rolling works

---

## Current Blockers 🚧

### Critical Information Needed:

1. **Duality Dice Mechanics**
   - [ ] What constitutes a critical success?
   - [ ] What constitutes a critical failure?
   - [ ] How are ties resolved?
   - [ ] Do both dice get modifiers or just one?

2. **Advantage/Disadvantage**
   - [ ] How does advantage work? (extra dice?)
   - [ ] How does disadvantage work?
   - [ ] Can you have both (cancel out)?

3. **Modifier Application**
   - [ ] When are modifiers added? (before or after determining Hope/Fear?)
   - [ ] Do modifiers apply to both dice or just result?

---

## How to Unblock

### Option 1: Download Official Rules ⭐ RECOMMENDED
```bash
# Visit these sources:
- https://www.daggerheart.com/ (download section)
- https://drivethrurpg.com/ (search "Daggerheart")
- https://app.demiplane.com/ (may have rules reference)
```

### Option 2: Watch Official Videos
- Daggerheart 101 video (linked on website)
- Character creation example
- Take notes on exact mechanics shown

### Option 3: Community Research
- Check r/Daggerheart subreddit
- Discord server discussions
- Fan-made quick reference sheets

---

## Recommended Next Steps

1. **You**: Download the official playtest PDF
2. **Me**: Once you have it, I'll help extract mechanics
3. **Together**: Watch the Daggerheart 101 video, pause and document
4. **Me**: Write comprehensive test suite based on verified rules
5. **Me**: Implement dice system TDD style
6. **You**: Review and validate against actual play

---

## Git Workflow

After each successful test implementation:
```bash
git add .
git commit -m "test: add [feature] test"
# Implement code
git commit -m "feat: implement [feature]"
# Refactor
git commit -m "refactor: clean up [feature]"
```

---

## Progress Tracking

Create GitHub issues for each test?
- [ ] Issue #1: Implement Die enum with basic rolling
- [ ] Issue #2: Implement DualityDice structure
- [ ] Issue #3: Determine critical success/failure rules
- [ ] Issue #4: Implement advantage/disadvantage
- [ ] etc.

Or would you prefer a different tracking method?

---

## Ready When You Are! 🎲

I'm ready to help you:
1. Download and analyze the official rules
2. Watch the videos together and document mechanics
3. Write failing tests based on verified rules
4. Implement the code to make tests pass
5. Iterate rapidly with TDD

**What would you like to do first?**
