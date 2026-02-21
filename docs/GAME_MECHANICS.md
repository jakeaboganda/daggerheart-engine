# Daggerheart Mechanics Deep Dive

## Complete Verified Mechanics Reference

### 🎲 Core Resolution System

#### Duality Dice (2d12)
**Roll Procedure:**
1. Roll 1 Hope die (d12) + 1 Fear die (d12)
2. Add modifiers (Trait + Proficiency + situational)
3. Compare total to target difficulty

**Success Types:**
- **Total < Target**: FAILURE
- **Total >= Target + Hope > Fear**: SUCCESS WITH HOPE
  - Gain 1 Hope point
  - Positive narrative outcome
  - Retain initiative (in combat)
- **Total >= Target + Fear > Hope**: SUCCESS WITH FEAR
  - GM gains 1 Fear point
  - Narrative complication
  - Initiative shifts to enemies (in combat)

**Critical Success:**
- **ANY DOUBLES** on the d12s (1+1, 2+2, ... 12+12)
- Grants extraordinary success regardless of total

**Advantage:**
- Roll an additional **d6** and add to total
- Does not affect Hope/Fear determination (only uses d12s)

**Tie Resolution:**
- When Hope die == Fear die value
- ⚠️ Not explicitly stated; assume Fear wins

---

###Character System

#### Attributes (Traits)
**Six Core Traits:**
1. **Agility** - Dexterity, reflexes, speed
2. **Strength** - Physical power
3. **Finesse** (Precision) - Fine motor control, accuracy
4. **Instinct** (Intuition) - Gut feelings, perception
5. **Presence** - Charisma, force of personality  
6. **Knowledge** - Intelligence, education

**Modifier Distribution:**
- Assign these six values in any order: **+2, +1, +1, +0, +0, -1**
- These modifiers add to relevant action rolls

**Usage:**
- Added to ability checks using that Trait
- Added to weapon attacks that utilize that Trait
- Does NOT increase with level (static)

---

#### Proficiency
- Determined by **weapon choice** during character creation
- Adds to attack rolls and specific checks
- ⚠️ Exact proficiency bonus values not documented
  - Likely starts at +1 or +2
  - May increase with level

---

#### Evasion
- **Defensive stat** (like AC in D&D)
- Determined by **Class**
- Each class has starting Evasion value
- Can be modified by:
  - Armor special properties (e.g., Faerie flight grants +2)
  - Ancestry abilities (e.g., Simiah +1)
  - Buffs and conditions

---

#### Experiences
- **Player-created narrative hooks**
- No mechanical effect by default
- **When spending Hope**: Can add +2 modifier to a relevant roll
  - Must narratively justify the Experience's relevance
  - Example: "I've fought in tavern brawls before" → +2 to bar fight

**Progression:**
- At certain levels, choose 2 Experiences
- Gain permanent +1 bonus to both selected Experiences

---

### ⚔️ Combat System

#### Hit Points & Damage
**HP System:**
- Everyone starts with **6 HP** (all classes!)
- Giants get **7 HP** (1 extra slot from ancestry)
- Very low HP makes combat deadly and tactical

**Damage Resolution:**
1. Attacker rolls weapon damage dice (d6 to d20)
2. **Subtract target's Armor Score** from damage
   - Armor takes damage (mark Armor Slot)
   - Armor can be repaired during short rest
3. Compare result to **damage threshold**:
   - **Below threshold**: Take **1 Stress** instead
   - **At/above threshold by 0-X**: Lose **1 HP**
   - **Above threshold by X-Y**: Lose **2 HP**
   - **Above threshold by Y+**: Lose **3 HP**

⚠️ Exact threshold values not documented

**Stress System:**
- Separate track from HP
- Accumulates from:
  - Low damage hits (below threshold)
  - Ancestry abilities (e.g., Faerie flight costs Stress)
  - Special actions
- Can be cleared during rests
- Affects Elven Trance mechanics

---

#### Death & Dying
**At 0 HP, choose ONE:**

1. **Blaze of Glory**
   - Character permanently dies
   - Final action becomes automatic critical success
   - Heroic last stand

2. **Avoid Death**
   - Survive but permanently lose 1 maximum Hope
   - Lasting consequence

3. **Risk It All**
   - Roll duality dice (2d12 Hope/Fear)
   - Outcome determines survival
   - ⚠️ Exact mechanics not fully documented

