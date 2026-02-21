# Daggerheart Rules Engine

[![CI](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/ci.yml)
[![Documentation](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/docs.yml/badge.svg)](https://github.com/jakeaboganda/daggerheart-engine/actions/workflows/docs.yml)

A Rust implementation of the Daggerheart TTRPG rules system.

## Project Status

✅ **Phase 1 Complete** - Core dice system implemented with comprehensive tests and examples.

## Architecture

```
daggerheart-engine/
├── src/
│   ├── core/           # Dice, rolls, Hope/Fear mechanics
│   ├── character/      # Character creation, progression
│   ├── combat/         # Combat actions, resolution
│   ├── cards/          # Domain cards system
│   ├── items/          # Equipment, inventory
│   └── error.rs        # Error types
```

## Features

### Implemented ✅
- ✅ Basic dice system (d4, d6, d8, d10, d12, d20)
- ✅ Duality dice (2d12 Hope/Fear system)
- ✅ Damage dice with bonuses
- ✅ Critical success detection (doubles)
- ✅ Advantage mechanics
- ✅ Comprehensive test suite (62 tests)
- ✅ Property-based testing
- ✅ Usage examples
- ✅ CI/CD pipeline

### Planned ⏳
- ⏳ Character attributes system
- ⏳ Classes and ancestries
- ⏳ Combat resolution system
- ⏳ Hope/Fear pool management
- ⏳ Domain cards
- ⏳ Items and equipment
- ⏳ WASM compilation support

## Building

```bash
# Check compilation
cargo check

# Run tests
cargo test --lib

# Run specific test module
cargo test --lib core::dice::duality

# Build release
cargo build --release

# Build for WASM (future)
cargo build --target wasm32-unknown-unknown --release
```

## Examples

The project includes comprehensive usage examples:

**Dice System (Phase 1):**
```bash
# Basic dice rolling
cargo run --example basic_dice

# Hope/Fear duality system
cargo run --example duality_dice

# Weapon damage mechanics
cargo run --example weapon_damage

# Full combat encounter
cargo run --example combat_scenario
```

**Character System (Phase 2):**
```bash
# Character attributes
cargo run --example character_attributes

# Classes and domains
cargo run --example character_classes

# Ancestries and traits
cargo run --example character_ancestries

# Complete character creation
cargo run --example character_creation
```

## Documentation

- **[API Documentation](https://jakeaboganda.github.io/daggerheart-engine/)** - Auto-generated from code
- **[Implementation Plan](IMPLEMENTATION_PLAN.md)** - 10-week development roadmap
- **[Mechanics Deep Dive](MECHANICS_DEEP_DIVE.md)** - Complete Daggerheart mechanics reference
- **[Phase 1 Review](PHASE_1_REVIEW.md)** - Dice system review and testing strategy
- **[Quick Start](QUICKSTART.md)** - Project navigation guide

## Development

See `DEVELOPMENT.md` for the detailed development plan and roadmap.

## License

MIT OR Apache-2.0
