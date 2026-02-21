#!/usr/bin/env bash
# Local CI script - runs the same checks as GitHub Actions
# Run this before committing to catch issues early

echo -e "\033[0;34m========================================\033[0m"
echo -e "\033[0;34m  Daggerheart Engine - Local CI Check\033[0m"
echo -e "\033[0;34m========================================\033[0m"
echo ""

# Run the unified CI script
USE_COLOR=true ./scripts/ci-run.sh "$@"
EXIT_CODE=$?

echo ""
if [ $EXIT_CODE -eq 0 ]; then
    echo -e "\033[0;34m========================================\033[0m"
    echo -e "\033[0;32m✓ All checks passed! Safe to commit.\033[0m"
    echo -e "\033[0;34m========================================\033[0m"
else
    echo -e "\033[0;34m========================================\033[0m"
    echo -e "\033[0;31m✗ Some checks failed. Fix issues before committing.\033[0m"
    echo -e "\033[0;34m========================================\033[0m"
fi

exit $EXIT_CODE
