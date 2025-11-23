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

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    cargo install wasm-pack
fi

# Build WASM runtime first
echo ""
echo "🦀 Building WASM runtime..."
cd crates/velocity-wasm
wasm-pack build --target web
cd ../..

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

# Check if ~/.cargo/bin is in PATH and add it automatically
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "📝 Adding ~/.cargo/bin to PATH..."

    # Detect shell
    if [ -n "$BASH_VERSION" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.profile"
    fi

    # Add to shell config if not already there
    if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' "$SHELL_RC" 2>/dev/null; then
        echo "" >> "$SHELL_RC"
        echo '# Added by Velocity Framework installer' >> "$SHELL_RC"
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$SHELL_RC"
        echo "✓ Added to $SHELL_RC"
    else
        echo "✓ Already in $SHELL_RC"
    fi

    # Add to current session
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "✓ Added to current session"
    echo ""
    echo "Note: Open a new terminal or run:"
    echo "  source $SHELL_RC"
    echo ""
else
    echo "✓ ~/.cargo/bin is already in PATH"
    echo ""
fi

echo "Next steps:"
echo "  1. velocity create my-app"
echo "  2. cd my-app"
echo "  3. velocity dev"
echo ""
echo "Built with ⚡ by Velocity"
echo ""
