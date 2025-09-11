#!/usr/bin/env bash
set -euo pipefail

print_header() {
    echo "=========================="
    echo "=========================="
    echo "$1"
    echo "=========================="
}

# Prior to a release commit merge everything for release into main branch, 
# then from main branch:

# verify on main branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$BRANCH" != "main" ]]; then
    echo "ERROR: Must be on main branch to run release flow (current: $BRANCH)"
    echo "Prior to release commit merge everything for the release into main branch"
    echo "Then run release.sh on main branch"
    exit 1
fi

# verify at quickfig workspace root
ROOT=$(cargo metadata --no-deps --format-version=1 | jq -r '.workspace_root')
if [ "$ROOT" != "$(pwd)" ]; then
    echo "ERROR: Run release.sh from workspace root"
    echo "Workspace root: $ROOT"
    echo "Current/pwd: $(pwd)"
    exit 1
fi

# must run from root of quickfig
# - bump all cargo.tomls following semver by checking commit messages
print_header "Bumping cargo.toml files"
./scripts/check_bump.sh | ./scripts/execute_bump.sh

# - get new version num
print_header "Checking version number"
NEW_VERSION=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="quickfig") | .version')

# - use NEW_VERSION with git-cliff to add entry to changelog
print_header "Updating changelog"
git-cliff -c cliff.toml --tag v$NEW_VERSION -o CHANGELOG.md --github-token "$GITCLIFF_TOKEN" --latest

# - Create the release commit (includes cargo bumps & CHANGELOG update)
print_header "Creating release commit"
git add .
git commit -m "chore: release v$NEW_VERSION"
git push origin main

# - Tag it
print_header "Creating git tag"
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
git push origin main --tags

# - Create Github release
print_header "Creating Github release"
gh release create v$NEW_VERSION --title "$NEW_VERSION" \
--notes "See [CHANGELOG.md](https://github.com/fluxdiv/quickfig/blob/main/CHANGELOG.md)"
# - `gh release create v0.1.0 --title "v0.1.0" --notes "See [CHANGELOG.md](https://github.com/<your-username>/<repo-name>/blob/main/CHANGELOG.md)"`
#   - `v0.1.0` must match the tag I just pushed
#   - `--title` is usually just the version
#   - `--notes` what shows in github release description

# - Publish to crates.io (quickfig_core, then quickfig_derive, then quickfig)
print_header "Publishing quickfig_core to crates.io"
cargo publish --package quickfig_core
print_header "Waiting 20 sec between publishing crates..."
sleep 20

print_header "Publishing quickfig_derive to crates.io"
cargo publish --package quickfig_derive
print_header "Waiting 20 sec between publishing crates..."
sleep 20

print_header "Publishing quickfig to crates.io"
cargo publish --package quickfig
sleep 5

echo "=========================================================="
echo "=========================================================="
echo ""
echo "Release v$NEW_VERSION complete"
echo ""
echo "=========================================================="
echo "=========================================================="