**Resurrection:**
- **ONE single-use resurrection spell per game**
- Extremely limited
- Death is meant to be significant

---

#### Initiative & Turn Order
**No traditional initiative rolls!**

**Turn Flow:**
- Success with Hope → Players keep initiative
- Success with Fear → Initiative passes to GM/enemies
- Fluid, narrative-driven combat
- No strict turn order

**Action Economy:**
⚠️ Specific action types not fully documented
- Likely: Major action, Minor action, Reaction
- Movement rules not detailed
- Need verification from SRD

---

#### Range Bands
**Mentioned in abilities:**
- **Very Close** - Melee range, adjacent
- **Close** - Short range
- **Far** - Long range
- ⚠️ Exact distances not specified

---

### 🎴 Domain Cards & Abilities

**Starting Cards:**
- Choose **2 domain cards** from your class's 2 domains
- Can choose both from one domain or split

**Maximum Active:**
- Up to **5 abilities active** at any time
- Can swap abilities during rests

**Card Structure:** ⚠️ Need verification
- Likely has: Name, Domain, Level requirement, Cost, Effect, Duration
- Cost may be: Hope, Stress, Action type, or resource-based

**Progression:**
- Gain new cards as you level up
- Some cards have level requirements (e.g., "Bone Level 9")

**Examples from SRD errata:**
- Splintering Strike (Bone 9): Multi-target attack, distribute damage
- Book of Grynn (Codex 4): Arcane Deflection, Wall of Flame

---

### 🧬 Complete Ancestry List

| Ancestry | Description | Abilities |
|----------|-------------|-----------|
| **Clank** | Mechanical humanoids | Purposeful Design (+1 to chosen Experience) |
| **Drakona** | Wingless dragon humanoids | Elemental Breath (weapon) |
| **Dwarf** | Stout, hardy folk | Increased Fortitude (3 Hope for half damage) |
| **Elf** | Graceful, long-lived | Elven Trance (clear Stress on long rest with d8 rolls) |
| **Faerie** | Winged fae creatures | Wings (fly with Stress, +2 Evasion)<br>Luckbender (reroll Duality 1x/session) |
| **Faun** | Deer humanoids | Headbutt (use 1 Fear to damage) |
| **Firbolg** | Gentle giants | Natural Calm (roll 6 on d6 = no Stress) |
| **Fungril** | Fungus humanoids | Always Connected (hivemind) |
| **Galapa** | Turtle humanoids | Shell of Protection (+Proficiency to Armor) |
| **Giant** | Towering humanoids | Endurance (+1 HP slot = 7 total)<br>Reach (melee range → Very Close) |
| **Goblin** | Small, cunning | Danger Sense (reroll attack 1x/short rest, reduce by Proficiency) |
| **Halfling** | Small, lucky folk | Little Lucky (reroll 1s on Hope die)<br>Party starts each session with 1 Hope |
| **Human** | Adaptable, diverse | Perseverance (reroll failed Experience with 1 Hope) |
| **Infernis** | Descendants of fallen gods | Fearless (mark Stress instead of GM gaining Fear)<br>Dread Visage (advantage on intimidation vs non-daemons) |
| **Katari** | Cat humanoids | Feline Instincts (1 Stress to reroll Hope on Agility) |
| **Orc** | Strong, resilient | Sturdy (roll d6 when using armor, 5-6 = no armor cost) |
| **Ribbet** | Frog humanoids | Amphibious (breathe/move underwater)<br>Long Tongue (grab, d12 damage weapon for 1 Stress) |
| **Simiah** | Ape/monkey humanoids | Nimble (+1 Evasion, advantage on balance/climbing) |

**Total: 17 Ancestries**

---

### 🏛️ Community
- Part of Heritage (Ancestry + Community)
- Examples: Highborne, Underborne
- ⚠️ Mechanical effects not documented

---

### 📊 Progression & Leveling

**What Happens on Level Up:**
- Gain new domain cards
- Choose 2 Experiences, gain +1 permanent bonus to both
- ⚠️ Unknown:
  - Total level range (1-10? 1-20?)
  - HP increases?
  - Proficiency increases?
  - Trait increases?
  - XP vs milestone

---

### 🛡️ Equipment & Weapons

