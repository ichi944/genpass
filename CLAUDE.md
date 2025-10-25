# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `genpass`, a CLI password generator written in Rust.

## Design Philosophy

**Critical Requirements:**
- **Lightweight** - Keep the binary size small and execution fast
- **Minimal Dependencies** - Password generation logic must have zero external dependencies; use only Rust standard library
- **Flexible Options** - Provide comprehensive CLI options for customizing password generation (length, character sets, patterns, etc.)

When adding features or making changes, always prioritize minimizing dependencies and maintaining a small footprint.

### Dependency Policy
- **CLI parsing**: `clap` is allowed for argument parsing and interface
- **Password generation**: ZERO dependencies - use only `std::fs::File` to read from `/dev/urandom` (Unix) or FFI for Windows Crypto API
- **No other dependencies** should be added without strong justification

## Dependencies

**External Crates:**
- `clap` (with `derive` feature) - CLI argument parsing only

**Standard Library Usage:**
- `std::env` - Environment and program metadata
- `std::fs::File` - Read from `/dev/urandom` for secure random numbers on Unix-like systems
- `std::io` - I/O operations and error handling
- `std::process` - Exit codes
- Standard collections (`Vec`, `String`, etc.)

## Development Commands

### Building and Running
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release binary
- `cargo run` - Run the project
- `cargo run -- [args]` - Run with arguments

### Code Quality
- `cargo check` - Quick compile check without producing binary
- `cargo clippy` - Run linter for catching common mistakes
- `cargo fmt` - Format code according to Rust style guidelines
- `cargo fmt -- --check` - Check formatting without modifying files

### Testing
- `cargo test` - Run all tests
- `cargo test [test_name]` - Run a specific test
- `cargo test -- --nocapture` - Run tests with stdout/stderr visible

## CLI Options Specification

The password generator supports fine-grained control over character composition and password length.

### Character Type Controls
Each character type can have independent minimum and/or maximum constraints:

- **Numeric (0-9)**
  - `--min-numeric <n>` - Minimum number of numeric characters
  - `--max-numeric <n>` - Maximum number of numeric characters

- **Lowercase Alpha (a-z)**
  - `--min-lower <n>` - Minimum number of lowercase letters
  - `--max-lower <n>` - Maximum number of lowercase letters

- **Uppercase Alpha (A-Z)**
  - `--min-upper <n>` - Minimum number of uppercase letters
  - `--max-upper <n>` - Maximum number of uppercase letters

- **Symbols (!@#$%^&*, etc.)**
  - `--min-symbol <n>` - Minimum number of symbol characters
  - `--max-symbol <n>` - Maximum number of symbol characters

### Password Length Controls
- `--length <n>` - Set exact password length (shorthand for setting both min and max to the same value)
- `--min-length <n>` - Minimum total password length
- `--max-length <n>` - Maximum total password length

**Note:** When only min or max is specified for a character type, the other bound is unconstrained (within the overall password length limits).

### Character Set Customization
- `--symbols <string>` - Define exactly which symbol characters to use (default: `!@#$%^&*()_+-=[]{}|;:,.<>?`)
- `--exclude-ambiguous` - Exclude visually similar characters (0/O, 1/l/I, etc.)

### Output Options
- `--count <n>` - Number of passwords to generate (default: 1)

## Architecture

The codebase is organized into three main modules:

### Module Structure

**`src/main.rs`** - CLI entry point
- Parses command-line arguments using `clap`
- Converts CLI args to `PasswordConstraints`
- Handles errors and outputs passwords

**`src/random.rs`** - Zero-dependency secure random number generation
- `SecureRandom::fill_bytes()` - Reads from `/dev/urandom` on Unix systems
- `SecureRandom::random_range()` - Generates random numbers in a range using rejection sampling to avoid modulo bias
- `SecureRandom::shuffle()` - Fisher-Yates shuffle algorithm for randomizing password characters

**`src/generator.rs`** - Password generation logic (zero dependencies)
- `PasswordConstraints` - Struct holding all password requirements
- `PasswordConstraints::validate()` - Validates that constraints are internally consistent
- `PasswordGenerator` - Generates passwords based on constraints
- Character set filtering for ambiguous characters when requested

### Password Generation Algorithm

1. **Determine password length**: Random value between min_length and max_length
2. **Allocate minimum required characters**: Start with minimum counts for each character type
3. **Distribute remaining slots**: Randomly add characters to types that haven't reached their max, respecting all constraints
4. **Build character list**: Create actual characters for each type based on final counts
5. **Shuffle**: Use Fisher-Yates algorithm to randomize character positions

### Constraint Validation

The implementation validates constraints before generation:
- Min values must not exceed max values for each character type
- Sum of minimum character requirements cannot exceed password length
- Character sets must not be empty after filtering

### Binary Size

The release binary is approximately **491KB** thanks to:
- Minimal dependencies (only `clap` for CLI)
- Zero-dependency password generation using OS random sources
- Size optimizations in `Cargo.toml` (opt-level="z", LTO, strip symbols)
