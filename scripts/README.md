# Development Scripts

Helper scripts for local development and CI checks.

## Quick Reference

```bash
# Run full CI locally (recommended before pushing)
./scripts/ci-local.sh

# Run quick checks only (fast, for frequent use)
./scripts/ci-quick.sh

# Install Git pre-commit hook (runs quick checks automatically)
./scripts/install-hooks.sh
```

---

## Scripts

### `ci-local.sh` - Full CI Check

Runs **all** the same checks as GitHub Actions CI:

**Checks:**
1. ✅ Code formatting (`cargo fmt -- --check`)
2. ✅ Clippy lints (`cargo clippy -D warnings`)
3. ✅ Unit tests (`cargo test --lib`)
4. ✅ Doc tests (`cargo test --doc`)
5. ✅ Examples compilation (`cargo build --examples`)
6. ✅ Examples execution (all 4 examples)
7. ✅ Library build (`cargo build`)
8. ✅ All features build (`cargo build --all-features`)

**Usage:**
```bash
# Full check (includes running examples)
./scripts/ci-local.sh

# Skip example execution (faster)
./scripts/ci-local.sh --skip-examples
```

**Time:** ~3-5 minutes (with cache)

**When to use:**
- Before pushing to GitHub
- Before creating a PR
- After making significant changes
- To ensure all CI checks will pass

---

### `ci-quick.sh` - Quick Check

Runs only the **fast** checks:

**Checks:**
1. ✅ Code formatting
2. ✅ Clippy lints
3. ✅ Compilation check

**Usage:**
```bash
./scripts/ci-quick.sh
```

**Time:** ~30-60 seconds (with cache)

**When to use:**
- Before each commit
- Frequent quick validation
- Pre-commit hook (automatic)
- During active development

---

### `install-hooks.sh` - Install Git Hooks

Installs the pre-commit hook that runs `ci-quick.sh` automatically.

**Usage:**
```bash
./scripts/install-hooks.sh
```

**What it does:**
- Copies `git-hooks/pre-commit` to `.git/hooks/pre-commit`
- Makes it executable
- Asks before overwriting existing hooks

**After installation:**
- Quick checks run automatically on `git commit`
- Prevents committing code that fails basic checks
- Can bypass with: `git commit --no-verify`

---

## Workflow Recommendations

### Option 1: Manual (No Hook)

Run checks manually before committing:

```bash
# During development
./scripts/ci-quick.sh

# Before pushing
./scripts/ci-local.sh
```

**Pros:** Full control, run when you want  
**Cons:** Easy to forget

---

### Option 2: Pre-Commit Hook (Recommended)

Install the hook once:

```bash
./scripts/install-hooks.sh
```

Then commit normally:

```bash
git add .
git commit -m "feat: add new feature"
# Hook runs automatically!
```

**Pros:** Never forget, catches issues early  
**Cons:** Adds ~30s to each commit

**Bypass when needed:**
```bash
git commit --no-verify -m "WIP: save progress"
```

---

### Option 3: Hybrid

Use hook for quick checks + manual full check:

```bash
# Install hook for automatic quick checks
./scripts/install-hooks.sh

# Run full CI before pushing
./scripts/ci-local.sh
git push
```

**Pros:** Best of both worlds  
**Cons:** Requires discipline

---

## Troubleshooting

### Script won't run

```bash
# Make sure scripts are executable
chmod +x scripts/*.sh
chmod +x scripts/git-hooks/pre-commit
```

### Pre-commit hook not triggering

```bash
# Check if hook is installed
ls -la .git/hooks/pre-commit

# Reinstall if needed
./scripts/install-hooks.sh
```

### Checks fail but you need to commit anyway

```bash
# Bypass pre-commit hook
git commit --no-verify -m "WIP: work in progress"

# Note: CI will still run on push!
```

### Cache issues / slow runs

```bash
# Clean build cache
cargo clean

# Clear all caches
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git
```

---

## CI vs Local Checks

### What's the same?

- ✅ Format checking
- ✅ Clippy lints
- ✅ Test suite
- ✅ Example builds
- ✅ Library builds

### What's different?

**Local scripts:**
- Run on your machine
- Use your local Rust version
- Immediate feedback
- No GitHub Actions minutes used

**GitHub Actions CI:**
- Runs on Ubuntu Linux
- Uses stable Rust (latest)
- Runs on every push/PR
- Enforced for merging

**Best practice:** Run local checks before pushing to catch issues early!

---

## Adding Your Own Checks

Edit `scripts/ci-local.sh` or `scripts/ci-quick.sh`:

```bash
# Example: Add security audit
print_step "Running security audit..."
if cargo audit; then
    print_success "No security vulnerabilities"
else
    print_error "Security issues found"
fi
```

---

## Git Hook Details

### What is a pre-commit hook?

A script that runs automatically before `git commit` completes.

**Located at:** `.git/hooks/pre-commit`

**Behavior:**
- Exit code 0 → commit proceeds
- Exit code 1 → commit aborted

**Bypass:**
```bash
git commit --no-verify
```

### Why use hooks?

**Benefits:**
- ✅ Catch issues before committing
- ✅ Maintain consistent code quality
- ✅ Save time (find issues locally vs in CI)
- ✅ Prevent "fix CI" commits

**Drawbacks:**
- ⚠️ Adds time to commits (~30s)
- ⚠️ Can be annoying during rapid iteration

**Solution:** Use quick checks in hook, full checks before push!

---

## Performance Tips

### Speed up local checks

1. **Use `--skip-examples`:**
   ```bash
   ./scripts/ci-local.sh --skip-examples
   ```

2. **Use quick check frequently:**
   ```bash
   ./scripts/ci-quick.sh
   ```

3. **Keep dependencies cached:**
   ```bash
   # Don't run cargo clean unless necessary
   ```

4. **Use `cargo check` instead of `cargo build` during development:**
   ```bash
   cargo check  # Faster, just checks compilation
   ```

---

## Integration with IDEs

### VS Code

Add to `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "CI Quick Check",
      "type": "shell",
      "command": "./scripts/ci-quick.sh",
      "group": "test",
      "presentation": {
        "reveal": "always",
        "panel": "new"
      }
    }
  ]
}
```

Run with: `Cmd/Ctrl + Shift + P` → "Run Task" → "CI Quick Check"

---

## Summary

**For daily development:**
```bash
./scripts/ci-quick.sh  # Fast checks
```

**Before pushing:**
```bash
./scripts/ci-local.sh  # Full checks
```

**One-time setup:**
```bash
./scripts/install-hooks.sh  # Auto-run quick checks on commit
```

**Emergency bypass:**
```bash
git commit --no-verify  # Skip hook when needed
```

---

**Questions? Check:**
- CI/CD_SETUP.md - Full CI/CD documentation
- .github/workflows/ci.yml - Exact CI configuration
