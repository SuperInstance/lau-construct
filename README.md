# lau-construct

**The Matrix Construct — a shared creative space where user and agent co-create PLATO environments.**

A Rust library that models "The Construct" from *The Matrix*: an infinite white room where you materialize rooms, agents, hardware, bridges, and skills by describing what you need in natural language. Like Neo loading weapons, you vibe-code what you want and the Construct materializes it.

---

## What This Does

`lau-construct` provides a complete simulation of a co-creative environment:

- **Natural-language requests** — parse what the user wants ("I need rooms for engineering") into structured intents
- **Materialization** — create rooms, agents, hardware, and bridges on the fly with energy costs
- **Lifecycle management** — white room → loading → ready → testing → modifying → deploying → active
- **Modification** — resize rooms, teach skills, equip agents
- **Multiple render modes** — Matrix-style, MUD-style, dashboard, JSON, narrative, voice
- **Pre-built programs** — combat training, engineering bay, social palaver, exploration guild
- **Energy budget** — every creation costs energy; the system tracks conservation

---

## Key Idea

The Construct is a state machine shaped like the white room from *The Matrix*:

```
Empty → Loading → Ready → Testing → Active
                  ↕
               Modifying
                  
         (Deploying)
```

You speak, the Construct materializes. You say "deploy", it goes live. You say "reset", the white room returns. Every operation costs energy, and the system verifies that `energy_spent ≤ energy_pool`.

---

## Install

```toml
[dependencies]
lau-construct = "0.1"
```

Or:

```sh
cargo add lau-construct
```

### Requirements

- Rust 2021 edition or later
- `serde` + `serde_json` (transitive)

---

## Quick Start

```rust
use lau_construct::{Construct, ConstructRequest, ConstructRenderMode};

fn main() {
    // 1. Create a Construct (the white room)
    let mut construct = Construct::new(
        "The Construct",
        "neo",       // creator
        "operator",  // operator
        500.0,       // energy budget
    );

    // 2. Say what you need
    let request = ConstructRequest::parse("I need rooms for engineering and training");
    let results = construct.manifest(&request);
    for r in &results {
        println!("{}", r.message);
    }

    // 3. Load an agent
    let agent_req = ConstructRequest::parse("I need a warrior agent");
    construct.manifest(&agent_req);

    // 4. Teach it skills
    let skill_req = ConstructRequest::parse("I need gun-fu training for my agent");
    construct.manifest(&skill_req);

    // 5. Walk through the Construct
    let view = construct.walk_through("Neo");
    println!("{}", view.render(lau_construct::ViewMode::Matrix));

    // 6. Deploy
    let deploy_req = ConstructRequest::parse("deploy");
    construct.manifest(&deploy_req);

    // 7. Check status
    println!("{}", construct.status());

    // 8. Render in different modes
    println!("{}", construct.render(ConstructRenderMode::MUD));
    println!("{}", construct.render(ConstructRenderMode::JSON));
}
```

---

## API Reference

### `Construct`

The main entry point — the shared creative space.

| Method | Returns | Description |
|--------|---------|-------------|
| `new(name, creator, operator, budget)` | `Self` | Create an empty white room |
| `manifest(request)` | `Vec<ConstructResult>` | Materialize what the user asked for |
| `dematerialize(item_id)` | `bool` | Remove a room/agent/hardware/bridge by name or ID |
| `modify(item_id, change)` | `ConstructResult` | Change something (resize, equip, etc.) |
| `walk_through(viewer)` | `ConstructView` | See the Construct from a perspective |
| `deploy()` | `DeployResult` | Take everything live |
| `reset()` | `()` | Clear back to white room |
| `status()` | `ConstructStatus` | Current counts and energy state |
| `render(mode)` | `String` | Full render in Matrix/MUD/JSON/Narrative/Voice mode |
| `is_conserved()` | `bool` | Check energy hasn't been over-spent |

### `ConstructRequest`

Natural language parser.

```rust
let req = ConstructRequest::parse("I need rooms for engineering and training");
```

| Intent | Trigger words |
|--------|---------------|
| `NeedRooms` | "room", "chamber", "space" |
| `NeedAgents` | "agent", "operator", "gun" |
| `NeedHardware` | "motor", "servo", "sensor", "hardware" |
| `NeedBridges` | "bridge", "connect", "link" |
| `NeedSkills` | "skill", "train", "learn" |
| `LoadProgram` | "load" + "program"/"training" |
| `Modify` | "make", "change", "modify", "bigger", "smaller" |
| `Test` | "test" |
| `Deploy` | "deploy", "take it live", "go live" |
| `Reset` | "reset", "clear", "start over" |
| `NeedCustom` | anything else |

### `ConstructRoom`

A materialized space in the Construct.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Room name |
| `room_type` | `RoomType` | Training, Mission, Hardware, Bridge, Social, Creative, Custom |
| `contents` | `Vec<String>` | What's inside |
| `size` | `(usize, usize)` | Width × Height |
| `energy_level` | `f64` | Energy (1.0 when materialized) |
| `loaded` | `bool` | Whether it's materialized |
| `connected_to` | `Vec<String>` | Connected room IDs |

| Method | Description |
|--------|-------------|
| `new(name, room_type)` | Create an un-materialized room |
| `materialize()` | Load it into the Construct |
| `describe() → String` | Text description |
| `render() → String` | ASCII art box rendering |

### `ConstructAgent`

