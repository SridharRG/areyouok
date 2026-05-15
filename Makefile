.PHONY: help build release test clean check lint fmt install

help:
	@echo "Available targets:"
	@echo "  make build       - Build debug binary"
	@echo "  make release     - Build optimized release binary"
	@echo "  make test        - Run tests (scan current directory)"
	@echo "  make test-html   - Run tests and generate HTML report"
	@echo "  make clean       - Remove build artifacts"
	@echo "  make check       - Run cargo check"
	@echo "  make lint        - Run clippy linter"
	@echo "  make fmt         - Format code"
	@echo "  make fmt-check   - Check code formatting"

build:
	cargo build

release:
	cargo build --release

test:
	./target/debug/areyouok -r txt .

test-html:
	./target/debug/areyouok -r html .

test-json:
	./target/debug/areyouok -r json . | head -50

test-github:
	./target/debug/areyouok -r github .

clean:
	cargo clean
	rm -f areyouok_report_*.txt areyouok_report_*.html areyouok_report_*.json areyouok_report_*.md

check:
	cargo check

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

install: release
	cp target/release/areyouok ~/.local/bin/ || cp target/release/areyouok /usr/local/bin/

version:
	@grep "^version" Cargo.toml | head -1
