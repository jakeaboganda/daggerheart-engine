# Daggerheart Engine - Implementation Plan

**Based on comprehensive mechanics research completed 2026-02-21**

---

## Implementation Philosophy

### Core Principles
1. **Test-Driven Development** - Write tests first, implement to pass
2. **Type Safety First** - Leverage Rust's type system to enforce game rules
3. **Incremental Delivery** - Each phase produces a usable, tested module
4. **Document as We Go** - Code comments + examples for every public API
5. **Performance Aware** - Benchmark critical paths, optimize later

### Rust Advantages
- **Enums with data** → Perfect for game state (DualityResult, DeathChoice)
- **Pattern matching** → Clean rule evaluation
- **Zero-cost abstractions** → Fast dice rolls, no GC pauses
- **Compile-time guarantees** → Can't exceed HP, invalid states impossible
- **`serde`** → Easy save/load for characters
- **WASM compilation** → Future web frontend ready

---

## Phase 1: Core Dice System (Week 1-2)
**Goal:** Rock-solid foundation for all rolls

### 1.1: Basic Dice (Days 1-2)

**Data Structures:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Die {
    pub fn roll(&self) -> u8;
    pub fn roll_with_rng<R: Rng>(&self, rng: &mut R) -> u8;
    pub fn max(&self) -> u8;
}
```

**Tests to Write:**
```rust
#[test]
fn test_d6_range() {
    for _ in 0..1000 {
        let result = Die::D6.roll();
        assert!(result >= 1 && result <= 6);
    }
}

#[test]
fn test_die_max_values() {
    assert_eq!(Die::D4.max(), 4);
    assert_eq!(Die::D6.max(), 6);
    // ...
}

// Property test
proptest! {
    #[test]
    fn die_rolls_are_valid(seed in 0u64..10000) {
        let mut rng = StdRng::seed_from_u64(seed);
        let die = Die::D12;
        let roll = die.roll_with_rng(&mut rng);
        prop_assert!(roll >= 1 && roll <= 12);
    }
}
```

**Deliverable:** `src/core/dice.rs` with full test coverage

---

### 1.2: Duality Dice (Days 3-5)

**Data Structures:**
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DualityRoll {
    pub hope: u8,      // 1-12
    pub fear: u8,      // 1-12
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllingDie {
    Hope,
    Fear,
    Tied,  // Handle tie case
}

#[derive(Debug, Clone, PartialEq)]
pub struct DualityResult {
    pub roll: DualityRoll,
    pub modifier: i8,
    pub advantage_die: Option<u8>,  // d6 if advantage
    pub total: u16,
    pub controlling: ControllingDie,
    pub is_critical: bool,  // Any doubles
}

impl DualityRoll {
    pub fn roll() -> Self;
    pub fn from_values(hope: u8, fear: u8) -> Self;
    
    pub fn with_modifier(self, modifier: i8) -> DualityResult;
    pub fn with_advantage(self, advantage_roll: u8) -> DualityResult;
    
    pub fn is_critical(&self) -> bool {
        self.hope == self.fear
    }
    
    pub fn controlling_die(&self) -> ControllingDie {
        match self.hope.cmp(&self.fear) {
            Ordering::Greater => ControllingDie::Hope,
            Ordering::Less => ControllingDie::Fear,
            Ordering::Equal => ControllingDie::Tied,
        }
    }
}

impl DualityResult {
    pub fn is_success(&self, difficulty: u16) -> bool {
        self.total >= difficulty
    }
    
    pub fn success_type(&self, difficulty: u16) -> Option<SuccessType>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuccessType {
    Failure,
    SuccessWithHope,
    SuccessWithFear,
    CriticalSuccess,
}
```

