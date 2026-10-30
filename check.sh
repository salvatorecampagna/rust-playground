#!/usr/bin/env bash
# Runs the same checks as CI, for every project: rustfmt, clippy and tests.
set -euo pipefail
shopt -s nullglob

# Exercises whose enabled tests are expected to fail, as paths from the repo
# root. They must still be formatted and pass clippy. The script fails if a
# listed exercise doesn't exist or already passes, so the list can't go stale.
UNFINISHED=(
  exercises/raindrops-plung
)

cd "$(dirname "$0")"

is_unfinished() {
  local dir
  for dir in "${UNFINISHED[@]}"; do
    [[ $dir == "$1" ]] && return 0
  done
  return 1
}

for dir in "${UNFINISHED[@]}"; do
  if [[ ! -f $dir/Cargo.toml ]]; then
    echo "error: $dir is listed in UNFINISHED but is not a project" >&2
    exit 1
  fi
done

for manifest in book/*/Cargo.toml exercises/*/Cargo.toml exercism/rust/*/Cargo.toml; do
  dir=$(dirname "$manifest")
  echo "== $dir"
  (
    cd "$dir"
    cargo fmt --check
    cargo clippy --quiet --all-targets --all-features -- -D warnings
    if is_unfinished "$dir"; then
      if cargo test --quiet --all-features >/dev/null 2>&1; then
        echo "error: $dir passes its tests; remove it from UNFINISHED in check.sh" >&2
        exit 1
      fi
      echo "unfinished: tests fail as expected"
    else
      cargo test --quiet --all-features
    fi
  )
done
