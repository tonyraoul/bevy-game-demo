# Bevy Bear Balance

A charming 3D platformer game built with the Bevy game engine and Rust, where you control a cute 4-legged bear trying to maintain balance on a floating platform while competing with other bears.

## Game Description

In Bevy Bear Balance, you take control of an adorable 4-legged bear who must carefully navigate a suspended platform. But you're not alone! Other bears are also fighting for territory on the platform. The goal is simple: stay on the platform as long as possible while avoiding being pushed off by rival bears. Each time your bear falls off, you lose a precious point from your initial score of 10.

## Development Roadmap

### Core Mechanics [Priority 1]
- [ ] Create 3D bear model and animations
  - [ ] Idle animation
  - [ ] Walking animation
  - [ ] Pushing animation
- [ ] Implement WASD movement system
  - [ ] Add velocity-based movement
  - [ ] Add rotation towards movement direction
  - [ ] Add movement animations
- [ ] Design and implement the platform
  - [ ] Create platform mesh
  - [ ] Add physics colliders
  - [ ] Add visual effects for platform edges
- [ ] Implement falling detection
  - [ ] Add ray casting for ground detection
  - [ ] Create respawn system
  - [ ] Add score reduction on fall

### Enemy System [Priority 2]
- [ ] Create enemy bear AI
  - [ ] Implement pathfinding
  - [ ] Add target selection logic
  - [ ] Create behavior states (patrol, chase, fight)
- [ ] Add combat mechanics
  - [ ] Implement pushing physics
  - [ ] Add collision detection for combat
  - [ ] Create combat animations
- [ ] Enemy spawning system
  - [ ] Add spawn points
  - [ ] Implement wave system
  - [ ] Balance number of enemies

### UI and Scoring [Priority 3]
- [ ] Design and implement HUD
  - [ ] Add score display
  - [ ] Create health/status indicators
  - [ ] Add mini-map or position indicators
- [ ] Create scoring system
  - [ ] Implement score persistence
  - [ ] Add high score table
  - [ ] Create score animations

### Polish and Effects [Priority 4]
- [ ] Add sound effects
  - [ ] Movement sounds
  - [ ] Combat sounds
  - [ ] Falling sounds
  - [ ] Background music
- [ ] Visual effects
  - [ ] Particle effects for movement
  - [ ] Combat impact effects
  - [ ] Fall warning indicators
- [ ] Game feel improvements
  - [ ] Camera shake effects
  - [ ] Screen effects for near-falls
  - [ ] Haptic feedback indicators

### Additional Features [Priority 5]
- [ ] Power-up system
  - [ ] Speed boost
  - [ ] Strength boost
  - [ ] Temporary invulnerability
- [ ] Different game modes
  - [ ] Time trial
  - [ ] Survival mode
  - [ ] Tournament mode
- [ ] Platform variations
  - [ ] Moving platforms
  - [ ] Shrinking platforms
  - [ ] Obstacle courses

### Technical Tasks
- [ ] Optimization
  - [ ] Profile and optimize physics calculations
  - [ ] Implement entity pooling for performance
  - [ ] Add level of detail system
- [ ] Cross-platform support
  - [ ] Test and fix web export
  - [ ] Add mobile controls
  - [ ] Optimize for different platforms
- [ ] Save system
  - [ ] Implement save/load functionality
  - [ ] Add configuration persistence
  - [ ] Create backup system

## Features

- **Cute Bear Character**: Control a lovable 4-legged bear character
- **Simple Controls**: Use WASD keys for intuitive movement
  - W: Move forward
  - A: Move left
  - S: Move backward
  - D: Move right
- **Physics-Based Movement**: Realistic physics simulation using bevy_rapier3d
- **Score System**: Start with 10 points and try to maintain them
- **Penalty System**: Lose a point each time your bear falls off the platform
- **Clean UI**: Score display and main menu interface
- **Enemy Bears**: Compete against AI-controlled bears
  - Bears actively try to push others off the platform
  - Enemy bears fight among themselves
  - Dynamic combat system where bears can push and shove each other
- **Strategic Gameplay**: 
  - Use other bears' fights to your advantage
  - Position yourself strategically to avoid being pushed off
  - Time your movements to dodge aggressive bears

## Technical Details

Built using:
- Rust programming language
- Bevy Game Engine (v0.12)
- bevy_rapier3d for physics simulation
- Custom AI behavior system for enemy bears

## Development Status

This game is currently under development. Future features may include:
- High score system
- Different platform layouts
- Collectible items
- Sound effects and background music
- Visual effects for falling and scoring
- Different bear types with unique abilities
- Power-ups that affect bear strength or speed
- Tournament mode with multiple rounds

## Prerequisites

1. Rust and Cargo (install via [rustup](https://rustup.rs/))
2. For web builds:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-bindgen-cli
   ```

## Building and Running

### Native (macOS)

```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release
```

### Web

```bash
# Build for web
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target/wasm --target web ./target/wasm32-unknown-unknown/release/bevy-demo.wasm

# Serve the content (using Python's built-in server as an example)
python3 -m http.server
```

Then open your browser and navigate to `http://localhost:8000`

## Controls

The cube will automatically spin on its Y and X axes. The camera is positioned to view the cube from a slight elevation. 