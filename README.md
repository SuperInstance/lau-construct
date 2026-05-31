# lau-construct

The Matrix Construct — everything is a room, every room has controls.

This is the shared creative space where user and agent co-create. The user describes what they want ("I need a training room with a warrior agent and gun-fu"), and the Construct materializes it as rooms, agents, hardware, and bridges — with an energy budget that enforces conservation.

## The concept in 60 seconds

A Construct starts as a **white room** — empty, infinite potential. You make requests in natural language, and the system parses them into typed intents (`NeedRooms`, `NeedAgents`, `LoadProgram`, etc.). Each request costs energy. You can render the result in four modes: Matrix (dashboard), MUD (text adventure), JSON (raw), or Voice (narrative only).

The lifecycle: **Empty → Loading → Ready → Testing → Active**. You can modify, reset, and redeploy at any time. Energy is conserved — you can't spend more than the pool.

## Quick start

```rust
use lau_construct::*;

// Start with a white room
let mut construct = Construct::new("Mission Control", "alice", "operator", 500.0);
assert_eq!(construct.state, ConstructState::Empty);

// Materialize rooms
let req = ConstructRequest::parse("I need rooms for engineering and training");
let results = construct.manifest(&req);
assert!(results.iter().all(|r| r.success));
assert_eq!(construct.rooms.len(), 2);

// Load an agent
let req = ConstructRequest::parse("I need a warrior agent");
construct.manifest(&req);

// Teach it a skill
let req = ConstructRequest::parse("I need gun-fu training");
construct.manifest(&req);

// Render it
let view = construct.walk_through("Alice");
println!("{}", view.render(ViewMode::Matrix));
// ═══ The Construct ═══
//   Rooms:
//     ■ Engineering [loaded] ...
//   Agents:
//     ◆ Warrior [warrior] ...

// Deploy it
let req = ConstructRequest::parse("deploy");
construct.manifest(&req);
assert_eq!(construct.state, ConstructState::Active);
assert!(construct.is_conserved());
```

## Key types

| Type | What it does |
|------|-------------|
| `Construct` | The shared space: rooms, agents, hardware, bridges, energy |
| `ConstructRequest` | Natural language request, parsed into typed intents |
| `ConstructRoom` | A room with type, size, contents, energy level |
| `ConstructAgent` | An agent with archetype, skills, equipment, level |
| `ConstructHardware` | Connected device: servo, sensor, camera, etc. |
| `ConstructBridge` | Connection to another instance |
| `ConstructView` | Rendered perspective of the construct |

## Pre-built programs

Load a complete Construct in one call:

```rust
// "I know gun-fu" — combat training
let combat = load_combat_program();  // Dojo + Arsenal + Arena, Neo & Trinity

// Engineering bay — hardware control
let eng = load_engineering_program(); // Lab + Motor Bay + Sensor Array

// Social — palaver rooms and diplomacy
let social = load_social_program();   // Palaver Room + Bridge Room

// Exploration — scouting and mapping
let explore = load_exploration_program(); // Cartography + Scout Den
```

Or load via manifest:

```rust
construct.manifest(&ConstructRequest::parse("load the combat program"));
```

## Render modes

```rust
let text = construct.render(ConstructRenderMode::Matrix);    // Dashboard
let mud  = construct.render(ConstructRenderMode::MUD);       // "You are Alice..."
let json = construct.render(ConstructRenderMode::JSON);       // Raw serde
let voice = construct.render(ConstructRenderMode::Voice);     // Narrative only
```

## Contributing

PRs welcome. This crate is part of the [SuperInstance](https://github.com/SuperInstance) ecosystem. The Construct is the heart of the PLATO system — contributions to room types, agent archetypes, and hardware models are especially welcome.