**Tests to Write:**
```rust
#[test]
fn test_critical_success_doubles() {
    let roll = DualityRoll::from_values(7, 7);
    assert!(roll.is_critical());
    
    let roll = DualityRoll::from_values(1, 1);
    assert!(roll.is_critical());
    
    let roll = DualityRoll::from_values(12, 12);
    assert!(roll.is_critical());
}

#[test]
fn test_hope_wins() {
    let roll = DualityRoll::from_values(10, 5);
    assert_eq!(roll.controlling_die(), ControllingDie::Hope);
}

#[test]
fn test_fear_wins() {
    let roll = DualityRoll::from_values(3, 9);
    assert_eq!(roll.controlling_die(), ControllingDie::Fear);
}

#[test]
fn test_tie_handling() {
    let roll = DualityRoll::from_values(8, 8);
    assert_eq!(roll.controlling_die(), ControllingDie::Tied);
    // Assume ties go to Fear for implementation
}

#[test]
fn test_advantage_adds_d6() {
    let roll = DualityRoll::from_values(5, 7);
    let result = roll.with_advantage(4);  // d6 rolled 4
    assert_eq!(result.total, 5 + 7 + 4);
}

#[test]
fn test_modifier_application() {
    let roll = DualityRoll::from_values(6, 4);
    let result = roll.with_modifier(2);  // +2 Strength
    assert_eq!(result.total, 6 + 4 + 2);
}

#[test]
fn test_success_with_hope() {
    let roll = DualityRoll::from_values(8, 5);  // Hope wins
    let result = roll.with_modifier(2);
    let success = result.success_type(12);  // DC 12
    // Total = 15, Hope > Fear
    assert_eq!(success, Some(SuccessType::SuccessWithHope));
}
```

**Deliverable:** Complete duality dice system with 95%+ test coverage

---

### 1.3: Damage Dice (Days 6-7)

**Data Structures:**
```rust
#[derive(Debug, Clone)]
pub struct DamageDice {
    dice: Vec<Die>,
    bonus: i16,
}

#[derive(Debug, Clone)]
pub struct DamageRoll {
    pub rolls: Vec<u8>,
    pub bonus: i16,
    pub total: u16,
}

impl DamageDice {
    pub fn new(dice: Vec<Die>) -> Self;
    pub fn with_bonus(mut self, bonus: i16) -> Self;
    pub fn roll(&self) -> DamageRoll;
    
    // Builder pattern
    pub fn d6(count: usize) -> Self;
    pub fn d8(count: usize) -> Self;
    // ...
}
```

**Tests:**
```rust
#[test]
fn test_damage_roll() {
    let damage = DamageDice::new(vec![Die::D6, Die::D6])
        .with_bonus(3);
    
    let roll = damage.roll();
    assert!(roll.total >= 2 + 3);  // min: 1+1+3
    assert!(roll.total <= 12 + 3); // max: 6+6+3
}

#[test]
fn test_weapon_damage_examples() {
    // Longsword Tier 1: d10+3
    let longsword = DamageDice::d10(1).with_bonus(3);
    // ...
}
```

**Deliverable:** Damage dice system for weapons

---

## Phase 2: Character System (Week 3-4)

### 2.1: Attributes (Days 8-10)

**Data Structures:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attributes {
    pub agility: i8,
    pub strength: i8,
    pub finesse: i8,
    pub instinct: i8,
    pub presence: i8,
    pub knowledge: i8,
}

impl Attributes {
    pub const STANDARD_MODIFIERS: [i8; 6] = [2, 1, 1, 0, 0, -1];
    
    pub fn new(
        agility: i8,
        strength: i8,
        finesse: i8,
        instinct: i8,
        presence: i8,
        knowledge: i8,
    ) -> Result<Self>;
    
    pub fn validate(&self) -> Result<()> {
        let mut mods = vec![
            self.agility, self.strength, self.finesse,
            self.instinct, self.presence, self.knowledge
        ];
        mods.sort();
        
        if mods == Self::STANDARD_MODIFIERS.to_vec() {
            Ok(())
        } else {
            Err(EngineError::InvalidCharacterState(
                "Attributes must use standard modifiers".into()
            ))
        }
    }
}
```

**Tests:**
```rust
#[test]
fn test_valid_attributes() {
    let attrs = Attributes::new(2, 1, 0, 1, 0, -1).unwrap();
    assert!(attrs.validate().is_ok());
}

