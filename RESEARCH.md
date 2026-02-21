# Daggerheart Rules Research

## Official Sources
- **Website**: https://www.daggerheart.com/
- **Publisher**: Darrington Press (Critical Role)
- **Lead Designer**: Spenser Starke
- **Status**: Open playtest (2024-2025), full release 2025

## Core Mechanics (To Verify & Implement)

### 1. Duality Dice System ⭐
**What we know:**
- Uses 2d12 (one Hope die, one Fear die)
- Roll both simultaneously
- Higher die determines success + narrative control
- Hope = player advantage, Fear = GM gets to make a move

**Need to verify:**
- [ ] Exact critical success conditions (12+12?)
- [ ] Exact critical failure conditions (1+1?)
- [ ] How ties are resolved (equal Hope/Fear)
- [ ] Modifier application (before or after roll?)
- [ ] Advantage/disadvantage mechanics

**Implementation priority:** HIGH (foundation of entire system)

---

### 2. Character Attributes
**What we know:**
- Six core attributes
- Likely: Agility, Strength, Finesse, Instinct, Presence, Knowledge
- Modifiers affect rolls

**Need to verify:**
- [ ] Exact attribute names
- [ ] Modifier range (e.g., -3 to +5?)
- [ ] How attributes are determined at character creation
- [ ] Do attributes scale with level?

**Implementation priority:** HIGH

---

### 3. Classes
**What we know:**
- 9 classes mentioned in planning
- Card-based abilities (279 cards total)

**Need to verify:**
- [ ] Complete list of 9 classes
- [ ] Class starting abilities
- [ ] Progression structure
- [ ] Domain card system per class

**Classes to research:**
- Warrior
- Ranger  
- Wizard/Mage
- Rogue
- Cleric/Healer
- [Need to find other 4]

**Implementation priority:** MEDIUM

---

### 4. Combat System
**What we know:**
- Card-based abilities
- Hope/Fear mechanic influences narrative
- Tactical depth mentioned

**Need to verify:**
- [ ] Initiative system (if any)
- [ ] Action economy (actions per turn)
- [ ] Attack roll resolution
- [ ] Damage calculation
- [ ] Armor/defense system
- [ ] HP/Stress/Wounds tracking
- [ ] Death and recovery mechanics

**Implementation priority:** HIGH

---

### 5. Hope & Fear Mechanics
**What we know:**
- Central to the game's design
- Affects narrative control
- "Every choice matters as adventure hangs in balance"

**Need to verify:**
- [ ] How Hope accumulates
- [ ] How Fear accumulates
- [ ] What can players spend Hope on?
- [ ] What does GM do with Fear?
- [ ] Pool sizes and limits
- [ ] Reset conditions

**Implementation priority:** HIGH

---

### 6. Domain Cards
**What we know:**
- 279 beautifully illustrated cards
- Core to character abilities
- Related to classes/specializations

**Need to verify:**
- [ ] Card structure (cost, effect, duration)
- [ ] How cards are acquired
- [ ] Deck building rules
- [ ] Card activation timing
- [ ] Card categories/types

**Implementation priority:** MEDIUM

---

### 7. Ancestry System
**Need to verify:**
- [ ] Available ancestries
- [ ] Ancestry traits/bonuses
- [ ] Heritage/community mechanics

**Implementation priority:** MEDIUM

---

### 8. Damage & Healing
**Need to verify:**
- [ ] Damage die types (d4, d6, d8, d10, d20?)
- [ ] How damage is calculated
- [ ] Armor/resistance system
- [ ] Healing mechanics
- [ ] Rest and recovery rules

**Implementation priority:** HIGH

---

### 9. Conditions & Status Effects
**Need to verify:**
- [ ] List of conditions
- [ ] How conditions are applied
- [ ] Condition durations
- [ ] Removal/save mechanics

**Implementation priority:** MEDIUM

---

### 10. Progression System
**What we know:**
- "Designed for long-term campaign play"
- "Rich character progression"

**Need to verify:**
- [ ] XP system
- [ ] Level range (1-10? 1-20?)
- [ ] What changes per level
- [ ] Milestone vs XP progression

**Implementation priority:** LOW (can implement basic version first)

---

## Resources Needed

### Official Materials
- [ ] **Playtest PDF** - Need to download from daggerheart.com or DriveThruRPG
- [ ] **Character Sheet** - Reference for data structure
- [ ] **FAQ** - Already accessed, but need more specific Q&A
- [ ] **Daggerheart 101 Video** - Watch for mechanics overview
- [ ] **Character Creation Video** - Watch Mercer/Willingham demo

### Community Resources
- [ ] Reddit discussions (r/Daggerheart likely exists)
- [ ] Discord community feedback
- [ ] Fan-made reference sheets
- [ ] Actual play recordings

### Digital Tools
- [ ] Demiplane character creator (may reveal data structures)
- [ ] Check if there's a System Reference Document (SRD)

---

## Next Steps

1. **Download official playtest materials**
   - Visit https://www.daggerheart.com/ 
   - Download PDFs from DriveThruRPG
   - Extract core mechanics

2. **Watch official videos**
   - Daggerheart 101 (mechanics overview)
   - Character creation example

3. **Create detailed mechanics doc**
   - Document exact rules
   - Note edge cases
   - Identify ambiguities

4. **Build test cases**
   - Write tests based on official examples
   - Implement TDD approach
   - Validate against playtest scenarios

---

## TDD Approach

Once we have verified rules:

```rust
// Example test-first approach
#[test]
fn duality_dice_critical_success() {
    // Given: Both dice roll 12
    let result = DualityDice::from_values(12, 12);
    
    // Then: Should be critical success
    assert!(result.is_critical_success());
    assert_eq!(result.total(), 24);
}

#[test]
fn duality_dice_hope_wins() {
    // Given: Hope die is higher
    let result = DualityDice::from_values(10, 5);
    
    // Then: Hope should win
    assert!(result.hope_wins());
    assert_eq!(result.highest_die(), 10);
}
```

---

## Questions for User

1. Do you have access to the playtest materials already?
2. Should I help you download them?
3. Which mechanic should we focus on first for implementation?
4. Do you want to watch the videos together and take notes?
