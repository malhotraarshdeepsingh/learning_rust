# Snake Game in Rust

Welcome to the **Snake Game** implementation written in Rust! This project demonstrates how to build a graphical snake game using the [Piston](https://github.com/PistonDevelopers/piston) game engine. The goal is to familiarize developers with Rust concepts, game development basics, and modular programming.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Architecture](#project-architecture)
- [How the Code Works](#how-the-code-works)
  - [Detailed Code Explanation](#detailed-code-explanation)
  - [Flow Diagram](#flow-diagram)
- [Setup and Usage](#setup-and-usage)
- [Acknowledgments](#acknowledgments)

---

## Overview

The Snake Game is a classic arcade game where the player maneuvers a growing snake around the screen, collecting food and avoiding collisions with walls or itself. This implementation uses Rust's powerful safety features and the Piston library for a clean, modular, and efficient codebase.

---

## Features

- Modular structure with reusable components like `draw`, `snake`, and `game`.
- Interactive gameplay with keyboard controls.
- Randomized food generation.
- Game-over state handling with restart functionality.
- Neatly rendered graphics using Piston.

---

## Project Architecture

The project has been divided into the following modules:

1. **`main.rs`**: Entry point of the application. Handles window creation and event loop.
2. **`game.rs`**: Contains the core game logic, including food generation, snake movement, and game-over conditions.
3. **`draw.rs`**: Handles rendering and drawing on the screen.
4. **`snake.rs`**: Implements the `Snake` structure and its behavior.

Directory Structure:
```
snake_ui/
├── src/
│   ├── draw.rs       # Rendering utilities
│   ├── game.rs       # Game mechanics and logic
│   ├── main.rs       # Program entry point
│   ├── snake.rs      # Snake structure and behavior
```

---

## How the Code Works

### Detailed Code Explanation

#### 1. **`main.rs`**
This is the entry point of the program. It creates a game window, initializes the game, and enters the event loop.

Key Lines:
```rust
let (width, height) = (30, 30);
```
- Defines the grid size of the game.

```rust
let mut window: PistonWindow = WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
    .exit_on_esc(true)
    .build()
    .unwrap();
```
- Creates a game window using the Piston library. Converts grid dimensions to pixel coordinates for rendering.

```rust
while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
        game.key_pressed(key);
    }
    window.draw_2d(&event, |c, g, _| {
        clear(BACK_COLOR, g);
        game.draw(&c, g);
    });
    event.update(|arg| {
        game.update(arg.dt);
    });
}
```
- The event loop listens for keyboard inputs, updates the game logic, and redraws the screen.

#### 2. **`draw.rs`**
Provides utility functions for converting coordinates and drawing on the screen.

Key Functions:
```rust
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}
```
- Converts grid coordinates to pixel coordinates.

```rust
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}
```
- Draws a single block (used for snake segments and food).

#### 3. **`snake.rs`**
Defines the `Snake` structure and its behavior.

Key Components:
```rust
pub struct Snake {
    body: LinkedList<Block>,
    direction: Direction,
    tail: Option<Block>,
}
```
- Represents the snake as a linked list of blocks (segments). Keeps track of the snake's direction and the last removed block (tail).

```rust
pub fn move_forward(&mut self, dir: Option<Direction>) {
    match dir {
        Some(d) => self.direction = d,
        None => (),
    }
    let (last_x, last_y): (i32, i32) = self.head_position();
    let new_block = match self.direction {
        Direction::Up => Block { x: last_x, y: last_y - 1 },
        Direction::Down => Block { x: last_x, y: last_y + 1 },
        Direction::Left => Block { x: last_x - 1, y: last_y },
        Direction::Right => Block { x: last_x + 1, y: last_y },
    };
    self.body.push_front(new_block);
    let removed_block = self.body.pop_back().unwrap();
    self.tail = Some(removed_block);
}
```
- Moves the snake forward by adding a new block to the front and removing the last block.

#### 4. **`game.rs`**
Handles game state, including food generation, game-over conditions, and updates.

Key Components:
```rust
pub fn update(&mut self, delta_time: f64) {
    self.waiting_time += delta_time;
    if self.game_over {
        if self.waiting_time > RESTART_TIME {
            self.restart();
        }
        return;
    }
    if !self.food_exists {
        self.add_food();
    }
    if self.waiting_time > MOVING_PERIOD {
        self.update_snake(None);
    }
}
```
- Updates the game state. Adds food if it doesn't exist and moves the snake at regular intervals.

```rust
fn add_food(&mut self) {
    let mut rng = thread_rng();
    let mut new_x = rng.gen_range(1..self.width - 1);
    let mut new_y = rng.gen_range(1..self.height - 1);
    while self.snake.overlap_tail(new_x, new_y) {
        new_x = rng.gen_range(1..self.width - 1);
        new_y = rng.gen_range(1..self.height - 1);
    }
    self.food_x = new_x;
    self.food_y = new_y;
    self.food_exists = true;
}
```
- Randomly generates food coordinates, ensuring they don't overlap with the snake's body.

---

### Flow Diagram

Below is a high-level flow diagram of the game logic:

```plaintext
+-------------------+
|   Start Game      |
+-------------------+
         |
         v
+-------------------+
| Initialize Window |
+-------------------+
         |
         v
+-------------------+
|  Event Loop       |
| - Handle Input    |
| - Update State    |
| - Render Graphics |
+-------------------+
         |
         v
+-------------------+
|  Check for        |
|  Game Over        |
+-------------------+
         |
         v
+-------------------+
| Restart or Exit   |
+-------------------+
```

---

## Setup and Usage

### Prerequisites

- Install [Rust](https://www.rust-lang.org/).
- Install [Piston dependencies](https://github.com/PistonDevelopers/piston).

### Running the Game

1. Clone this repository:
   ```bash
   git clone https://github.com/malhotraarshdeepsingh/learning_rust.git
   cd learning_rust/snake_ui
   ```

2. Build and run the game:
   ```bash
   cargo run
   ```

3. Use arrow keys to control the snake.

---


## Acknowledgments

- **[Piston Game Engine](https://github.com/PistonDevelopers/piston):** For providing the framework for this project.
- **Rust Community:** For the amazing resources and support.
- **[Rand crate](https://crates.io/crates/rand):** For random number generation.

---

### Key Features
- **Food:** Red blocks randomly placed on the grid.
- **Snake:** Green blocks that grow as you eat food.
- **Borders:** Black edges defining the game area.
- **Game Over:** Semi-transparent red overlay.

---
