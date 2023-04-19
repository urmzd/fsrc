#!/usr/bin/env bash

COMMIT_MESSAGE="$1"
FILES="${@:2}"

# Configure git
git config user.name "embed-md bot"
git config user.email "embed-md-bot@urmzd.com"
git config --add safe.directory "$(git rev-parse --show-toplevel)"
git config pull.rebase true
git config rebase.autoStash true

# Embed files
echo "Embedding files: $FILES"
npx embedme "$FILES"

# Commit changes
echo "Commiting with message: $COMMIT_MESSAGE"

# Pull and apply changes
git pull

# Stage and commit the changes
git add "$FILES"
git commit -m "$COMMIT_MESSAGE"

# Push the changes to the current branch
git push
