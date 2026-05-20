#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tmp_dir="${TMPDIR:-/tmp}/orogen-chain-node-core2-check"

tree_output="$(cd "$repo_root" && cargo tree -i core2)"
for expected in \
  "core2 v0.4.0" \
  "multihash v0.17.0" \
  "litep2p v0.13.3" \
  "sc-network v0.57.0"; do
  if ! grep -q "$expected" <<<"$tree_output"; then
    echo "missing expected dependency edge in cargo tree: $expected" >&2
    echo "$tree_output" >&2
    exit 1
  fi
done

rm -rf "$tmp_dir"
mkdir -p "$tmp_dir"
rsync -a --exclude target "$repo_root/" "$tmp_dir/"

python3 - "$tmp_dir/Cargo.toml" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
source = path.read_text()
next_source = re.sub(
    r"\n# See note at end of file: upstream `core2` is yanked\.[\s\S]*?\[patch\.crates-io\]\ncore2 = \{ path = \"vendor/core2\" \}\n",
    "\n",
    source,
    count=1,
)
if source == next_source:
    raise SystemExit("failed to remove core2 patch block from temporary manifest")
path.write_text(next_source)
PY

set +e
without_patch_output="$(cd "$tmp_dir" && cargo check --locked 2>&1)"
without_patch_status=$?
set -e

if [[ "$without_patch_status" -eq 0 ]]; then
  echo "core2 patch is no longer required; remove [patch.crates-io] and vendor/core2" >&2
  exit 1
fi

for expected in \
  'failed to select a version for the requirement `core2 = "^0.4.0"`' \
  'version 0.4.0 is yanked'; do
  if ! grep -q "$expected" <<<"$without_patch_output"; then
    echo "unexpected cargo failure when checking without core2 patch" >&2
    echo "$without_patch_output" >&2
    exit 1
  fi
done

echo "core2 workaround still required: sc-network 0.57.0 pulls litep2p 0.13.3 -> multihash 0.17.0 -> yanked core2 0.4.0"
