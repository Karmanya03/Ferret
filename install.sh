#!/bin/bash

# Installation script for Ferret (fr)
# For Linux/Unix systems (Ubuntu, Arch, Kali, Debian, Fedora, etc.)
# Supports both system-wide install and cargo install with PATH setup

set -e

echo "=================================="
echo "Ferret 🦡 - Installer"
echo "=================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "After installation, run: source ~/.cargo/env"
    exit 1
fi

echo "✓ Rust found: $(rustc --version)"
echo ""

# Ask user for installation method
echo "Choose installation method:"
echo "  1) System-wide install to /usr/local/bin (recommended)"
echo "  2) Cargo install (user-only, auto PATH setup)"
echo ""
read -p "Enter choice (1 or 2): " choice

if [ "$choice" = "2" ]; then
    # Cargo install method
    echo ""
    echo "📦 Installing via cargo..."
    cargo install ferret-rs
    
    if [ $? -ne 0 ]; then
        echo "❌ Cargo install failed!"
        exit 1
    fi
    
    echo "✓ Installed ferret-rs"
    echo ""
    
    # Setup PATH
    CARGO_BIN="$HOME/.cargo/bin"
    
    # Check if already in PATH
    if echo "$PATH" | grep -q "$CARGO_BIN"; then
        echo "✓ $CARGO_BIN already in PATH"
    else
        echo "📝 Setting up PATH..."
        
        # Detect shell
        SHELL_RC=""
        if [ -n "$BASH_VERSION" ]; then
            SHELL_RC="$HOME/.bashrc"
        elif [ -n "$ZSH_VERSION" ]; then
            SHELL_RC="$HOME/.zshrc"
        else
            case "$SHELL" in
                */bash) SHELL_RC="$HOME/.bashrc" ;;
                */zsh) SHELL_RC="$HOME/.zshrc" ;;
                *) SHELL_RC="$HOME/.bashrc" ;;
            esac
        fi
        
        # Add PATH export if not already present
        if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' "$SHELL_RC" 2>/dev/null; then
            echo "" >> "$SHELL_RC"
            echo "# Added by Ferret installer" >> "$SHELL_RC"
            echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$SHELL_RC"
            echo "✓ Added PATH to $SHELL_RC"
        fi
        
        # Export for current session
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
else
    # System-wide install method
    echo ""
    echo "🔨 Building Ferret..."
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo "✓ Build successful!"
        echo ""
    else
        echo "❌ Build failed!"
        exit 1
    fi
    
    echo "📦 Installing to /usr/local/bin..."
    
    if [ -w /usr/local/bin ]; then
        cp target/release/fr /usr/local/bin/
        echo "✓ Installed to /usr/local/bin/fr"
    else
        echo "Need sudo privileges to install to /usr/local/bin"
        sudo cp target/release/fr /usr/local/bin/
        echo "✓ Installed to /usr/local/bin/fr"
    fi
fi

echo ""
echo "=================================="
echo "✅ Installation Complete!"
echo "=================================="
echo ""

# Verify installation
if command -v fr &> /dev/null; then
    echo "✓ 'fr' command is ready to use!"
    echo ""
    echo "Quick start:"
    echo "  fr find '*.txt'         - Find files"
    echo "  fr organize ~/Downloads - Organize files"
    echo "  fr stats                - Show directory stats"
    echo ""
    echo "Examples:"
    echo "  fr find '*config*' -i   - Find config files (case-insensitive)"
    echo "  fr find '*.log' --min-size 10M  - Find large log files"
    echo "  fr find '*.tmp' -x 'rm {} 2>/dev/null'  - Delete temp files"
    echo ""
    echo "For more help: fr --help"
    echo ""
else
    if [ "$choice" = "2" ]; then
        echo "⚠️  Installation complete but 'fr' not found yet."
        echo ""
        echo "To use 'fr' in this terminal, run:"
        echo "  source $SHELL_RC"
        echo ""
        echo "Or just open a new terminal window."
    else
        echo "⚠️  Installation completed but 'fr' not found in PATH"
        echo "You may need to add /usr/local/bin to your PATH"
    fi
    echo ""
fi
