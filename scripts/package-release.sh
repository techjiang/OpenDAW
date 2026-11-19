#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-1.0.0}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/Release/opendaw-${VERSION}-source.tar.gz"

tar -C "$ROOT" \
  --exclude=.git \
  --exclude=target \
  --exclude=node_modules \
  --exclude=dist \
  -czf "$OUT" \
  Cargo.toml README.md LICENSE RELEASE.md \
  crates app docs .github scripts Release

echo "$OUT"
