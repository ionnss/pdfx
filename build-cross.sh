#!/bin/bash
# Cross-compilation script for pdfx

set -e

echo "🚀 Building pdfx for multiple platforms..."

# Create release directory
mkdir -p release

# Current platform (macOS)
echo "📦 Building for macOS (current platform)..."
cargo build --release
cp target/release/pdfx release/pdfx-v0.1.0-$(rustc -vV | grep host | cut -d' ' -f2)

# Try Linux with Docker (if Docker is available)
if command -v docker &> /dev/null; then
    echo "🐋 Building for Linux using Docker..."
    docker run --rm \
        -v "$(pwd)":/workspace \
        -w /workspace \
        rust:1.70 \
        bash -c "cargo build --release --target x86_64-unknown-linux-gnu && cp target/x86_64-unknown-linux-gnu/release/pdfx release/pdfx-v0.1.0-x86_64-unknown-linux-gnu"
else
    echo "⚠️  Docker not available - skipping Linux build"
fi

# Windows cross-compilation (requires mingw)
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "🪟 Building for Windows..."
    rustup target add x86_64-pc-windows-gnu
    cargo build --release --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/release/pdfx.exe release/pdfx-v0.1.0-x86_64-pc-windows-gnu.exe
else
    echo "⚠️  MinGW not available - skipping Windows build"
fi

echo "✅ Build complete! Check the 'release/' directory."
ls -la release/
