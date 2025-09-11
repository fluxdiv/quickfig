#!/usr/bin/env bash
set -euo pipefail

# read bump type from stdin (likely piped from check_bump.sh)

if ! read -r BUMP || [[ -z "$BUMP" ]]; then
    echo "ERR: execute_bump.sh requires bump type" >&2
    exit 1
fi

# echo "Bump type: $BUMP"

if [[ "$BUMP" == "none" ]]; then
    echo "Exiting: check_bump indicated no version bump required"
    exit 0
elif [[ "$BUMP" =~ ^(major|minor|patch)$ ]]; then
    echo "Performing $BUMP version bump..."
    cargo set-version --dry-run --bump "$BUMP"
else
    echo "ERR: execute_bump received invalid bump type '$BUMP'" >&2
    echo "Possible values: major | minor | patch | none" >&2
    exit 1
fi


