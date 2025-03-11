# System Patterns

## Architecture

The game uses the Entity Component System (ECS) architecture, as is standard in Bevy.

## Key Components

- **Entities:** Unique identifiers representing game objects.
- **Components:** Data containers that hold specific properties of entities (e.g., position, health, appearance).
- **Systems:** Logic that operates on entities with specific components.
- **Resources:** Globally accessible data shared across systems.
- **Plugins:** Groups of related components, resources, and systems.

## Relationships

- Entities are composed of multiple components.
- Systems query for entities with specific combinations of components.
- Systems modify component data, driving game logic.
- Resources provide global data and services to systems.
- Plugins encapsulate and organize related game features.

## Current Systems (Inferred from file structure)
- `player.rs`: Likely handles player input, movement, and interactions.
- `enemy.rs`: Likely manages enemy behavior and AI.
- `score.rs`: Likely tracks and updates the player's score.
- `ui.rs`: Likely handles user interface elements.
- `menu.rs`: Likely manages menu interactions and navigation.
- `pause.rs`: Likely handles pausing and unpausing the game.
- `camera.rs`: Likely controls the game camera.
- `powerup.rs`: Likely manages power-up behavior and effects.
- `boost.rs`: Likely handles boost mechanics.
- `gameover.rs`: Likely handles game over conditions and screen.
- `win.rs`: Likely handles win conditions and screen.
- `skybox.rs`: Implements the skybox rendering.