#[test]
fn test_invalid_attributes() {
    // Wrong modifiers
    let attrs = Attributes::new(3, 2, 1, 0, 0, 0);
    assert!(attrs.validate().is_err());
}
```

**Deliverable:** Type-safe attribute system with validation

---

### 2.2: Classes & Domains (Days 11-13)

**Data Structures:**
```rust
use strum_macros::{EnumIter, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, Serialize, Deserialize)]
pub enum Class {
    Bard,
    Druid,
    Guardian,
    Ranger,
    Rogue,
    Seraph,
    Sorcerer,
    Warrior,
    Wizard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum Subclass {
    // Bard
    Wordsmith,
    Troubadour,
    
    // Druid
    WardenOfElements,
    WardenOfRenewal,
    
    // ... all 18 subclasses
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum Domain {
    Arcana,
    Blade,
    Bone,
    Codex,
    Grace,
    Midnight,
    Sage,
    Splendor,
    Valor,
}

impl Class {
    pub fn domains(&self) -> (Domain, Domain) {
        match self {
            Class::Bard => (Domain::Codex, Domain::Grace),
            Class::Druid => (Domain::Arcana, Domain::Sage),
            // ...
        }
    }
    
    pub fn starting_evasion(&self) -> u8 {
        // TODO: Need values from SRD
        match self {
            Class::Guardian => 10,  // Placeholder
            Class::Rogue => 14,     // Placeholder
            // ...
        }
    }
    
    pub fn starting_hp(&self) -> u8 {
        6  // All classes start with 6 HP
    }
}
```

**Tests:**
```rust
#[test]
fn test_class_domains() {
    assert_eq!(Class::Bard.domains(), (Domain::Codex, Domain::Grace));
    assert_eq!(Class::Warrior.domains(), (Domain::Blade, Domain::Bone));
}

#[test]
fn test_all_classes_have_6_hp() {
    use strum::IntoEnumIterator;
    for class in Class::iter() {
        assert_eq!(class.starting_hp(), 6);
    }
}
```

**Deliverable:** Complete class/domain enumeration

---

### 2.3: Ancestries (Days 14-16)

**Data Structures:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display)]
pub enum Ancestry {
    Clank,
    Drakona,
    Dwarf,
    Elf,
    Faerie,
    Faun,
    Firbolg,
    Fungril,
    Galapa,
    Giant,
    Goblin,
    Halfling,
    Human,
    Infernis,
    Katari,
    Orc,
    Ribbet,
    Simiah,
}

pub trait AncestryAbility {
    fn apply_to_character(&self, character: &mut Character);
}

impl Ancestry {
    pub fn hp_bonus(&self) -> u8 {
        match self {
            Ancestry::Giant => 1,  // 7 HP instead of 6
            _ => 0,
        }
    }
    
    pub fn evasion_bonus(&self) -> i8 {
        match self {
            Ancestry::Simiah => 1,  // Nimble
            _ => 0,
        }
    }
    
    // TODO: Implement specific abilities as methods or traits
}
```

**Tests:**
```rust
#[test]
fn test_giant_hp_bonus() {
    assert_eq!(Ancestry::Giant.hp_bonus(), 1);
}

#[test]
fn test_ancestry_count() {
    use strum::IntoEnumIterator;
    assert_eq!(Ancestry::iter().count(), 18);
}
```

**Deliverable:** Complete ancestry system

---

### 2.4: Character Sheet (Days 17-18)

**Data Structures:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    // Identity
    pub name: String,
    pub class: Class,
    pub subclass: Subclass,
    pub ancestry: Ancestry,
    pub level: u8,
    
    // Attributes
    pub attributes: Attributes,
    
    // Combat stats
    pub max_hp: u8,
    pub current_hp: u8,
    pub stress: u8,
    pub evasion: u8,
    pub proficiency: i8,
    
    // Resources
    pub max_hope: u8,
    pub current_hope: u8,
    
    // Abilities
    pub domain_cards: Vec<DomainCard>,
    pub experiences: Vec<Experience>,
    
    // Equipment
    pub armor_score: u8,
    pub armor_slots: u8,
    pub armor_slots_used: u8,
}

impl Character {
    pub fn builder() -> CharacterBuilder;
    
    pub fn take_damage(&mut self, damage: u16, armor_threshold: u16) -> DamageResult;
    pub fn take_stress(&mut self);
    pub fn heal_hp(&mut self, amount: u8);
    pub fn clear_stress(&mut self, amount: u8);
    
    pub fn gain_hope(&mut self);
    pub fn spend_hope(&mut self, amount: u8) -> Result<()>;
    
    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }
}

pub struct CharacterBuilder {
    // Builder pattern for character creation
}
```

**Tests:**
```rust
#[test]
fn test_character_creation() {
    let character = Character::builder()
        .name("Bertrand Bell")
        .class(Class::Warrior)
        .subclass(Subclass::CallOfTheBrave)
        .ancestry(Ancestry::Human)
        .attributes(Attributes::new(2, 1, 0, 1, 0, -1).unwrap())
        .build()
        .unwrap();
    
    assert_eq!(character.max_hp, 6);
    assert_eq!(character.current_hp, 6);
}

