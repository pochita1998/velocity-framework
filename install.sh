#!/bin/bash

# Velocity Framework Installation Script

set -e

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⚡ Velocity Framework Installer"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found!"
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

echo "✓ Cargo found"

# Build and install
echo ""
echo "📦 Building Velocity CLI..."
cargo build --release

echo ""
echo "🔧 Installing globally..."
cargo install --path crates/velocity-cli

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Installation Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if ~/.cargo/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "⚠️  ~/.cargo/bin is not in your PATH"
    echo ""
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    echo ""
    echo "Then run: source ~/.bashrc (or ~/.zshrc)"
    echo ""
else
    echo "✓ ~/.cargo/bin is in PATH"
    echo ""
fi

echo "Next steps:"
echo "  1. velocity create my-app"
echo "  2. cd my-app"
echo "  3. velocity dev"
echo ""
echo "Built with ⚡ by Velocity"
echo ""
