# Save Format Documentation

The Daggerheart engine uses JSON for all save files, making them human-readable, editable, and version-control friendly.

## File Types

### Character File (`*_char.json`)

Contains the complete state of a combatant (player or enemy).

```json
{
  "name": "Grom the Mighty",
  "level": 3,
  "class": "Warrior",
  "ancestry": "Orc",
  "attributes": {
    "agility": 2,
    "strength": 1,
    "finesse": 1,
    "instinct": 0,
    "presence": 0,
    "knowledge": -1
  },
  "hp": {
    "current": 6,
    "maximum": 6
  },
  "stress": {
    "current": 0,
    "maximum": 5
  },
  "evasion": 10,
  "armor": 3,
  "initiative": 0,
  "is_player": true
}
```

**Fields:**
- `name`: Character name (string)
- `level`: Character level (1-10)
- `class`: Class name (see Classes below)
- `ancestry`: Ancestry name (see Ancestries below)
- `attributes`: Six attributes with modifiers (-1 to +2)
- `hp`: Current and maximum hit points
- `stress`: Current stress (0-5)
- `evasion`: Evasion score
- `armor`: Armor value
- `initiative`: Initiative roll (set during combat)
- `is_player`: Whether this is a player character

### Progress File (`*_progress.json`)

Tracks character progression across sessions.

```json
{
  "level": 2,
  "experience": 50,
  "available_cards": [
    "blade_strike",
    "shield_bash"
  ]
}
```

**Fields:**
- `level`: Current level (1-10)
- `experience`: Current XP
- `available_cards`: List of learned abilities/cards

**XP Requirements:**
- Level 1→2: 100 XP
- Level 2→3: 200 XP
- Level N→N+1: N × 100 XP

### Combat Encounter File (`*.json`)

Stores the complete state of a combat encounter.

```json
{
  "combatants": [
    { /* ... character data ... */ },
    { /* ... enemy data ... */ }
  ],
  "round": 1,
  "turn_order": [0, 2, 1],
  "current_turn": 0,
  "hope": {
    "current": 5,
    "maximum": 5
  },
  "fear": {
    "current": 0,
    "maximum": 10
  }
}
```

**Fields:**
- `combatants`: Array of combatant objects
- `round`: Current combat round (0 = not started)
- `turn_order`: Initiative order (indices into combatants array)
- `current_turn`: Current position in turn_order
- `hope`: Hope pool (shared by party)
- `fear`: Fear pool (GM resource)

## Classes

Valid class names:
- `Bard` - Charismatic performer
- `Druid` - Nature shapeshifter
- `Guardian` - Protective defender
- `Ranger` - Wilderness expert
- `Rogue` - Cunning trickster
- `Seraph` - Divine warrior
- `Sorcerer` - Arcane caster
- `Warrior` - Combat master
- `Wizard` - Scholarly mage

## Ancestries

Valid ancestry names:
- `Clank` (mechanical beings)
- `Daemon` (demonic heritage)
- `Drakona` (dragon-kin)
- `Dwarf`
- `Faerie` (fey folk)
- `Faun` (goat-like)
- `Fungril` (mushroom people)
- `Galapa` (turtle-folk)
- `Giant`
- `Goblin`
- `Halfling`
- `Human`
- `Inferis` (infernal)
- `Katari` (cat-folk)
- `Orc`
- `Ribbet` (frog-folk)
- `Simiah` (ape-like)

## Manual Editing

JSON files can be safely edited by hand. Common edits:

### Heal a character
```json
{
  "hp": {
    "current": 6,  // Set to maximum to full heal
    "maximum": 6
  }
}
```

### Add abilities
```json
{
  "available_cards": [
    "blade_strike",
    "new_ability"  // Add here
  ]
}
```

### Adjust XP
```json
{
  "experience": 150  // Set any value
}
```

### Change equipment
```json
{
  "armor": 5  // Update armor value
}
```

## Versioning & Compatibility

**Current version:** 0.1.0

**Forward compatibility:** Files from version 0.1.x should work with any 0.1.y release.

**Schema changes:** Major version bumps (1.0, 2.0) may change the file format. Migration tools will be provided.

## Backup & Version Control

### Git-friendly
JSON is text-based, so it works great with Git:

```bash
git add *.json
git commit -m "Session 5: Defeated the dragon"
```

### Backup strategy
```bash
# Automatic backups
mkdir -p backups
cp *_char.json *_progress.json backups/session_$(date +%Y%m%d)/
```

### Cloud sync
JSON files work well with:
- Google Drive
- Dropbox
- GitHub
- iCloud

## Common Workflows

### Starting a campaign
1. Create characters: `daggerheart char create ...`
2. Initialize progress files (auto-created)
3. Commit to Git: `git init && git add *.json && git commit -m "Campaign start"`

### Session workflow
1. Load character: `Combatant::load_from_file("char.json")`
2. Play session
3. Update progress: Add XP, level up, learn cards
4. Save all: `save_to_file()`
5. Commit: `git commit -am "Session N complete"`

### Campaign sharing
Share JSON files with other players:
```bash
# Player sends their character
scp grom_char.json grom_progress.json gm@server:/campaign/

# GM loads into encounter
daggerheart combat add encounter.json --character grom_char.json
```

## Troubleshooting

### "Invalid JSON" error
- Check for missing commas, braces
- Use `jq` to validate: `jq . character.json`
- Use an online JSON validator

### Missing fields
Old save files may need fields added:
```json
{
  "stress": {  // Add if missing
    "current": 0,
    "maximum": 5
  }
}
```

### Corrupted file
Restore from Git history:
```bash
git checkout HEAD^ character.json
```

## Advanced: Custom Parsers

You can parse save files in any language:

### Python
```python
import json

with open('character.json') as f:
    char = json.load(f)
    print(f"HP: {char['hp']['current']}/{char['hp']['maximum']}")
```

### JavaScript
```javascript
const char = require('./character.json');
console.log(`${char.name} (Level ${char.level})`);
```

### Rust
```rust
let char: Combatant = serde_json::from_str(&json)?;
```

---

**See also:**
- [QUICKSTART.md](QUICKSTART.md) - Getting started
- [EXAMPLES.md](EXAMPLES.md) - Code examples
- [API Documentation](https://jakeaboganda.github.io/daggerheart-engine/) - Full API reference
