# Project Brief

## Overview

This project is a Bevy game, likely a 2D or 3D game based on the provided file structure. The presence of `components`, `plugins`, `resources`, `scenes`, `states`, and `systems` directories suggests a typical Bevy ECS (Entity Component System) architecture.

## Goal

The current goal is to add a skybox to the game.

## Initial File Structure

The project includes the following directories and files:

- `.cursorignore`
- `.gitignore`
- `Cargo.toml`: Rust's manifest file, indicating this is a Rust project using Cargo.
- `color_scheme.md`
- `index.html`: Potentially related to web deployment (Bevy can compile to WASM).
- `README.md`
- `assets/`: Contains game assets like audio, fonts, and images.
- `src/`: Contains the game's source code.
  - `main.rs`: The main entry point of the application.
  - `components/`: Likely contains definitions for game entities' components.
  - `plugins/`: Contains Bevy plugins that group related logic and resources.
  - `resources/`: Contains Bevy resources, which are globally accessible data.
  - `scenes/`: Likely contains definitions for different game scenes.
  - `states/`: Contains definitions for different game states.
  - `styles/`: Contains styling information, possibly for UI elements.
  - `systems/`: Contains Bevy systems, which are functions that operate on entities and components.
- `target/`: Rust's build output directory.

## Technologies

- Rust
- Bevy Engine
