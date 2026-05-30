# lau-construct

**The Matrix Construct** — the shared creative space where user and agent co-create PLATO environments.

Like Neo and Trinity loading weapons, the user vibe-codes what they need (via voice or text) and the Construct materializes it as PLATO rooms, agent capabilities, hardware controls, and game mechanics.

## Usage

```rust
use lau_construct::{Construct, ConstructRequest};

// Create the Construct (white room)
let mut c = Construct::new("The Construct", "neo", "operator", 500.0);

// "I need guns, lots of guns"
let results = c.manifest(&ConstructRequest::parse("load the combat training program"));
assert!(results[0].success);

// Walk through
let view = c.walk_through("Neo");

// Deploy
let results = c.manifest(&ConstructRequest::parse("deploy"));
assert!(results[0].success);
```

## Pre-built Programs

- `load_combat_program()` — Training rooms, combat agents, skill programs
- `load_engineering_program()` — Hardware control, motor agents, sensor arrays
- `load_social_program()` — Palaver rooms, bridges, diplomacy agents
- `load_exploration_program()` — Scout agents, mapping rooms, terrain generators

## Concepts

- **Construct** — The shared white room that fills with whatever you need
- **Manifest** — Materialize what the user asked for
- **Dematerialize** — Remove something from the Construct
- **Walk Through** — See what was built from a perspective
- **Deploy** — Take it live

## License

MIT
