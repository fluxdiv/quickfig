#!/usr/bin/env fish

## commit.group is "Docs" etc
## commit.scope is (parser) etc
# -u unreleased
# -l commits since last tag
# -x prints context of changelog as json
set COMMITS_JSON $(git-cliff -u -x -c cliff.toml --github-token "$GITCLIFF_TOKEN")

set BUMP "none"

set LIST_MODE false
if test (count $argv) -gt 0
    switch $argv[1]
        case "list" "--list"
            set LIST_MODE true
    end
end

# print divider line
function divider --argument-names char
    if test -z "$char"
        set char '='
    end
    printf '%*s\n' (tput cols) '' | tr ' ' $char
end

# major for breaking, minor for added feats, patch for fix
if echo $COMMITS_JSON | jq -e '.[].commits | .[] | select(.breaking == true)' >/dev/null
    if test $LIST_MODE = true
        divider "="
        set COMMIT_COUNT $(echo $COMMITS_JSON | jq '[.[].commits | .[] | select(.breaking == true)] | length')
        echo "$COMMIT_COUNT commits require MAJOR version bump:"
        echo
        divider "-"
        echo
        echo $COMMITS_JSON | jq -r '.[].commits | .[] | select(.breaking == true) | "\(.id) \(.raw_message)"'
        exit 0
    end
    set BUMP "major"
else if echo $COMMITS_JSON | jq -e '.[].commits | .[] | select(.group == "Added")' >/dev/null
    if test $LIST_MODE = true
        divider "="
        set COMMIT_COUNT $(echo $COMMITS_JSON | jq '[.[].commits | .[] | select(.group == "Added")] | length')
        echo "$COMMIT_COUNT commits require MINOR version bump:"
        echo
        divider "-"
        echo
        echo $COMMITS_JSON | jq -r '.[].commits | .[] | select(.group == "Added") | "\(.id) \(.raw_message)"'
        exit 0
    end
    set BUMP "minor"
else if echo $COMMITS_JSON | jq -e '.[].commits | .[] | select(.group == "Fixed")' >/dev/null
    if test $LIST_MODE = true
        divider "="
        set COMMIT_COUNT $(echo $COMMITS_JSON | jq '[.[].commits | .[] | select(.group == "Fixed")] | length')
        echo "$COMMIT_COUNT commits require PATCH version bump:"
        echo
        divider "-"
        echo
        echo $COMMITS_JSON | jq -r '.[].commits | .[] | select(.group == "Fixed") | "\(.id) \(.raw_message)"'
        exit 0
    end
    set BUMP "patch"
end

echo $BUMP
