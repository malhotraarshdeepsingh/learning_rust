# Learning Rust: File Compression with Gzip

Welcome to the **Learning Rust** repository! This repository contains a hands-on project that demonstrates how to implement file compression using the `flate2` crate in Rust. The focus of this project is to learn the basics of Rust programming by developing a simple yet powerful command-line tool to compress files using the Gzip format.

---

## Table of Contents
1. [Introduction](#introduction)
2. [How It Works](#how-it-works)
3. [Code Walkthrough](#code-walkthrough)
4. [Setup and Usage](#setup-and-usage)
5. [Diagrams](#diagrams)
6. [Project Architecture](#project-architecture)
7. [Acknowledgements](#acknowledgements)

---

## Introduction

This project serves as a learning experience for Rust's core features such as:
- Handling command-line arguments.
- File I/O operations.
- Using external crates and libraries.
- Error handling.
- Measuring execution time for performance analysis.

The program reads a file, compresses it using Gzip, and writes the compressed content to a new file. It also outputs useful performance metrics such as the time taken for compression, the original file size, and the compressed file size.

---

## How It Works

The program uses the following steps:
1. Parses command-line arguments to get the source and destination file paths.
2. Reads the source file using a buffered reader for efficient I/O.
3. Compresses the file using the Gzip algorithm provided by the `flate2` crate.
4. Writes the compressed content to the destination file.
5. Outputs the elapsed time, original file size, and compressed file size to the console.

---

## Code Walkthrough

Below is an explanation of the key components of the code:

### 1. **Importing Necessary Libraries**
```rust
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;
use std::env::args;
```
- `flate2`: Provides the Gzip compression utilities.
- `std::fs::File`: Allows reading from and writing to files.
- `std::io::copy`: Copies data between readers and writers.
- `std::io::BufReader`: Wraps a reader to provide efficient buffered reading.
- `std::time::Instant`: Used for performance measurement.
- `std::env::args`: Handles command-line arguments.

---

### 2. **Command-Line Argument Parsing**
```rust
if args().len() != 3 {
    eprintln!("Usage: {} <source> <destination>", args().next().unwrap());
    return;
}
```
- Ensures the correct number of arguments (source and destination paths) are provided.
- If not, it prints an error message with the correct usage format.

---

### 3. **Reading and Writing Files**
```rust
let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
let output = File::create(args().nth(2).unwrap()).unwrap();
```
- `File::open`: Opens the source file for reading. Errors are unwrapped for simplicity in this example.
- `BufReader::new`: Wraps the file in a buffered reader for efficient I/O.
- `File::create`: Creates the destination file for writing.

---

### 4. **Initializing the Gzip Encoder**
```rust
let mut encoder = GzEncoder::new(output, Compression::default());
```
- `GzEncoder::new`: Creates a new Gzip encoder that writes compressed data to the destination file.
- `Compression::default()`: Uses the default compression level.

---

### 5. **Measuring Elapsed Time**
```rust
let start = Instant::now();
```
- Captures the current time to measure the duration of the compression process.

---

### 6. **Performing Compression**
```rust
copy(&mut input, &mut encoder).unwrap();
let output = encoder.finish().unwrap();
```
- `copy`: Reads data from the source file and writes it to the Gzip encoder.
- `encoder.finish()`: Finalizes the compression and returns the underlying writer (the destination file).

---

### 7. **Outputting Metrics**
```rust
println!("Elapsed time: {:?}", start.elapsed());
println!("Original size: {} bytes", input.get_ref().metadata().unwrap().len());
println!("Compressed size: {} bytes", output.metadata().unwrap().len());
```
- `start.elapsed()`: Calculates the time taken for compression.
- `metadata().len()`: Retrieves the file size in bytes for both the original and compressed files.

---

## Setup and Usage

### Prerequisites
- Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).

### Steps to Run
1. Clone the repository:
   ```bash
   git clone https://github.com/malhotraarshdeepsingh/learning_rust.git
   ```
2. Navigate to the project directory:
   ```bash
   cd learning_rust/compress
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Run the program:
   ```bash
   ./target/release/compress <source_file> <destination_file>
   ```
   Example:
   ```bash
   ./target/release/compress sample.txt sample.txt.gz
   ```

---

## Diagrams

### Workflow Diagram
Below is a simplified representation of how the program operates:
```
+-------------------+
| Command-Line Args |<-----------------------------+
+-------------------+                              |
         |                                         |
         v                                         |
+-------------------+                              |
| Open Input File   |                              |
+-------------------+                              |
         |                                         |
         v                                         |
+-------------------+                              |
| Buffered Reader   |                              |
+-------------------+                              |
         |                                         |
         v                                         |
+-------------------+                              |
| Gzip Compression  |                              |
+-------------------+                              |
         |                                         |
         v                                         |
+-------------------+                              |
| Write Output File |                              |
+-------------------+                              |
         |                                         |
         v                                         |
+-------------------------+                        |
| Print Metrics to Console|-----------------------+
+-------------------------+
```

---

## Project Architecture

### File Structure
```
learning_rust/
└── compress/
    ├── src/
    │   └── main.rs      # Main Rust file containing the program logic
    ├── Cargo.toml       # Project dependencies and metadata
    └── target/          # Build output directory (generated by Cargo)
```

### Key Components
- **`main.rs`**: Contains the entire logic for file compression.
- **`Cargo.toml`**: Specifies the dependencies (`flate2`) and project metadata.

---

## Acknowledgements
Special thanks to:
- The Rust community for their extensive documentation and support.
- The authors of the `flate2` crate for providing robust compression utilities.

---

## Author
**Arshdeep Singh Malhotra**  
GitHub: [malhotraarshdeepsingh](https://github.com/malhotraarshdeepsingh)

---

## Contribution
Contributions are welcome! Feel free to submit a pull request or open an issue if you find a bug or have a feature request.
