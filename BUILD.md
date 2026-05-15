# Building AreYouOk

This document describes how to build AreYouOk binaries for different platforms.

## Prerequisites

- Rust 1.70 or later: [Install Rust](https://www.rust-lang.org/tools/install)
- For cross-platform builds: `cargo install cross`

## Quick Build

### Development Build (debug mode)

```bash
cargo build
# Binary: target/debug/areyouok
```

### Release Build (optimized)

```bash
cargo build --release
# Binary: target/release/areyouok
```

## Cross-Platform Builds

Install `cross` for easy cross-compilation:

```bash
cargo install cross
```

### Build for Linux (amd64)

```bash
cross build --release --target x86_64-unknown-linux-gnu
# Binary: target/x86_64-unknown-linux-gnu/release/areyouok
```

### Build for macOS (amd64)

On an Apple Silicon Mac, you need to target Intel:

```bash
cross build --release --target x86_64-apple-darwin
# Binary: target/x86_64-apple-darwin/release/areyouok
```

Or natively (Apple Silicon):

```bash
cargo build --release --target aarch64-apple-darwin
# Binary: target/aarch64-apple-darwin/release/areyouok
```

### Build for Windows

On Windows:

```bash
cargo build --release --target x86_64-pc-windows-msvc
# Binary: target/x86_64-pc-windows-msvc/release/areyouok.exe
```

Or on Linux/macOS (requires MSVC toolchain):

```bash
cross build --release --target x86_64-pc-windows-msvc
```

## Creating Release Artifacts

Create a `build.sh` script to automate multi-platform builds:

```bash
#!/bin/bash
set -e

VERSION="0.1.0"
TARGETS=(
  "x86_64-unknown-linux-gnu:linux-amd64"
  "x86_64-apple-darwin:macos-amd64"
  "aarch64-apple-darwin:macos-arm64"
  "x86_64-pc-windows-msvc:windows-amd64"
)

mkdir -p dist

for target_pair in "${TARGETS[@]}"; do
  IFS=':' read -r target name <<< "$target_pair"
  echo "Building for $name ($target)..."
  
  cross build --release --target "$target"
  
  binary_path="target/$target/release/areyouok"
  if [[ "$name" == *"windows"* ]]; then
    binary_path="$binary_path.exe"
  fi
  
  output_name="areyouok-$name"
  if [[ "$name" == *"windows"* ]]; then
    output_name="$output_name.exe"
  fi
  
  cp "$binary_path" "dist/$output_name"
  echo "✓ Created dist/$output_name"
done

echo ""
echo "Release artifacts ready in ./dist/"
ls -lh dist/
```

Make it executable:

```bash
chmod +x build.sh
./build.sh
```

## Size Optimization

The release build uses Link Time Optimization (LTO) and other optimizations. Check `Cargo.toml` for settings:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

For even smaller binaries:

```bash
cargo build --release -Z build-std=std,panic_abort --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/areyouok
```

## Verification

After building, verify the binary works:

```bash
./target/release/areyouok --help
./target/release/areyouok -r txt .
```

## CI/CD Integration

The `.github/workflows/link-check.yml` includes a step to download prebuilt binaries from releases. To set this up:

1. Build binaries for all platforms (using `build.sh`)
2. Create a GitHub Release with version tag `v0.1.0`
3. Upload all binaries from `dist/` to the release
4. The workflow will automatically download and use them

## Troubleshooting

### Cross-compilation issues on macOS

If you encounter issues with Apple Silicon, install Rosetta 2:

```bash
softwareupdate --install-rosetta
```

### Windows MSVC toolchain

Ensure you have the Windows SDK and MSVC installed. Run:

```bash
rustup target add x86_64-pc-windows-msvc
```

### Binary size too large

Strip the binary:

```bash
strip target/release/areyouok
```

This removes debug symbols. For Linux:

```bash
strip --strip-all areyouok
```

---

For questions or issues, see the main README.md.
