# 🔍 AreYouOk

A fast, standalone Rust tool to scan files for broken or dead links. Works with Markdown, HTML, and any text files.

## Features

✅ **Multi-format support** - Scan Markdown, HTML, or any text files  
✅ **Flexible filtering** - Ignore specific directories and files  
✅ **Multiple report formats** - JSON, HTML, TXT, and GitHub-flavored Markdown  
✅ **Fast link validation** - Async HTTP checking with timeout protection  
✅ **Cross-platform** - Works on Linux, macOS, and Windows  
✅ **Standalone binary** - No dependencies required, just download and run  

## Installation

### Download Pre-built Binary

Visit [Releases](https://github.com/yourusername/areyouok/releases) and download the binary for your platform:

- **Linux** (amd64): `areyouok-linux-amd64`
- **macOS** (amd64): `areyouok-macos-amd64`
- **Windows** (amd64): `areyouok-windows-amd64.exe`

After downloading, make the binary executable (on Linux/macOS):

```bash
chmod +x areyouok-linux-amd64
```

### Build from Source

Requires [Rust 1.70+](https://www.rust-lang.org/tools/install)

```bash
git clone https://github.com/yourusername/areyouok.git
cd areyouok
cargo build --release
```

The binary will be at `target/release/areyouok` (or `areyouok.exe` on Windows).

## Usage

### Basic Usage

```bash
./areyouok ./docs
```

Scans all Markdown files in `./docs` and subdirectories for broken links.

### Command-line Options

```
USAGE:
    areyouok [OPTIONS] <PATH>

OPTIONS:
    -t, --type <TYPE>
        Type of files to scan [default: markdown]
        Options: markdown, html, text, or specific extensions (e.g., "md,txt")

    -i, --ignore <PATTERNS>
        Files or directories to ignore (comma-separated)
        Example: "node_modules,.git,_site"

    -r, --report <FORMAT>
        Report format to generate [default: txt]
        Options: json, html, txt, github

    -h, --help
        Print help information

ARGS:
    <PATH>
        Directory path to scan
```

### Examples

#### Scan with file type filter

```bash
./areyouok -t html ./website
```

#### Scan with ignore patterns

```bash
./areyouok -i "node_modules,.git,_site,README.md" ./project
```

#### Generate HTML report

```bash
./areyouok -r html ./docs
```

#### Multiple file types

```bash
./areyouok -t "md,txt,html" ./content
```

#### Complete example (Jekyll blog)

```bash
./areyouok \
  -t markdown \
  -i "_layouts,.git,_site,README.md,USAGE.md,build.py" \
  -r html \
  ./blog
```

This generates an interactive HTML report at `areyouok_report_YYYY-MM-DD_HH-MM-SS.html`

## Report Formats

### TXT (Default)
Human-readable text format with summary and broken links listing.

### JSON
Structured JSON output containing all link validation results. Useful for CI/CD pipelines and automation.

```json
[
  {
    "url": "https://example.com",
    "status_code": 404,
    "is_valid": false,
    "error_message": "HTTP 404",
    "files": [
      {
        "file": "docs/index.md",
        "line": 42
      }
    ]
  }
]
```

### HTML
Visual, interactive report with statistics and color-coded results. Perfect for web viewing.

### GitHub
GitHub-flavored Markdown format compatible with GitHub Issues and Pull Requests.

## CI/CD Integration

### GitHub Actions

Create `.github/workflows/link-check.yml`:

```yaml
name: Check Links

on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday
  workflow_dispatch:

jobs:
  check-links:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Download AreYouOk
        run: |
          curl -L https://github.com/yourusername/areyouok/releases/download/v0.1.0/areyouok-linux-amd64 -o areyouok
          chmod +x areyouok
      
      - name: Check links
        run: ./areyouok -r github .
        continue-on-error: true
      
      - name: Upload report
        uses: actions/upload-artifact@v4
        with:
          name: link-report
          path: areyouok_report_*.html
```

### Generic CI/CD (Every 30 days)

```bash
#!/bin/bash
set -e

# Download fresh copy
curl -L https://github.com/yourusername/areyouok/releases/download/v0.1.0/areyouok-linux-amd64 -o areyouok
chmod +x areyouok

# Run scan
./areyouok -r github . > report.md || true

# Upload/notify on broken links
if grep -q "Broken Links" report.md; then
  echo "⚠️ Found broken links in the repository"
  # Send notification, create issue, etc.
fi
```

## How It Works

1. **Scan** - Recursively walks through directories, identifying files matching your criteria
2. **Extract** - Uses regex patterns to find URLs in Markdown, HTML, and text files
3. **Validate** - Makes HEAD requests to check if URLs are accessible
4. **Report** - Generates a report in your chosen format with results and locations

### Supported Link Formats

- **Markdown links**: `[text](https://example.com)`
- **HTML attributes**: `href="https://example.com"`
- **Plain URLs**: `https://example.com`

## Performance

- **Fast scanning** - Walks directories efficiently, skips ignored paths
- **Async validation** - Checks multiple links in parallel
- **Deduplication** - Avoids re-checking duplicate URLs
- **Configurable timeouts** - 10-second timeout per link (default)

## Troubleshooting

### SSL/Certificate errors

Some websites have SSL issues. The tool will report these as broken links.

### Timeout errors

If a site responds slowly, it may timeout. Adjust the request timeout in the code if needed.

### Rate limiting

Some servers rate-limit requests. Space out link checks by running the tool at different times.

## License

See LICENSE file for details.

## Contributing

Contributions welcome! Please submit issues and pull requests.

---

**AreYouOk v0.1.0** - Made with 🦀 Rust
