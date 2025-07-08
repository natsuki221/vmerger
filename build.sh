#!/bin/bash

# Build script for vmerger-cli
# This script handles building, testing, and installation

set -e

echo "🚀 Building vmerger-cli..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

print_status "Rust version: $(rustc --version)"

# Check if FFmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    print_warning "FFmpeg is not installed. The tool will not work without FFmpeg."
    print_warning "Please install FFmpeg from https://ffmpeg.org/"
else
    print_status "FFmpeg version: $(ffmpeg -version | head -n 1)"
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Format code
echo "🎨 Formatting code..."
cargo fmt

# Run clippy for linting
echo "🔍 Running clippy..."
cargo clippy -- -D warnings

# Run tests
echo "🧪 Running tests..."
cargo test

# Build in release mode
echo "🔨 Building in release mode..."
cargo build --release

# Check if binary was created
if [ -f "target/release/vmerger" ]; then
    print_status "Build successful!"
    print_status "Binary location: target/release/vmerger"
    
    # Show binary size
    BINARY_SIZE=$(ls -lh target/release/vmerger | awk '{print $5}')
    print_status "Binary size: $BINARY_SIZE"
    
    # Option to install
    echo ""
    read -p "Do you want to install vmerger to your system? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "📦 Installing vmerger..."
        cargo install --path .
        print_status "vmerger installed successfully!"
        print_status "You can now use 'vmerger' command anywhere!"
    fi
else
    print_error "Build failed!"
    exit 1
fi

echo ""
echo "🎉 All done!"
echo "To test the installation, run: vmerger --help"