# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run -- --top N --min-length M [--file FILE_PATH] [--json]` - Run with arguments
- `cargo test` - Run all tests
- `cargo test -- --nocapture` - Run tests with stdout output
- `cargo test test_name` - Run a specific test

### Examples
```bash
# Read from stdin
echo "hello world hello" | cargo run -- --top 2 --min-length 1

# Read from file with JSON output
cargo run -- --top 5 --min-length 3 --file input.txt --json
```

## Architecture

This is a Rust word frequency counter CLI tool that processes text input and outputs the most frequent words.

### Project Structure

- **src/main.rs** - CLI entry point. Handles file/stdin input and output formatting.
- **src/lib.rs** - Core library containing:
  - `top_words()` - Main algorithm that counts words and returns top N results
  - `parse_args()` - Custom CLI argument parser (no external CLI library)
  - `Options`, `WordCount`, `Output` - Data structures (with serde derives for JSON)
  - Unit tests for word counting logic
- **src/json_tests.rs** - Separate module for JSON serialization tests

### Key Design Decisions

1. **Custom argument parsing** - No clap/structopt; manually parses `std::env::args()`
2. **Input sources** - Supports both stdin (single line) and file input
3. **Text normalization** - Converts to lowercase, filters to alphanumeric/whitespace only
4. **Sorting** - By count descending, then alphabetically ascending for tie-breaking
5. **Output formats** - Plain text (default) or JSON via `--json` flag

### Dependencies

- `serde` + `serde_json` - JSON serialization support
- No other external crates

### Test Organization

Tests are co-located in `lib.rs` and a separate `json_tests.rs` module, both using `#[cfg(test)]`.
