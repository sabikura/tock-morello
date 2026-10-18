#!/usr/bin/env bash
set -euo pipefail

: "${CRABILITY_BIN:=$HOME/.crability/bin}"

if [ $# -lt 1 ]; then
    echo "usage: $0 <path-to-elf>" >&2
    exit 1
fi

exec "$CRABILITY_BIN/qemu-system-morello" \
    -M virt \
    -cpu morello \
    -nographic \
    -semihosting \
    -kernel "$1"
