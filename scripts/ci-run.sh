#!/usr/bin/env bash
# Unified CI runner - used by both local development and GitHub Actions
# This ensures perfect parity between local and remote CI

set -e

# Accept environment variable for color output
USE_COLOR="${USE_COLOR:-true}"

if [ "$USE_COLOR" = "true" ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m'
else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    NC=''
fi

FAILED=0

print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
    FAILED=1
}

# Determine what to run
RUN_FMT=true
RUN_CLIPPY=true
RUN_TESTS=true
RUN_DOC_TESTS=true
RUN_EXAMPLES_BUILD=true
RUN_EXAMPLES_RUN=false
RUN_BUILD=true

# Parse arguments
for arg in "$@"; do
    case $arg in
        --skip-examples)
            RUN_EXAMPLES_RUN=false
            ;;
        --run-examples)
            RUN_EXAMPLES_RUN=true
            ;;
        --fmt-only)
            RUN_CLIPPY=false
            RUN_TESTS=false
            RUN_DOC_TESTS=false
            RUN_EXAMPLES_BUILD=false
            RUN_BUILD=false
            ;;
        --clippy-only)
            RUN_FMT=false
            RUN_TESTS=false
            RUN_DOC_TESTS=false
            RUN_EXAMPLES_BUILD=false
            RUN_BUILD=false
            ;;
        --test-only)
            RUN_FMT=false
            RUN_CLIPPY=false
            RUN_DOC_TESTS=false
            RUN_EXAMPLES_BUILD=false
            RUN_BUILD=false
            ;;
    esac
done

# Format check
if [ "$RUN_FMT" = "true" ]; then
    print_step "Checking code formatting..."
    if cargo fmt -- --check; then
        print_success "Code is properly formatted"
    else
        print_error "Code formatting issues found. Run: cargo fmt"
    fi
fi

# Clippy lints
if [ "$RUN_CLIPPY" = "true" ]; then
    print_step "Running Clippy lints..."
    if cargo clippy --all-targets --all-features -- -D warnings; then
        print_success "No Clippy warnings"
    else
        print_error "Clippy warnings found"
    fi
fi

# Unit tests
if [ "$RUN_TESTS" = "true" ]; then
    print_step "Running unit tests..."
    if cargo test --lib --verbose; then
        print_success "All unit tests passed"
    else
        print_error "Unit tests failed"
    fi
fi

# Doc tests
if [ "$RUN_DOC_TESTS" = "true" ]; then
    print_step "Running documentation tests..."
    if cargo test --doc --verbose; then
        print_success "All doc tests passed"
    else
        print_error "Doc tests failed"
    fi
fi

# Build examples
if [ "$RUN_EXAMPLES_BUILD" = "true" ]; then
    print_step "Building examples..."
    if cargo build --examples --verbose; then
        print_success "All examples compiled"
    else
        print_error "Example compilation failed"
    fi
fi

# Run examples
if [ "$RUN_EXAMPLES_RUN" = "true" ]; then
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
fi

# Build library
if [ "$RUN_BUILD" = "true" ]; then
    print_step "Building library..."
    if cargo build --verbose; then
        print_success "Library build succeeded"
    else
        print_error "Library build failed"
    fi
    
    print_step "Building with all features..."
    if cargo build --all-features --verbose; then
        print_success "Build with all features succeeded"
    else
        print_error "Build with all features failed"
    fi
fi

# Exit with failure if any step failed
if [ $FAILED -ne 0 ]; then
    exit 1
fi

exit 0
