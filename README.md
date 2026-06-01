# lau-construct

> The Matrix Construct — a shared creative space where user and agent co-create PLATO environments

## What This Does

The Matrix Construct — a shared creative space where user and agent co-create PLATO environments. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-construct
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_construct::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub struct ConstructId(pub String);
    pub fn new() -> Self 
pub enum ConstructState 
pub enum RoomType 
pub enum BridgeStatus 
pub enum ConstructIntent 
pub struct ConstructRequest 
    pub fn parse(text: &str) -> Self 
    pub fn is_voice(&self) -> bool 
pub struct ConstructRoom 
    pub fn new(name: &str, room_type: RoomType) -> Self 
    pub fn materialize(&mut self) 
    pub fn describe(&self) -> String 
    pub fn render(&self) -> String 
pub struct ConstructAgent 
    pub fn new(name: &str, archetype: &str) -> Self 
    pub fn learn_skill(&mut self, skill: &str) 
    pub fn equip(&mut self, item: &str) 
    pub fn describe(&self) -> String 
pub struct ConstructHardware 
    pub fn new(name: &str, hw_type: &str) -> Self 
    pub fn describe(&self) -> String 
pub struct ConstructBridge 
    pub fn new(target: &str) -> Self 
    pub fn describe(&self) -> String 
pub struct ConstructResult 
pub struct ConstructView 
    pub fn render(&self, mode: ViewMode) -> String 
pub enum ViewMode 
pub enum ConstructRenderMode 
pub struct ConstructStatus 
pub struct DeployResult 
pub struct Construct 
    pub fn new(name: &str, creator: &str, operator: &str, budget: f64) -> Self 
    pub fn manifest(&mut self, request: &ConstructRequest) -> Vec<ConstructResult> 
    pub fn dematerialize(&mut self, item_id: &str) -> bool 
    pub fn modify(&mut self, item_id: &str, change: &str) -> ConstructResult 
    pub fn walk_through(&self, viewer: &str) -> ConstructView 
    pub fn deploy(&self) -> DeployResult 
    pub fn reset(&mut self) 
    pub fn status(&self) -> ConstructStatus 
    pub fn render(&self, mode: ConstructRenderMode) -> String 
    pub fn is_conserved(&self) -> bool 
pub fn load_combat_program() -> Construct 
pub fn load_engineering_program() -> Construct 
pub fn load_social_program() -> Construct 
pub fn load_exploration_program() -> Construct 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**83 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
