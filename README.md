# Daggerheart Rules Engine

A Rust implementation of the Daggerheart TTRPG rules system.

## Project Status

🚧 **Early Development** - Core architecture established, implementing dice system next.

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

## Features (Planned)

- ✅ Project structure
- ✅ Error handling foundation
- ⏳ Dice system (2d12 duality, damage dice)
- ⏳ Character attributes
- ⏳ Combat resolution
- ⏳ Hope/Fear mechanics
- ⏳ Domain cards
- ⏳ WASM compilation support

## Building

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Build release
cargo build --release

# Build for WASM (future)
cargo build --target wasm32-unknown-unknown --release
```

## Development

See `DEVELOPMENT.md` for the detailed development plan and roadmap.

## License

MIT OR Apache-2.0
