#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
OSMOSIS_REV=${1:-main}

LATEST_OSMOSIS_VERSION="v16"

# if "$OSMOIS_REV" is /v\d+/ then extract it as var
if [[ "$OSMOSIS_REV" =~ ^v[0-9]+ ]]; then
  OSMOSIS_VERSION=$(echo "$OSMOSIS_REV" | sed "s/\..*$//")
else
  OSMOSIS_VERSION="$LATEST_OSMOSIS_VERSION"
fi

########################################
## Update and rebuild osmosis-test-tube ##
########################################

# update all submodules
git submodule update --init --recursive
cd "$SCRIPT_DIR/../packages/osmosis-test-tube/osmosis"

OSMOSIS_REV_NO_ORIGIN="$(echo "$OSMOSIS_REV" | sed "s/^origin\///")"

git checkout "$OSMOSIS_REV_NO_ORIGIN"


# build and run update-osmosis-test-tube
cd "$SCRIPT_DIR/update-osmosis-test-tube-deps" && go build

# run update-osmosis-test-tube-deps which will replace the `replace directives` in osmosis-test-tube
# with osmosis' replaces
"$SCRIPT_DIR/update-osmosis-test-tube-deps/update-osmosis-test-tube-deps" "$OSMOSIS_REV_NO_ORIGIN"

cd "$SCRIPT_DIR/../packages/osmosis-test-tube/osmosis"
PARSED_REV=$(git rev-parse --short "$OSMOSIS_REV")

cd "$SCRIPT_DIR/../packages/osmosis-test-tube/libosmosistesttube"

go get "github.com/osmosis-labs/osmosis/${OSMOSIS_VERSION}@${PARSED_REV}"

# tidy up updated go.mod
go mod tidy


########################################
## Update git revision if there is    ##
## any change                         ##
########################################

if [[ -n "${SKIP_GIT_UPDATE:-}" ]]; then
  echo '[SKIP] SKIP_GIT_UPDATE is set, skipping git update'
  exit 0
fi

# if dirty or untracked file exists
if [[ $(git diff --stat) != '' ||  $(git ls-files  --exclude-standard  --others) ]]; then
  # add, commit and push
  git add "$SCRIPT_DIR/.."
  git commit -m "rebuild with $(git rev-parse --short HEAD:dependencies/osmosis)"

  # remove "origin/"
  OSMOSIS_REV=$(echo "$OSMOSIS_REV" | sed "s/^origin\///")
  BRANCH="autobuild-$OSMOSIS_REV"

  # force delete local "$BRANCH" if exists
  git branch -D "$BRANCH" || true

  git checkout -b "$BRANCH"
  git push -uf origin "$BRANCH"
else
  echo '[CLEAN] No update needed for this build'
fi