An agent in the Construct.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Agent name |
| `archetype` | `String` | warrior, scout, diplomat, engineer, medic, etc. |
| `level` | `u32` | Level (increases with skills) |
| `skills` | `Vec<String>` | Learned skills |
| `equipment` | `Vec<String>` | Equipped items |
| `voice` | `Option<String>` | Voice model |

| Method | Description |
|--------|-------------|
| `new(name, archetype)` | Create an agent |
| `learn_skill(skill)` | Teach a skill (no duplicates, levels up) |
| `equip(item)` | Give equipment (no duplicates) |
| `describe() → String` | Full description |

### `ConstructHardware`

Physical or simulated hardware.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Hardware name |
| `hw_type` | `String` | servo, sensor, camera, display, etc. |
| `channels` | `u32` | Number of channels |
| `connected` | `bool` | Real connection status |
| `simulated` | `bool` | Whether it's simulated |

### `ConstructBridge`

Connection to another instance.

| Field | Type | Description |
|-------|------|-------------|
| `target` | `String` | Target instance name |
| `status` | `BridgeStatus` | Connected, Pending, or Simulated |

### `ConstructView`

A walkthrough perspective with multiple render modes.

| Mode | Style |
|------|-------|
| `Matrix` | Unicode box drawing with sections |
| `MUD` | "You are X. Around you..." text adventure |
| `Dashboard` | Compact stats box |
| `Voice` | Narrative text only |

### `ConstructState`

The lifecycle states:

| State | Meaning |
|-------|---------|
| `Empty` | White room, nothing loaded |
| `Loading` | Materializing the user's request |
| `Ready` | Everything loaded, waiting for walkthrough |
| `Testing` | User is walking through |
| `Modifying` | User is changing things |
| `Deploying` | Taking it live |
| `Active` | Deployed and running |

### Pre-built Programs

| Function | Description |
|----------|-------------|
| `load_combat_program()` | Dojo + Arsenal + Arena, Neo & Trinity, gun-fu skills |
| `load_engineering_program()` | Hardware Lab + Motor Bay + Sensor Array, Spark & Torque |
| `load_social_program()` | Palaver Room + Bridge Room, Eloquence & Herald |
| `load_exploration_program()` | Cartography + Scout Den, Pathfinder & Chart |

---

## How It Works

### Request Parsing

`ConstructRequest::parse()` uses keyword matching to classify the user's intent:

1. **Check for system commands** — deploy, reset, test, modify, load program
2. **Check for entity types** — rooms, agents, hardware, bridges, skills
3. **Extract items** — scan for known keywords (e.g., "engineering", "servo", "gun-fu")
4. **Fall through** — unrecognized requests become custom constructs

### Materialization

When `manifest()` processes a request:

1. Transitions state to `Loading`
2. For each item, checks if the energy budget can afford it:
   - Room: 10 energy
   - Agent: 15 energy
   - Hardware: 5 energy
   - Bridge: 8 energy
   - Skill: 3 energy
   - Custom: 12 energy
   - Program load: 20 energy
3. If affordable, creates the entity and deducts energy
4. If not, returns a failure result
5. Auto-transitions to `Ready` when done

### Modification

`modify()` searches rooms and agents by name or ID:

- **"bigger"/"larger"** — doubles room size
- **"smaller"** — halves room size (min 4×4)
- **"skill"/"learn"/"train"** — teaches agent the change text as a skill
- Otherwise — adds change text to room contents or equips on agent

### Energy Conservation

The Construct maintains:

```
energy_spent ≤ energy_pool
```

`is_conserved()` verifies this. `energy_remaining()` returns `max(0, energy_pool - energy_spent)`.

### Entity Identification

Rooms, agents, hardware, and bridges all get auto-generated IDs (`room-<uuid>`, `agent-<uuid>`, etc.) but can also be referenced by name in `dematerialize()` and `modify()`.

---

## The Math

### Energy Budget

The Construct operates on a fixed energy budget:

```
E_remaining = max(0, E_pool - E_spent)
```

Each entity has a fixed cost:

| Entity | Cost |
|--------|------|
| Room | 10 |
| Agent | 15 |
| Hardware | 5 |
| Bridge | 8 |
| Skill | 3 |
| Custom | 12 |
| Program load | 20 |
| Modify (resize up) | 2 |
| Modify (resize down) | 1 |
| Modify (generic) | 1 |

The budget is non-negative: `can_afford(cost)` requires `energy_remaining ≥ cost`.

### Shannon Entropy Connection

While `lau-construct` itself doesn't compute entropy, it's designed to integrate with `lau-conservation-engine`, where the variety (V) of behavioral categories feeds into the conservation law `γ + H = C − α·ln(V)`. The room types, skill categories, and operation types in the Construct all contribute to the category distribution that determines V.

### Agent Leveling

Agent level increases by 1 for each unique skill learned:

```
level = 1 + |unique_skills|
```

Skills are deduplicated — learning the same skill twice doesn't double-count.

---

## Testing

The library includes **83 unit tests** covering:

- Construct creation and full lifecycle (empty → active)
- All 11 intent types parsed from natural language
- Room creation, materialization, description, and rendering
- Agent creation, skill learning (dedup), equipment, and description
- Hardware and bridge creation and description
- Modification (resize, equip, teach skills)
- Dematerialization by name and ID
- Energy conservation and depletion
- All 4 pre-built programs
- All 5 render modes (Matrix, MUD, JSON, Narrative, Voice)
- Full Neo workflow integration test
- Serde roundtrip serialization
- Status display and entity ID generation

Run them:

```sh
cargo test
```

---

## License

MIT
