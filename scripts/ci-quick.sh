#!/usr/bin/env bash
# Quick CI check - runs only the fast checks
# Use this for pre-commit hooks or quick validation

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

FAILED=0

print_step() {
    echo -e "${BLUE}→${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
    FAILED=1
}

echo -e "${BLUE}Quick CI Check${NC}"

# 1. Format (fast)
print_step "Checking format..."
if cargo fmt -- --check > /dev/null 2>&1; then
    print_success "Format OK"
else
    print_error "Format issues (run: cargo fmt)"
fi

# 2. Clippy (medium)
print_step "Checking clippy..."
if cargo clippy --all-targets --all-features -- -D warnings 2>&1 > /dev/null; then
    print_success "Clippy OK"
else
    print_error "Clippy warnings found"
fi

# 3. Quick compile check
print_step "Checking compilation..."
if cargo check > /dev/null 2>&1; then
    print_success "Compiles OK"
else
    print_error "Compilation failed"
fi

# Summary
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ Quick checks passed${NC}"
    exit 0
else
    echo -e "${RED}✗ Quick checks failed${NC}"
    exit 1
fi
