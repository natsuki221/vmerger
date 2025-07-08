# Makefile for vmerger-cli

.PHONY: build test clean install format lint help dev release

# Default target
all: format lint test build

# Build the project
build:
	@echo "🔨 Building project..."
	cargo build

# Build release version
release:
	@echo "🚀 Building release version..."
	cargo build --release

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Install to system
install:
	@echo "📦 Installing vmerger..."
	cargo install --path .

# Format code
format:
	@echo "🎨 Formatting code..."
	cargo fmt

# Run clippy linter
lint:
	@echo "🔍 Running clippy..."
	cargo clippy -- -D warnings

# Development build and run
dev:
	@echo "🔧 Development build..."
	cargo build
	@echo "Run with: cargo run -- --help"

# Check for security advisories
audit:
	@echo "🔒 Checking for security advisories..."
	cargo audit

# Update dependencies
update:
	@echo "📦 Updating dependencies..."
	cargo update

# Generate documentation
doc:
	@echo "📚 Generating documentation..."
	cargo doc --open

# Run benchmarks (if any)
bench:
	@echo "📊 Running benchmarks..."
	cargo bench

# Create a new release
dist: clean format lint test release
	@echo "📦 Creating distribution..."
	@mkdir -p dist
	@cp target/release/vmerger dist/
	@cp README.md dist/
	@echo "✅ Distribution created in dist/ directory"

# Run integration tests with real videos
integration:
	@echo "🎬 Running integration tests..."
	cargo test --features integration

# Check code coverage
coverage:
	@echo "📊 Checking code coverage..."
	cargo tarpaulin --out Html

# Help target
help:
	@echo "Available targets:"
	@echo "  build      - Build the project"
	@echo "  release    - Build release version"
	@echo "  test       - Run tests"
	@echo "  clean      - Clean build artifacts"
	@echo "  install    - Install to system"
	@echo "  format     - Format code"
	@echo "  lint       - Run clippy linter"
	@echo "  dev        - Development build"
	@echo "  audit      - Security audit"
	@echo "  update     - Update dependencies"
	@echo "  doc        - Generate documentation"
	@echo "  bench      - Run benchmarks"
	@echo "  dist       - Create distribution"
	@echo "  integration- Run integration tests"
	@echo "  coverage   - Check code coverage"
	@echo "  help       - Show this help"