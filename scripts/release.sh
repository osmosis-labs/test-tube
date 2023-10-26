#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
OSMOSIS_REV=$1

${SCRIPT_DIR}/update-osmosis-test-tube.sh ${OSMOSIS_REV}