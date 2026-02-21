#!/bin/bash
# GitHub Remote Setup Script
# Usage: ./setup-remote.sh YOUR_GITHUB_USERNAME

if [ -z "$1" ]; then
    echo "Usage: ./setup-remote.sh YOUR_GITHUB_USERNAME"
    echo "Example: ./setup-remote.sh jake"
    exit 1
fi

USERNAME=$1
REPO_NAME="daggerheart-engine"

echo "🔗 Setting up GitHub remote..."
echo "Repository: https://github.com/$USERNAME/$REPO_NAME"
echo ""

# Add remote
git remote add origin "git@github.com:$USERNAME/$REPO_NAME.git"

echo "✅ Remote added!"
echo ""
echo "📋 Remote configuration:"
git remote -v
echo ""
echo "🚀 Ready to push! Run:"
echo "   git push -u origin master"
