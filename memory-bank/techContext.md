# Tech Context

## Technologies

- **Programming Language:** Rust
- **Game Engine:** Bevy Engine
- **Build System:** Cargo

## Development Setup

- The project is set up as a standard Rust project using Cargo.
- Bevy is used as the game engine.

## Dependencies

- The project likely depends on the `bevy` crate, along with other potential crates for specific functionalities (e.g., physics, networking). These dependencies would be listed in `Cargo.toml`.

## Technical Constraints

- None identified at the moment.

## Skybox Implementation

- Implemented a basic skybox using a sphere mesh and placeholder textures.
- Skybox system added in `src/systems/skybox.rs` and integrated into `GamePlugin`.
- Currently using placeholder textures loaded from `assets/images/skybox_*.png`.