#[test]
fn test_giant_has_7_hp() {
    let character = Character::builder()
        .class(Class::Guardian)
        .ancestry(Ancestry::Giant)
        .build()
        .unwrap();
    
    assert_eq!(character.max_hp, 7);
}
```

**Deliverable:** Complete character data model with serialization

---

## Phase 3: Combat System (Week 5-6)

### 3.1: Damage Resolution (Days 19-21)

**Data Structures:**
```rust
#[derive(Debug, Clone)]
pub struct DamageResult {
    pub raw_damage: u16,
    pub after_armor: u16,
    pub hp_lost: u8,
    pub stress_gained: u8,
    pub armor_damaged: bool,
}

impl Character {
    pub fn take_damage(&mut self, raw_damage: u16) -> DamageResult {
        let after_armor = raw_damage.saturating_sub(self.armor_score as u16);
        
        // TODO: Need exact threshold values from SRD
        let threshold = 5;  // Placeholder
        
        let (hp_lost, stress_gained) = if after_armor < threshold {
            (0, 1)  // Below threshold = 1 Stress
        } else {
            let excess = after_armor - threshold;
            let hp = match excess {
                0..=4 => 1,
                5..=9 => 2,
                _ => 3,
            };
            (hp, 0)
        };
        
        self.current_hp = self.current_hp.saturating_sub(hp_lost);
        self.stress += stress_gained;
        
        // Damage armor
        let armor_damaged = if raw_damage > 0 && self.armor_slots_used < self.armor_slots {
            self.armor_slots_used += 1;
            true
        } else {
            false
        };
        
        DamageResult {
            raw_damage,
            after_armor,
            hp_lost,
            stress_gained,
            armor_damaged,
        }
    }
}
```

**Tests:**
```rust
#[test]
fn test_damage_below_threshold_gives_stress() {
    let mut character = create_test_character();
    character.armor_score = 5;
    
    let result = character.take_damage(7);  // 7 - 5 = 2 (below threshold)
    
    assert_eq!(result.hp_lost, 0);
    assert_eq!(result.stress_gained, 1);
}

#[test]
fn test_damage_above_threshold() {
    let mut character = create_test_character();
    character.armor_score = 2;
    
    let result = character.take_damage(10);  // 10 - 2 = 8 (above threshold)
    
    assert!(result.hp_lost > 0);
    assert_eq!(result.stress_gained, 0);
}
```

**Deliverable:** Complete damage system

---

### 3.2: Death Mechanics (Days 22-23)

**Data Structures:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathChoice {
    BlazeOfGlory,
    AvoidDeath,
    RiskItAll,
}

#[derive(Debug, Clone)]
pub enum DeathResult {
    BlazeOfGlory { critical_action: String },
    Survived { hope_cost: u8 },
    RiskItAllSuccess { narrative: String },
    RiskItAllFailure { narrative: String },
}

impl Character {
    pub fn handle_death(&mut self, choice: DeathChoice) -> DeathResult {
        match choice {
            DeathChoice::BlazeOfGlory => {
                DeathResult::BlazeOfGlory {
                    critical_action: "Final heroic act".into()
                }
            },
            DeathChoice::AvoidDeath => {
                self.max_hope = self.max_hope.saturating_sub(1);
                self.current_hp = 1;
                DeathResult::Survived { hope_cost: 1 }
            },
            DeathChoice::RiskItAll => {
                let roll = DualityRoll::roll();
                // TODO: Define exact Risk It All mechanics
                if roll.hope > roll.fear {
                    self.current_hp = 1;
                    DeathResult::RiskItAllSuccess { narrative: "...".into() }
                } else {
                    DeathResult::RiskItAllFailure { narrative: "...".into() }
                }
            }
        }
    }
}
```

**Tests:**
```rust
#[test]
fn test_avoid_death_costs_hope() {
    let mut character = create_test_character();
    character.max_hope = 5;
    character.current_hp = 0;
    
    character.handle_death(DeathChoice::AvoidDeath);
    
    assert_eq!(character.max_hope, 4);
    assert!(character.is_alive());
}
```

**Deliverable:** Death mechanic system

---

### 3.3: Hope & Fear Resources (Days 24-25)

