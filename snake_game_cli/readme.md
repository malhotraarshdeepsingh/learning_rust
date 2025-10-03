# Snake Game CLI in Rust

Welcome to the **Snake Game CLI**! This is a fun, terminal-based implementation of the classic Snake game, written in **Rust**. The game is designed to run directly in your terminal, using the `crossterm` crate for terminal manipulation and `rand` for random food placement.

This repository serves as a beginner-to-intermediate Rust programming project, demonstrating concepts like terminal control, event handling, game loops, and more.

---

## Table of Contents

1. [Overview](#overview)
2. [How the Code Works](#how-the-code-works)
3. [Code Breakdown](#code-breakdown)
4. [Installation](#installation)
5. [Usage](#usage)
6. [Game Flow Diagram](#game-flow-diagram)
7. [Acknowledgements](#acknowledgements)
8. [License](#license)

---

## Overview

The **Snake Game CLI** is a simple game where:
- You control a snake moving around the terminal window.
- The snake eats food to grow longer.
- The game ends when the snake collides with the walls or itself.

This project is a great way to learn:
- **Rust basics**: Working with vectors, closures, and random number generation.
- **Terminal manipulation**: Using the `crossterm` crate.
- **Game development patterns**: Game loop, event handling, and state management.

---

## How the Code Works

The game runs in a continuous loop until the player exits or loses. Here's a high-level explanation of how the code works:
1. **Setup**:
   - The terminal is configured for raw mode to handle real-time input.
   - The game initializes with a snake, a food position, and a score.

2. **Game Loop**:
   - Continuously listens for user input to change the snake's direction.
   - Moves the snake forward and checks for collisions or if the snake eats the food.
   - Updates the game state and redraws the screen.

3. **Termination**:
   - When the snake collides with itself or the walls, the game ends.
   - The terminal state is cleaned up before exiting.

---

## Code Breakdown

Let's break down each section of the code.

### 1. **Imports**
```rust
use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, terminal,
};
use rand::Rng;
use std::io::stdout;
use std::thread;
use std::time::{Duration, Instant};
```
- `crossterm`: Handles terminal input/output, cursor control, and raw mode.
- `rand::Rng`: Generates random numbers for food placement.
- `std::io::stdout`: Manages terminal output.
- `std::thread` and `std::time`: Manages timing and delays in the game loop.

### 2. **Constants**
```rust
const WIDTH: u16 = 80;
const HEIGHT: u16 = 20;
```
- Defines the terminal "grid" size for the game. The game area is `80x20` cells.

### 3. **Terminal Setup**
```rust
let mut stdout = stdout();
execute!(stdout, terminal::EnterAlternateScreen).unwrap();
execute!(stdout, cursor::Hide).unwrap();
terminal::enable_raw_mode().unwrap();
```
- Switches to an alternate terminal screen for the game.
- Hides the cursor to improve game visuals.
- Enables raw mode to handle real-time key presses.

### 4. **Snake Initialization**
```rust
let mut snake: Vec<(u16, u16)> = vec![
    (WIDTH / 4, HEIGHT / 2),
    (WIDTH / 4 - 1, HEIGHT / 2),
    (WIDTH / 4 - 2, HEIGHT / 2),
];
```
- The snake is represented as a vector of coordinates `(x, y)`.
- Starts as a 3-segment snake in the middle-left of the grid.

### 5. **Food Initialization**
```rust
let mut rng = rand::thread_rng();
let mut food = (rng.gen_range(1..WIDTH - 1), rng.gen_range(1..HEIGHT - 1));
```
- Randomly places food within the game area (excluding walls).

### 6. **Game Variables**
```rust
let mut score = 0;
let mut direction = KeyCode::Right;
let mut last_update_item = Instant::now();
let update_interval = Duration::from_millis(100);
```
- `score`: Tracks the player's score.
- `direction`: Tracks the snake's current movement direction.
- `last_update_item`: Tracks the last time the snake moved.
- `update_interval`: Sets the delay between snake movements (100ms).

### 7. **Game Loop**
```rust
loop {
    // Check for user input
    if event::poll(Duration::from_millis(0)).unwrap() {
        if let event::Event::Key(key_event) = event::read().unwrap() {
            direction = match key_event.code {
                KeyCode::Up if direction != KeyCode::Down => KeyCode::Up,
                KeyCode::Down if direction != KeyCode::Up => KeyCode::Down,
                KeyCode::Left if direction != KeyCode::Right => KeyCode::Left,
                KeyCode::Right if direction != KeyCode::Left => KeyCode::Right,
                KeyCode::Esc => break,
                _ => direction,
            };
        }
    }

    // Update game state
    if last_update_item.elapsed() >= update_interval {
        let head = snake[0];
        let new_head = match direction {
            KeyCode::Up => (head.0, head.1 - 1),
            KeyCode::Down => (head.0, head.1 + 1),
            KeyCode::Left => (head.0 - 1, head.1),
            KeyCode::Right => (head.0 + 1, head.1),
            _ => head,
        };

        snake.insert(0, new_head);

        // Collision checks
        if new_head.0 == 0 || new_head.0 == WIDTH - 1 || new_head.1 == 0 || new_head.1 == HEIGHT - 1 || snake[1..].contains(&new_head) {
            break;
        }

        // Food collision
        if new_head == food {
            score += 1;
            food = loop {
                let new_food = (rng.gen_range(1..WIDTH - 1), rng.gen_range(1..HEIGHT - 1));
                if !snake.contains(&new_food) {
                    break new_food;
                }
            };
        } else {
            snake.pop();
        }

        last_update_item = Instant::now();
    }

    // Render the game state
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                print!("#");
            } else if snake.contains(&(x, y)) {
                print!("O");
            } else if (x, y) == food {
                print!("F");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("Score: {}", score);

    thread::sleep(Duration::from_millis(100));
}
```
- **Input Handling**: Reads arrow keys to change the snake's direction.
- **Snake Movement**: Moves the snake in the current direction and checks for collisions with walls or itself.
- **Food Eating**: If the snake eats the food, the score increases, and new food is spawned.
- **Rendering**: Redraws the terminal with the updated game state.

### 8. **Game Over and Cleanup**
```rust
execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
execute!(stdout, cursor::Show).unwrap();
```
- Restores the terminal to its original state and shows the cursor.

---

## Game Flow Diagram

Below is a simplified flowchart of the game logic:

```plaintext
+-------------------+
|  Start Game       |
+-------------------+
         |
         v
+-------------------+
|  Setup Terminal   |
+-------------------+
         |
         v
+-------------------+
|  Game Loop        |
| - Read Input      |
| - Update Snake    |
| - Check Collisions|
| - Render Screen   |
+-------------------+
         |
         v
+-------------------+
|  Game Over        |
+-------------------+
         |
         v
+-------------------+
| Cleanup Terminal  |
+-------------------+
```

---

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/malhotraarshdeepsingh/learning_rust.git
   cd learning_rust/snake_game_cli
   ```

2. **Install Rust**:
   Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install).

3. **Build and Run**:
   ```bash
   cargo run
   ```

---

## Usage

- Use the **arrow keys** to control the snake.
- Press `Esc` to exit the game.

---

## Acknowledgements

- **Rust Programming Language**: For its powerful features and performance.
- **Crossterm**: For terminal manipulation.
- **Rand Crate**: For random number generation.

---
