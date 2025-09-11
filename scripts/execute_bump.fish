#!/usr/bin/env fish

# read bump type from stdin (likely piped from check_bump.fish)
read BUMP

if test -z "$BUMP"
    echo "ERR: execute_bump.fish requires bump type" >&2
    exit 1
end

# echo "Bump type: $BUMP"
switch $BUMP
    case none
        echo "Exiting: check_bump indicated no version bump required"
        exit 0
    case major minor patch
        echo "Performing $BUMP version bump..."
        cargo set-version --bump $BUMP
    case '*'
        echo "ERR: execute_bump.fish received invalid bump type '$BUMP'" >&2
        echo "Possible values: major | minor | patch | none" >&2
        exit 1
end



