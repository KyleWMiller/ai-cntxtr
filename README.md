# Rust Code Flattener

A command-line tool that consolidates multiple Rust files from a directory into a single file while preserving code structure and readability.

## Features

- Flattens all `.rs` files from a specified directory and its subdirectories
- Automatically outputs to `./output/output_[dirname].rs`
- Maintains code organization with clear file separation comments
- Preserves original code structure and formatting
- Automatically creates output directories if they don't exist
- Includes timestamp and source information in generated files

## Installation

1. Clone this repository:
```bash
git clone [your-repository-url]
cd rust-code-flattener
```

2. Add the required dependencies to your `Cargo.toml`:
```toml
[package]
name = "rust-code-flattener"
version = "0.1.0"
edition = "2021"

[dependencies]
walkdir = "2.3"
chrono = "0.4"
```

3. Build the project:
```bash
cargo build --release
```

## Usage

### Basic Usage

Flatten a directory:
```bash
cargo run src/models
```

This will create a file at `./output/output_models.rs`

### Examples

1. Flatten a models directory:
```bash
cargo run src/models
# Creates ./output/output_models.rs
```

2. Flatten a utils directory:
```bash
cargo run src/utils
# Creates ./output/output_utils.rs
```

3. Flatten a handlers directory:
```bash
cargo run src/handlers
# Creates ./output/output_handlers.rs
```

### Output Structure

Files are automatically created in the `./output` directory with the following naming convention:
```
./output/output_[directory_name].rs
```

For example:
- `src/models` -> `./output/output_models.rs`
- `src/utils` -> `./output/output_utils.rs`
- `src/handlers` -> `./output/output_handlers.rs`

### Generated File Format

The generated file will have the following structure:

```rust
// Concatenated Rust files from directory: [directory_path]
// Generated automatically on [timestamp]
// This file contains all Rust code from the specified directory

// File: [original_file_path_1]
[contents of file 1]

// File: [original_file_path_2]
[contents of file 2]

// ... additional files
```

[Rest of the README remains the same...]