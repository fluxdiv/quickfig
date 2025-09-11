#!/usr/bin/env bash
set -euo pipefail

# Prior to a release commit merge everything for release into main branch, 
# then from main branch:
#
# must run from root of quickfig
# - bump all cargo.tomls following semver by checking commit messages
./scripts/check_bump.sh | ./scripts/execute_bump.sh

# - get new version num
NEW_VERSION=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="quickfig") | .version')

# - use NEW_VERSION with git-cliff to add entry to changelog
git-cliff -c cliff.toml --tag v$NEW_VERSION -o CHANGELOG.md 

# - Create the release commit (includes cargo bumps & CHANGELOG update)
# git commit -m "chore: release v$NEW_VERSION"

# - Publish to crates.io (quickfig_core, then quickfig_derive, then quickfig)
cargo publish --package quickfig_core --dry-run
cargo publish --package quickfig_derive --dry-run
cargo publish --package quickfig --dry-run

# - Tag it
# git tag -a v$NEW_VERSION -m Release v$NEW_VERSION
# git push origin main --tags
#
# - Create Github release
# gh release create v$NEW_VERSION --title "$NEW_VERSION" \
# --notes "See [CHANGELOG.md](https://github.com/fluxdiv/quickfig/blob/main/CHANGELOG.md)"
#
# - `gh release create v0.1.0 --title "v0.1.0" --notes "See [CHANGELOG.md](https://github.com/<your-username>/<repo-name>/blob/main/CHANGELOG.md)"`
#   - `v0.1.0` must match the tag I just pushed
#   - `--title` is usually just the version
#   - `--notes` what shows in github release description
#
# - Publish to crates.io