**Data Structures:**
```rust
#[derive(Debug, Clone)]
pub struct HopePool {
    pub current: u8,
    pub maximum: u8,
}

impl HopePool {
    pub fn gain(&mut self) {
        self.current = (self.current + 1).min(self.maximum);
    }
    
    pub fn spend(&mut self, amount: u8) -> Result<()> {
        if self.current >= amount {
            self.current -= amount;
            Ok(())
        } else {
            Err(EngineError::ResourceExceeded(
                format!("Not enough Hope: have {}, need {}", self.current, amount)
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct FearPool {
    pub current: u8,
}

impl FearPool {
    pub fn gain(&mut self) {
        self.current += 1;
    }
    
    pub fn spend(&mut self, amount: u8) -> Result<()> {
        if self.current >= amount {
            self.current -= amount;
            Ok(())
        } else {
            Err(EngineError::ResourceExceeded("Not enough Fear".into()))
        }
    }
}
```

**Tests:**
```rust
#[test]
fn test_hope_gain_and_spend() {
    let mut pool = HopePool { current: 3, maximum: 5 };
    
    pool.gain();
    assert_eq!(pool.current, 4);
    
    pool.spend(2).unwrap();
    assert_eq!(pool.current, 2);
}
```

**Deliverable:** Resource management system

---

## Phase 4: Domain Cards & Abilities (Week 7-8)

### 4.1: Card Framework (Days 26-28)

**Data Structures:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainCard {
    pub id: String,
    pub name: String,
    pub domain: Domain,
    pub level_requirement: u8,
    pub description: String,
    pub effect: CardEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardEffect {
    // Placeholder - will be expanded
    Attack { damage: DamageDice, range: Range },
    Buff { target: Target, bonus: i8, duration: Duration },
    Heal { amount: u8 },
    // ... many more
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Range {
    VeryClose,
    Close,
    Far,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    Instant,
    UntilNextRest,
    Permanent,
}
```

**Deliverable:** Card framework (implementation details TBD based on SRD)

---

## Phase 5: Integration & Polish (Week 9-10)

### 5.1: Combat Simulation (Days 29-32)
- Full combat round implementation
- Initiative flow (Hope/Fear based)
- Action resolution

### 5.2: Character Progression (Days 33-35)
- Leveling system
- Card acquisition
- Experience progression

### 5.3: Save/Load System (Days 36-38)
- JSON serialization
- Character import/export
- Campaign state management

### 5.4: Examples & Documentation (Days 39-40)
- Usage examples
- API documentation
- Tutorial scenarios

---

## Testing Strategy

### Unit Tests
- Every public function
- Edge cases (0 HP, max values, etc.)
- Error conditions

### Property Tests (proptest)
- Dice rolls always in valid range
- Character state transitions valid
- Resource pools never negative

### Integration Tests
- Full combat scenarios
- Character creation flows
- Save/load round-trips

### Benchmarks
- Dice rolling performance
- Damage calculation
- Character serialization

---

## Success Metrics

### Phase 1 Complete When:
- [ ] Can roll all dice types
- [ ] Duality dice fully implemented
- [ ] Critical detection works
- [ ] 100% test coverage on dice

### Phase 2 Complete When:
- [ ] Can create valid characters
- [ ] All 9 classes defined
- [ ] All 18 ancestries implemented
- [ ] Character serialization works

### Phase 3 Complete When:
- [ ] Damage calculation matches rules
- [ ] Death mechanics implemented
- [ ] Hope/Fear pools functional

### Phase 4 Complete When:
- [ ] Domain card framework exists
- [ ] Can model basic abilities

### Phase 5 Complete When:
- [ ] Full combat simulation runs
- [ ] Documentation complete
- [ ] Example scenarios work

---

## Git Workflow

### Commit Convention:
```
type(scope): description

Types:
- test: Add or modify tests
- feat: New feature implementation
- refactor: Code cleanup without behavior change
- docs: Documentation updates
- fix: Bug fix
```

### Branch Strategy:
- `master` - always compiles, tests pass
- `feat/dice-system` - feature branches
- `feat/character-system`
- etc.

### PR Process:
1. Write failing tests
2. Implement feature
3. All tests pass
4. Code review (self or pair)
5. Merge to master

---

## Next Immediate Actions

1. **Create `src/core/dice.rs`** - Start with Die enum
2. **Write first test** - `test_d6_rolls_valid_range()`
3. **Run `cargo test --lib`** - See it fail (RED)
4. **Implement Die::roll()** - Make it pass (GREEN)
5. **Refactor** - Clean up if needed
6. **Commit** - `test: add basic die rolling tests`
7. **Repeat** - Next test!

---

**Ready to start coding! Let's begin with Phase 1.1: Basic Dice** 🎲
