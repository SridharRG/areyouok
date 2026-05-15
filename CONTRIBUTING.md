# Contributing to AreYouOk

Thanks for your interest in contributing! This document provides guidelines and instructions.

## Development Setup

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Git

### Quick Start

```bash
git clone https://github.com/yourusername/areyouok.git
cd areyouok
cargo build
cargo run -- -r txt .
```

## Code Style

We follow standard Rust conventions:

- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common mistakes
- Write tests for new features

### Before submitting a PR

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Project Structure

```
src/
├── main.rs          - Application entry point
├── cli.rs           - Command-line argument parsing
├── scanner.rs       - File scanning and link extraction
├── link_checker.rs  - HTTP validation
└── report.rs        - Report generation
```

## Adding Features

### New Link Format Support

Edit `scanner.rs` and add a regex pattern to `extract_links()`:

```rust
// In the patterns vec! []
Regex::new(r"your_pattern_here")?
```

### New Report Format

Add a new format handler in `report.rs`:

```rust
"custom" => generate_custom_report(results),
```

Then implement the generator function.

### New CLI Options

Edit `cli.rs` and add to the `Args` struct:

```rust
#[arg(short = 'x', long = "example")]
pub example_option: String,
```

## Testing

Create test files in the `tests/` directory or use the existing `test_sample.md`.

Run tests:

```bash
cargo test
# Or test the CLI directly:
./target/debug/areyouok -r txt tests/
```

## Documentation

- Update README.md for user-facing changes
- Add doc comments for public functions
- Keep BUILD.md updated for build instructions

## Reporting Issues

Use GitHub Issues with:

- Clear title describing the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment (OS, Rust version, etc.)

## Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit changes: `git commit -am 'Add feature'`
4. Push to branch: `git push origin feature/my-feature`
5. Open a Pull Request

### PR Guidelines

- Keep changes focused and atomic
- Include tests for new functionality
- Update documentation as needed
- Use conventional commit messages:
  - `feat: Add new report format`
  - `fix: Handle malformed URLs`
  - `docs: Update README`
  - `refactor: Simplify link validation`

## Code of Conduct

Be respectful and constructive in all interactions. We welcome diverse perspectives and experiences.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

---

Questions? Open an issue or reach out to maintainers.