**Weapon Tiers:**
- Tier 1, 2, 3, 4 documented in SRD errata
- Damage scales with tier

**Examples:**
- **Spear**
  - Tier 1: d8+3
  - Tier 2: d8+6
  - Tier 3: d8+9
  - Tier 4: d8+12
  - No Cumbersome feature

- **Longsword**
  - Tier 1: d10+3
  - Tier 2: d10+6
  - Tier 3: d10+9
  - Tier 4: d10+12

- **Glowing Rings**
  - Tier 1: d10+2 (magic weapon)

**Armor:**
- Has **Armor Slots** (track damage taken)
- Armor Score subtracts from incoming damage
- Damaged slots can be repaired during short rest
- Special armor: **Buckler** (Deflecting: mark slot for +Evasion bonus)

**Weapon Properties:**
- Burden (encumbrance)
- Cumbersome (penalties)
- Damage type: physical (phy), magic (mag), direct physical
- Range (Very Close, Close, Far)

---

### 🌙 Rest Mechanics

**Short Rest:**
- Repair armor slots
- Some abilities refresh (e.g., Goblin Danger Sense)
- ⚠️ Duration and exact benefits unclear

**Long Rest:**
- Elven Trance: Roll d8s = Stress count, clears all Stress, matching roll clears all HP
- Some abilities refresh (e.g., Faerie Luckbender)
- Swap domain cards
- **Sweet Moss**: Consume during rest to clear 1d10 HP or 1d10 Stress
- ⚠️ Standard HP/Stress recovery not documented

---

### 🎭 Conditions & Status Effects

**Documented Conditions:**
- **Restrained**: Lasts until freed (specific mechanics unclear)
- ⚠️ Full condition list not available

**Likely Conditions (to verify):**
- Prone, Stunned, Blinded, Frightened, etc.

---

### 💀 Adversary/Monster System

**Tiers:**
- Tier 1, 2, 3, 4 enemies
- Examples: Weaponmaster (T1), Stag Knight (T3), Volcanic Dragon (T4)

**Special Mechanics:**
- Adversaries within Very Close can trigger abilities
- Stages system for boss monsters (Fallen Warlord, Volcanic Dragon)

---

### 🧙 Spellcasting

**Spellcast Roll:**
- Target difficulty (e.g., 15)
- Uses duality dice
- Examples:
  - **Wall of Flame**: Spellcast 15, creates wall between two points (Far range), 4d10+3 magic damage

**Spell Types:**
- Instant effects (Wall of Flame)
- Defensive (Arcane Deflection)
- ⚠️ Full spell list not available

---

## 🚧 Remaining Unknowns

### High Priority
- [ ] Exact damage threshold values
- [ ] Proficiency starting value and progression
- [ ] Class-specific Evasion values
- [ ] Action economy details (Major/Minor/Reaction)
- [ ] Movement speed/rules
- [ ] Exact range band distances
- [ ] Hope/Fear starting and maximum values
- [ ] Rest HP/Stress recovery amounts

### Medium Priority
- [ ] Complete spell list
- [ ] Complete condition list
- [ ] Community mechanical effects
- [ ] Detailed weapon/armor tables
- [ ] Detailed domain card list
- [ ] Multi-classing rules
- [ ] Environmental hazards

### Low Priority
- [ ] Exact level progression (1-X)
- [ ] Complete adversary statblocks
- [ ] Campaign frame mechanics
- [ ] Downtime activities

---

## 📚 Sources
- Critical Role Wiki: https://criticalrole.fandom.com/wiki/Daggerheart
- Daggerheart SRD 1.0: https://www.daggerheart.com/srd/
- Official website: https://www.daggerheart.com/
- Verified through: SRD errata, wiki articles, official materials

**Last Updated:** 2026-02-21

---

## ✅ Ready for Implementation

We have enough verified mechanics to build:
1. **Core dice system** (2d12 duality, doubles for crits)
2. **Character attributes** (6 traits with modifiers)
3. **Basic combat** (6 HP, damage threshold system, Stress)
4. **Hope/Fear resources** (gain on success, spend for bonuses)
5. **Ancestry system** (17 ancestries with unique abilities)
6. **Class framework** (9 classes, 2 domains each)
7. **Domain cards** (start with 2, max 5 active)

The unknowns won't block initial implementation - we can use reasonable defaults and document them for future refinement!
