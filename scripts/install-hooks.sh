#!/usr/bin/env bash
# Install Git hooks for Daggerheart Engine

set -e

REPO_ROOT=$(git rev-parse --show-toplevel)
HOOK_SOURCE="$REPO_ROOT/scripts/git-hooks/pre-commit"
HOOK_TARGET="$REPO_ROOT/.git/hooks/pre-commit"

echo "Installing Git pre-commit hook..."

# Check if hook already exists
if [ -f "$HOOK_TARGET" ]; then
    echo "⚠️  Pre-commit hook already exists at: $HOOK_TARGET"
    read -p "Overwrite? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Installation cancelled"
        exit 1
    fi
    rm "$HOOK_TARGET"
fi

# Copy hook
cp "$HOOK_SOURCE" "$HOOK_TARGET"
chmod +x "$HOOK_TARGET"

echo "✅ Pre-commit hook installed!"
echo ""
echo "The hook will run before each commit to ensure code quality."
echo "To bypass: git commit --no-verify"
echo ""
