# word_counter

A fast command-line tool for counting word frequency in text files or from stdin.

## Features

- Count word frequencies from files or stdin
- Filter words by minimum length
- Output the top N most frequent words
- Support for plain text and JSON output formats
- Multi-architecture support (x86_64 and ARM64)

## Installation

### From Pre-built Packages

Pre-built DEB and RPM packages are available on the [Releases](https://github.com/vogonwannt/word_counter/releases) page.

**Debian/Ubuntu (x86_64):**
```bash
wget https://github.com/vogonpoet/word_counter/releases/download/v0.1.0/word-counter_0.1.0_amd64.deb
sudo dpkg -i word-counter_0.1.0_amd64.deb
```

**Debian/Ubuntu (ARM64):**
```bash
wget https://github.com/vogonpoet/word_counter/releases/download/v0.1.0/word-counter_0.1.0_arm64.deb
sudo dpkg -i word-counter_0.1.0_arm64.deb
```

**Fedora/RHEL/CentOS/openSUSE (x86_64):**
```bash
wget https://github.com/vogonpoet/word_counter/releases/download/v0.1.0/word-counter-0.1.0-1.x86_64.rpm
sudo rpm -i word-counter-0.1.0-1.x86_64.rpm
```

**Fedora/RHEL/CentOS/openSUSE (ARM64):**
```bash
wget https://github.com/vogonpoet/word_counter/releases/download/v0.1.0/word-counter-0.1.0-1.aarch64.rpm
sudo rpm -i word-counter-0.1.0-1.aarch64.rpm
```

### From Source

```bash
# Clone the repository
git clone https://github.com/vogonpoet/word_counter.git
cd word_counter

# Build and install
cargo build --release
sudo cp target/release/word_counter /usr/local/bin/
```

## Usage

### Basic Usage

```bash
# Count words from stdin
echo "hello world hello" | word_counter --top 2 --min-length 1
```

Output:
```
hello: 2
world: 1
```

### From File

```bash
# Count words from a file
word_counter --top 5 --min-length 3 --file input.txt
```

### JSON Output

```bash
# Output in JSON format
word_counter --top 5 --min-length 3 --file input.txt --json
```

Output:
```json
[
  {"word": "example", "count": 10},
  {"word": "frequency", "count": 5}
]
```

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `--top N` | Number of top words to display | Required |
| `--min-length N` | Minimum word length to count | Required |
| `--file PATH` | Path to input file | Read from stdin |
| `--json` | Output in JSON format | Plain text |

## Examples

**Find the 10 most common words (at least 4 characters):**
```bash
word_counter --top 10 --min-length 4 --file document.txt
```

**Count words from a pipeline:**
```bash
cat essay.txt | word_counter --top 20 --min-length 1
```

**Export results as JSON:**
```bash
word_counter --top 50 --min-length 3 --file book.txt --json > results.json
```

## How It Works

1. **Input**: Reads text from stdin or a file
2. **Normalization**: Converts to lowercase, keeps only alphanumeric characters and whitespace
3. **Counting**: Counts frequency of each word (filtered by minimum length)
4. **Sorting**: Sorts by count (descending), then alphabetically (ascending)
5. **Output**: Returns top N results in plain text or JSON format

## Building Packages Locally

```bash
# Install packaging tools
cargo install cargo-deb
cargo install cargo-generate-rpm

# Build packages
cargo build --release
cargo deb
cargo generate-rpm
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with stdout output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Project Structure

```
word_counter/
├── src/
│   ├── main.rs      # CLI entry point
│   ├── lib.rs       # Core library with algorithms
│   └── json_tests.rs # JSON serialization tests
├── Cargo.toml       # Package configuration
├── LICENSE          # MIT License
└── README.md        # This file
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
