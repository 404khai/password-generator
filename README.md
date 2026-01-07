# Secure Password Generator (Rust CLI)

A robust, cryptographically secure command-line password generator written in Rust. Designed for security, performance, and ease of use.

## 🚀 Features

- **Cryptographically Secure**: Uses `rand::rngs::OsRng` to source entropy directly from the operating system's CSPRNG.
- **Unbiased Selection**: Uses `SliceRandom::choose` to guarantee uniform distribution and eliminate modulo bias.
- **Cross-Platform**: Works seamlessly on Linux, macOS, and Windows.
- **Zero-Config Defaults**: Generates a strong 16-character password out of the box.
- **Flexible**: Customize length and character sets via CLI flags.

## 📦 Installation

Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/yourusername/password-generator-rust.git
cd password-generator-rust
cargo build --release
```

The binary will be located in `target/release/password-generator-rust`.

## 🛠 Usage

### Basic Usage
Generate a secure 16-character password (default):
```bash
cargo run
# Output example: ]eB3.N:EAA8Ax%?m
```

### Custom Length
Generate a 24-character password:
```bash
cargo run -- --length 24
```

### Character Set Control
Exclude symbols (alphanumeric only):
```bash
cargo run -- --no-symbols
```

Exclude numbers (letters and symbols only):
```bash
cargo run -- --no-numbers
```

Only letters (uppercase and lowercase):
```bash
cargo run -- --only-letters
```

### Help
View all available options:
```bash
cargo run -- --help
```

## 🔒 Security Model

### Source of Randomness
We strictly avoid user-space PRNGs (like `thread_rng` or `SmallRng`) for generation to minimize attack vectors related to state compromise or poor seeding. Instead, we use **`rand::rngs::OsRng`**, which interfaces directly with the operating system's cryptographic entropy source:
- **Linux/macOS**: `/dev/urandom` (via `getrandom`)
- **Windows**: `BCryptGenRandom`

### Uniform Distribution
Naive approaches (like `rand() % len`) introduce **modulo bias**, making certain characters more likely to appear than others. This tool uses `SliceRandom::choose`, which implements rejection sampling (or similar unbiased algorithms) to ensure every character in the set has an exactly equal probability of being selected.

## 🦀 Why Rust?

- **Memory Safety**: Rust's ownership model prevents buffer overflows and memory leaks without a garbage collector.
- **Type Safety**: Strong typing and exhaustive pattern matching ensure valid program states (e.g., ensuring charsets are never empty).
- **Performance**: Zero-cost abstractions allow high-level logic (like iterators) to compile down to efficient machine code.
- **Ecosystem**: The `rand` and `clap` crates provide industry-standard, audited implementations for randomness and CLI parsing.

## 📜 License

MIT License
