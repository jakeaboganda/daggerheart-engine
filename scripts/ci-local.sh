#!/usr/bin/env bash
# Local CI script - runs the same checks as GitHub Actions
# Run this before committing to catch issues early

set -e  # Exit on first error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Track overall status
FAILED=0

# Helper functions
print_step() {
    echo -e "\n${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
    FAILED=1
}

print_warning() {
    echo -e "${YELLOW}!${NC} $1"
}

# Header
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Daggerheart Engine - Local CI Check${NC}"
echo -e "${BLUE}========================================${NC}"

# 1. Format Check
print_step "Checking code formatting..."
if cargo fmt -- --check; then
    print_success "Code is properly formatted"
else
    print_error "Code formatting issues found. Run: cargo fmt"
fi

# 2. Clippy Lints
print_step "Running Clippy lints..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_success "No Clippy warnings"
else
    print_error "Clippy warnings found. Fix them before committing"
fi

# 3. Unit Tests
print_step "Running unit tests..."
if cargo test --lib; then
    print_success "All unit tests passed"
else
    print_error "Unit tests failed"
fi

# 4. Doc Tests
print_step "Running documentation tests..."
if cargo test --doc; then
    print_success "All doc tests passed"
else
    print_error "Doc tests failed"
fi

# 5. Build Examples
print_step "Building examples..."
if cargo build --examples; then
    print_success "All examples compiled"
else
    print_error "Example compilation failed"
fi

# 6. Run Examples (optional - can be slow)
if [ "$1" != "--skip-examples" ]; then
    print_step "Running examples..."
    
    EXAMPLES=("basic_dice" "duality_dice" "weapon_damage" "combat_scenario")
    for example in "${EXAMPLES[@]}"; do
        echo -n "  Running $example... "
        if cargo run --example "$example" > /dev/null 2>&1; then
            echo -e "${GREEN}✓${NC}"
        else
            echo -e "${RED}✗${NC}"
            print_error "Example $example failed to run"
        fi
    done
else
    print_warning "Skipping example runs (use without --skip-examples to run)"
fi

# 7. Build Library
print_step "Building library..."
if cargo build; then
    print_success "Library build succeeded"
else
    print_error "Library build failed"
fi

# 8. Build with all features
print_step "Building with all features..."
if cargo build --all-features; then
    print_success "Build with all features succeeded"
else
    print_error "Build with all features failed"
fi

# Summary
echo -e "\n${BLUE}========================================${NC}"
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All checks passed! Safe to commit.${NC}"
    echo -e "${BLUE}========================================${NC}"
    exit 0
else
    echo -e "${RED}✗ Some checks failed. Fix issues before committing.${NC}"
    echo -e "${BLUE}========================================${NC}"
    exit 1
fi
