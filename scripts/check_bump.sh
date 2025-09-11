#!/usr/bin/env bash
set -euo pipefail

## commit.group is "Docs" etc
## commit.scope is (parser) etc
# -u unreleased / only commits since last git tag
# -x prints context of changelog as json
COMMITS_JSON=$(git-cliff -u -x)

BUMP="none"

LIST_MODE=false
if [[ "${1:-}" =~ ^(list|--list)$ ]]; then
    LIST_MODE=true
fi

# Filter on group name
# git-cliff -u -x | jq '.[].commits | .[] | select(.group == "Build")'
# Filter on breaking
# git-cliff -u -x | jq '.[].commits | .[] | select(.breaking == true)'

# major for breaking, minor for added feats, patch for fix
if echo "$COMMITS_JSON" | jq -e '.[].commits | .[] | select(.breaking == true)' >/dev/null; then
    if $LIST_MODE; then
        printf '%*s\n' "$(tput cols)" '' | tr ' ' '='
        COMMIT_COUNT=$(echo "$COMMITS_JSON" | jq '[.[].commits | .[] | select(.breaking == true)] | length')
        echo "$COMMIT_COUNT commits require MAJOR version bump:"
        echo
        printf '%*s\n' "$(tput cols)" '' | tr ' ' '-'
        echo
        echo "$COMMITS_JSON" | jq -r '.[].commits | .[] | select(.breaking == true) | "\(.id) \(.raw_message)"'
        exit 0
    fi
    BUMP="major"
elif echo "$COMMITS_JSON" | jq -e '.[].commits | .[] | select(.group == "Added")' >/dev/null; then
    if $LIST_MODE; then
        printf '%*s\n' "$(tput cols)" '' | tr ' ' '='
        COMMIT_COUNT=$(echo "$COMMITS_JSON" | jq '[.[].commits | .[] | select(.group == "Added")] | length')
        echo "$COMMIT_COUNT commits require MINOR version bump:"
        echo
        printf '%*s\n' "$(tput cols)" '' | tr ' ' '-'
        echo
        echo "$COMMITS_JSON" | jq -r '.[].commits | .[] | select(.group == "Added") | "\(.id) \(.raw_message)"'
        exit 0
    fi
    BUMP="minor"
elif echo "$COMMITS_JSON" | jq -e '.[].commits | .[] | select(.group == "Fixed")' >/dev/null; then
    if $LIST_MODE; then
        printf '%*s\n' "$(tput cols)" '' | tr ' ' '='
        COMMIT_COUNT=$(echo "$COMMITS_JSON" | jq '[.[].commits | .[] | select(.group == "Fixed")] | length')
        echo "$COMMIT_COUNT commits require PATCH version bump:"
        echo
        printf '%*s\n' "$(tput cols)" '' | tr ' ' '-'
        echo
        echo "$COMMITS_JSON" | jq -r '.[].commits | .[] | select(.group == "Fixed") | "\(.id) \(.raw_message)"'
        exit 0
    fi
    BUMP="patch"
# elif echo "$COMMITS_JSON" | jq -e '.[].commits | .[] | select(.group == "Build")' >/dev/null; then
#     if $LIST_MODE; then
#         printf '%*s\n' "$(tput cols)" '' | tr ' ' '='
#         COMMIT_COUNT=$(echo "$COMMITS_JSON" | jq '[.[].commits | .[] | select(.group == "Build")] | length')
#         echo "$COMMIT_COUNT commits require BUILD version bump:"
#         echo
#         printf '%*s\n' "$(tput cols)" '' | tr ' ' '-'
#         echo
#         echo "$COMMITS_JSON" | jq -r '.[].commits | .[] | select(.group == "Build") | "\(.id) \(.raw_message)"'
#         exit 0
#     fi
#     BUMP="build"
fi

echo "$BUMP"
