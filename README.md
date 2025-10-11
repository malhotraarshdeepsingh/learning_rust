# Learning Rust

A comprehensive repository for learning and experimenting with the Rust programming language. This repository contains a collection of Rust projects, practice codes, and small applications to help developers grasp various Rust concepts by example.

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Directory Details](#directory-details)
- [Getting Started](#getting-started)
- [Requirements](#requirements)
- [Contributing](#contributing)
- [License](#license)

## Overview

This repository is structured as a set of independent folders, each demonstrating a different concept, project, or experiment in Rust. Whether you are a beginner or an intermediate Rustacean, you'll find hands-on code and practical examples ranging from simple CLI tools to games and blockchain prototypes.

## Project Structure

The repository contains the following top-level directories:

- `basicRUST/` – Foundational Rust programs showcasing syntax and simple features.
- `chat_cli/` – A command-line chat application built with Rust.
- `compress/` – Projects related to file compression.
- `csv_file_reader/` – Read and process CSV files in Rust.
- `iron_api/` – API-related code, likely using the Iron web framework.
- `practiceCodes/` – Miscellaneous Rust practice snippets and exercises.
- `snake_game_cli/` – Terminal-based Snake game implementation.
- `snake_ui/` – Snake game with a graphical user interface.
- `toy_blockchain/` – A simplistic blockchain implementation to demonstrate core blockchain principles.
- `unzip/` – Projects focused on unzipping files.

There is also a `.gitignore` file for managing version control exclusions.

## Directory Details

### `basicRUST/`
Contains basic programs and learning exercises, ideal for new Rust learners.

### `chat_cli/`
A command-line chat client demonstrating networking, concurrency, and user input handling in Rust.

### `compress/`
Contains Rust code for compressing files, likely showcasing I/O operations and third-party crate usage.

### `csv_file_reader/`
Utilities and applications for reading and parsing CSV files, teaching file handling and data processing.

### `iron_api/`
API server code, possibly using the Iron framework, demonstrating web service creation in Rust.

### `practiceCodes/`
A sandbox for various Rust snippets, algorithms, and small programs to practice and reinforce Rust concepts.

### `snake_game_cli/`
A playable Snake game in the terminal, using Rust's system libraries for real-time input and rendering.

### `snake_ui/`
An upgraded version of the Snake game with a graphical interface, introducing GUI programming in Rust.

### `toy_blockchain/`
A simple blockchain prototype, illustrating cryptographic hashes, proof-of-work, and distributed ledger basics.

### `unzip/`
Projects focusing on decompressing files, furthering understanding of file formats and Rust's ecosystem.

## Getting Started

Clone the repository:

```sh
git clone https://github.com/malhotraarshdeepsingh/learning_rust.git
cd learning_rust
```

Each subdirectory contains a separate Rust project. To build and run a project, navigate to its folder and use Cargo:

```sh
cd <project_folder>
cargo run
```

For example, to run the CLI Snake game:

```sh
cd snake_game_cli
cargo run
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Cargo (comes with Rust)
- Some projects may require additional dependencies, specified in their respective `Cargo.toml` files

## Contributing

Contributions are welcome! Please open issues or pull requests to suggest improvements, fix bugs, or add new Rust examples.


---

Happy learning and hacking with Rust!
