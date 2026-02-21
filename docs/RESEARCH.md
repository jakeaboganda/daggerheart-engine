# Daggerheart Rules Research

## Official Sources
- **Website**: https://www.daggerheart.com/
- **Publisher**: Darrington Press (Critical Role)
- **Lead Designer**: Spenser Starke  
- **Status**: Released May 2025, SRD 1.0 available
- **Wiki**: https://criticalrole.fandom.com/wiki/Daggerheart

---

## Core Mechanics (Verified & Documented)

### 1. Duality Dice System ⭐ **VERIFIED**
**Mechanics:** ✅ Confirmed from Critical Role Wiki[[1]](https://criticalrole.fandom.com/wiki/Daggerheart)

**How it works:**
- Roll **2d12** (one Hope die, one Fear die)  
- **Success condition**: Total >= target number (e.g., 12)
  - If total < target on both dice = **FAILURE**
  - If total >= target = **SUCCESS**

**Success Type (determined by higher die):**
- **Hope die higher** = Success with Hope
  - Player gains 1 Hope resource (spendable)
  - Positive narrative benefits
  - Players retain initiative in combat
  
- **Fear die higher** = Success with Fear  
  - GM gains 1 Fear resource
  - Can trigger narrative consequence
  - Initiative returns to enemies in combat

**Critical Success:**
- ✅ **Rolling a DOUBLE on the d12s** (any matching pair: 1+1, 2+2, ... 12+12)
- **Note**: Even 1+1 is a critical success if it's a double!

**Advantage:**
- ✅ Roll an **additional d6** (adds to total)

**Ties:**
- ⚠️ Not explicitly documented - assume Fear wins (needs verification)

**Modifiers:**
- Character Trait modifiers (+2, +1, +0, -1) add to roll total

**Implementation priority:** HIGH (foundation of entire system)

**Source:** Critical Role Wiki - Daggerheart article

---

### 2. Character Attributes **VERIFIED**
**Traits (6 total):** ✅ Confirmed from Critical Role Wiki

1. **Agility**
2. **Strength**  
3. **Finesse** (also called Precision)
4. **Instinct** (also called Intuition)
5. **Presence**
6. **Knowledge**

**Modifier Assignment:**
- Players assign these six modifiers in any order: **+2, +1, +1, +0, +0, -1**
- These add to ability checks and weapon attacks using that Trait

**Scaling:**
- ⚠️ Unknown if attributes increase with level (needs verification)

**Implementation priority:** HIGH

**Source:** Daggerheart SRD 1.0 via Critical Role Wiki

---

### 3. Classes **VERIFIED**
**All 9 Classes:** ✅ Confirmed from Critical Role Wiki

| Class | Domains | Subclasses | Description |
|-------|---------|------------|-------------|
| **Bard** | Codex & Grace | Wordsmith<br>Troubadour | Uses wordplay/music to captivate |
| **Druid** | Arcana & Sage | Warden of Elements<br>Warden of Renewal | Embodies nature, heals allies |
| **Guardian** | Blade & Valor | Stalwart<br>Vengeance | Tank/striker hybrid |
| **Ranger** | Bone & Sage | Wayfinder<br>Companion | Hunter with animal ally option |
| **Rogue** | Grace & Midnight | Nightwalker<br>Syndicate | Stealth/contacts specialist |
| **Seraph** | Splendor & Valor | Winged Sentinel<br>Divine Wielder | Flying warrior with legendary weapon |
| **Sorcerer** | Arcana & Midnight | Primal Origin<br>Elemental Origin | Raw/elemental magic caster |
| **Warrior** | Blade & Bone | Call of the Slayer<br>Call of the Brave | Power striker, uses enemy strength |
| **Wizard** | Codex & Splendor | School of Knowledge<br>School of War | Studied magic, knowledge/combat focus |

**Class Features:**
- Each class has 2 subclasses
- Each class focuses on 2 Domains
- Starting abilities determined by class
- Domain card selection from 2 class domains

**Implementation priority:** MEDIUM

**Source:** Daggerheart SRD 1.0 via Critical Role Wiki

---

### 4. Domains System **VERIFIED**
**9 Core Domains:** ✅ Confirmed from Critical Role Wiki

| Domain | Theme | Classes Using |
|--------|-------|---------------|
| **Arcana** | Instinctive magic | Druid, Sorcerer |
| **Blade** | Weaponry | Guardian, Warrior |
| **Bone** | Swiftness, coordination, tactics | Ranger, Warrior |
| **Codex** | Studied magic | Bard, Wizard |
| **Grace** | Charisma | Bard, Rogue |
| **Midnight** | Stealth | Rogue, Sorcerer |
| **Sage** | Nature magic | Druid, Ranger |
| **Splendor** | Life magic | Seraph, Wizard |
| **Valor** | Protection & defense | Guardian, Seraph |

**How Domains Work:**
- Each class focuses on **2 specific domains**
- Players select **domain cards** (abilities) from their 2 class domains
- Start with **2 domain cards** (one from each domain or both from one)
- Gain more cards as character levels up
- Can have up to **5 abilities active** at one time
- Abilities can be **swapped during rests**

**Implementation priority:** MEDIUM

**Source:** Daggerheart SRD 1.0 via Critical Role Wiki

---

### 5. Combat System **PARTIALLY VERIFIED**
**Core Mechanics:** ✅ Confirmed

**Initiative:**
- ⚠️ **No traditional initiative rolls!**
- Success with Hope = players retain initiative
- Success with Fear = initiative returns to enemies
- Fluid, narrative-driven turn order

**Action Economy:**
- ⚠️ Not fully documented yet - need specific action types
- Appears to use Major/Minor/Reaction actions (from initial planning)

**Attack Resolution:**
- Roll duality dice (2d12 Hope/Fear)
- Add weapon proficiency + Trait modifier
- Compare to target difficulty

**Damage Calculation:**  
- Damage dice: **d6, d8, d10, d12, d20** (varies by weapon)
- Roll damage dice
- **Subtract armor score** from damage total
- **Armor is damaged** when hit (can be repaired during short rest)

**Damage Thresholds:**
- If damage < threshold: Take **1 Stress** instead
- If damage >= threshold: Lose **1, 2, or 3 Hit Points** (based on how much it exceeds)

**HP System:**
- Starting HP: **6 Hit Points** (all characters!)
- Stress accumulates separately
- Low HP makes combat deadly and tactical

**Death Mechanics:** ✅ Confirmed
When HP reaches 0, player chooses:
1. **Blaze of Glory**: Perma-die but achieve critical success
2. **Avoid Death**: Permanently lose 1 Hope point maximum
3. **Risk It All**: Roll both Hope/Fear dice to potentially survive

**Resurrection:**
- **One single-use resurrection spell** per game
- Very limited - death is significant!

**Need to verify:**
- [ ] Exact action types (Major/Minor/Reaction details)
- [ ] Range system
- [ ] Movement rules
- [ ] Opportunity attacks
- [ ] Conditions application timing

**Implementation priority:** HIGH

**Source:** Critical Role Wiki - Daggerheart article

---

### 6. Hope & Fear Mechanics **VERIFIED**
**Core Resource System:** ✅ Confirmed from Critical Role Wiki

**Hope Resource:**
- **Gain Hope**: When you succeed with Hope (Hope die is higher)
- **Spend Hope for:**
  - +2 modifier to action rolls (if relevant to an Experience)
  - Special abilities and powers
  - Avoiding death (permanent cost of 1 max Hope)
  - Other class/ability-specific uses
- **Starting Hope**: ⚠️ Unknown (needs verification)
- **Maximum Hope**: ⚠️ Unknown (needs verification)
- **Halfling bonus**: Party starts each session with 1 Hope

**Fear Resource:**
- **GM gains Fear**: When players succeed with Fear (Fear die is higher)
- **GM spends Fear for:**
  - Triggering consequences
  - Activating enemy abilities
  - Making GM moves
  - Creating narrative complications
- **Fear pool**: ⚠️ Unknown if shared or per-encounter

**Balance:**
- System creates push-pull between success types
- Even successful rolls can give GM resources
- Encourages risk/reward decision-making

**Need to verify:**
- [ ] Starting Hope values
- [ ] Maximum Hope pool size
- [ ] Fear pool mechanics (shared? reset?)
- [ ] Complete list of Hope spending options
- [ ] Complete list of Fear spending/GM moves

**Implementation priority:** HIGH

**Source:** Critical Role Wiki - Daggerheart article

---

### 7. Domain Cards **PARTIALLY VERIFIED**
**Card System:** ✅ 279 cards total

**Card Selection:**
- Start with **2 cards** from your class's 2 domains
- Gain more as you level
- Maximum **5 active abilities** at any time
- Can swap abilities during rests

**Card Structure:** ⚠️ Need to verify
- [ ] Card cost (Hope? Action cost?)
- [ ] Card effect types
- [ ] Duration (instant, sustained, permanent?)
- [ ] Activation timing

**Implementation priority:** MEDIUM

**Source:** Critical Role Wiki, needs more detail

---

### 8. Ancestry System **VERIFIED**
**Available Ancestries:** ✅ Confirmed from Critical Role Wiki

| Ancestry | Description | Key Mechanic |
|----------|-------------|--------------|
| **Clank** | Mechanical humanoids | Purposeful Design (+1 to chosen Experience) |
| **Drakona** | Wingless dragon humanoids | Elemental Breath weapon |
| **Dwarf** | Classic fantasy dwarf | Increased Fortitude (spend 3 Hope for half damage) |
| **Elf** | Classic fantasy elf | Elven Trance (clear Stress during long rest) |
| **Faerie** | Winged fae | Flight (use Stress), Luckbender (reroll Duality Dice 1x/session) |
| **Faun** | Deer humanoids with antlers | Headbutt attack (use 1 Fear to damage enemy) |
| **Firbolg** | Gentle giants | Natural Calm (6 on d6 = no Stress) |
| **Fungril** | Fungus humanoids | Hivemind with other Fungril |
| **Galapa** | Turtle humanoids | Shell adds proficiency to armor |
| **Giant** | Large humanoids | Extra HP slot, increased melee reach |
| **Goblin** | Small cunning humanoids | Danger Sense (reroll attack 1x/short rest) |
| **Halfling** | Small folk | Lucky (reroll 1s on Hope), party gets 1 Hope at session start |
| **Human** | Adaptable humans | Perseverance (reroll failed Experience rolls with Hope) |
| **Infernis** | Descendants of fallen gods | Special abilities (source truncated) |

**Heritage Components:**
- **Ancestry** = race/species (above list)
- **Community** = cultural background (e.g., Highborne, Underborne)

**Implementation priority:** MEDIUM

**Source:** Critical Role Wiki - Daggerheart article

---

### 9. Character Creation **VERIFIED**
**Steps:** ✅ From Daggerheart SRD 1.0

1. **Choose Class & Subclass** - 9 classes, 2 subclasses each
2. **Choose Heritage** - Ancestry + Community
3. **Assign Character Traits** - Place +2, +1, +1, +0, +0, -1
4. **Set Evasion & HP** - Determined by class
5. **Choose Starting Inventory** - Weapon(s) + armor
6. **Create Background** - Narrative only, no mechanical effect
7. **Choose Experiences** - Player-created, provide +2 when spending Hope
8. **Choose Domain Cards** - 2 cards from class's 2 domains
9. **Create Connections** - Backstory relationships

**Implementation priority:** MEDIUM

**Source:** Daggerheart SRD 1.0 via Critical Role Wiki

---

### 10. Progression System **PARTIALLY VERIFIED**

**What we know:**
- "Designed for long-term campaign play"
- "Rich character progression"
- Gain new domain cards as you level
- Up to 5 abilities active at once

**Need to verify:**
- [ ] XP system vs milestone
- [ ] Level range (1-10? 1-20?)
- [ ] What changes per level (HP? Traits? Proficiency?)
- [ ] Rate of card acquisition

**Implementation priority:** LOW (can implement basic version first)

---

## Research Summary

### ✅ VERIFIED (Ready for Implementation)
- Duality dice mechanics (2d12 Hope/Fear)
- Critical success condition (doubles)
- Character traits (6 attributes with +2/+1/+1/0/0/-1 distribution)
- All 9 classes with subclasses
- 9 Domains and their associations
- 14+ ancestries with mechanics
- Basic combat flow (initiative, damage, death)
- Hope/Fear resource gain/spend basics
- Starting HP (6 for all characters)

### ⚠️ PARTIALLY VERIFIED (Need More Detail)
- Action economy specifics
- Domain card structure
- Hope/Fear pool limits
- Movement and range rules
- Conditions system
- Rest mechanics

### ❌ NOT YET VERIFIED
- Exact progression/leveling system
- Complete spell/ability lists
- Equipment tables
- GM Fear spending moves
- Condition list

---

## Next Steps

1. **Write Tests** - Now we have enough to write TDD tests for:
   - Duality dice rolling
   - Critical success detection
   - Character trait system
   - Basic combat damage
   - Hope/Fear resource tracking

2. **Implementation Order**:
   - Phase 1: Dice system (`src/core/dice.rs`)
   - Phase 2: Character traits (`src/character/attributes.rs`)
   - Phase 3: Basic combat (`src/combat/damage.rs`)
   - Phase 4: Hope/Fear (`src/core/resources.rs`)

3. **Download Official Materials** (Optional for more detail):
   - Character sheets PDF
   - Quickstart adventure
   - Full rulebook (if available)

---

Ready to start writing tests! 🎲
