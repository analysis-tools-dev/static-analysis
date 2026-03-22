# Static Analysis Tools Repository Makefile

.PHONY: render render-skip-deprecated check clippy fmt test clean help

# Default target shows help
help:
	@echo "Available targets:"
	@echo "  render              - Render README.md and JSON API from YAML sources"
	@echo "  render-skip-deprecated - Render without deprecated tools"
	@echo "  check               - Run cargo check"
	@echo "  clippy              - Run clippy lints"
	@echo "  fmt                 - Format Rust code"
	@echo "  test                - Run tests"
	@echo "  clean               - Clean build artifacts"
	@echo "  help                - Show this help"

# Main rendering targets
render:
	cargo run --manifest-path ci/Cargo.toml -p render -- --tags data/tags.yml --tools data/tools --md-out README.md --json-out data/api

render-skip-deprecated:
	cargo run --manifest-path ci/Cargo.toml -p render -- --tags data/tags.yml --tools data/tools --md-out README.md --json-out data/api --skip-deprecated

# Development targets
check:
	cargo check --manifest-path ci/Cargo.toml

clippy:
	cargo clippy --manifest-path ci/Cargo.toml -- -D warnings

fmt:
	cargo fmt --manifest-path ci/Cargo.toml

test:
	cargo test --manifest-path ci/Cargo.toml

clean:
	cargo clean --manifest-path ci/Cargo.toml