#!/bin/bash
# @claim:cli-local-read-only
set -euo pipefail

root_dir=$(pwd)
consumer_dir=$(mktemp -d)
cleanup() {
  case "$consumer_dir" in
    /tmp/*) rm -rf -- "$consumer_dir" ;;
    *) echo "Refusing to remove unexpected path" >&2; exit 2 ;;
  esac
}
trap cleanup EXIT

cargo package --allow-dirty --no-verify >/dev/null
crate="$root_dir/target/package/secret-sync-preflight-0.1.0.crate"
mkdir -p "$consumer_dir/source" "$consumer_dir/install" "$consumer_dir/work"
tar -xzf "$crate" -C "$consumer_dir/source"
cargo install --path "$consumer_dir/source/secret-sync-preflight-0.1.0" --root "$consumer_dir/install" --locked >/dev/null

binary="$consumer_dir/install/bin/sspf"
test -x "$binary"
test "$(find "$consumer_dir/install/bin" -maxdepth 1 -type f -perm -100 | wc -l)" -eq 1

cc -shared -fPIC "$root_dir/scripts/network-deny.c" -o "$consumer_dir/network-deny.so"
printf '%s\n' 'EXPECTED_KEY' > "$consumer_dir/work/destination.keys"
printf '%s\n' \
  'version = 1' \
  '[[environments]]' \
  'name = "consumer"' \
  'desired = ["EXPECTED_KEY"]' \
  '[[environments.destinations]]' \
  'name = "local-export"' \
  'export = "destination.keys"' > "$consumer_dir/work/preflight.toml"

before_manifest=$(sha256sum "$consumer_dir/work/preflight.toml")
before_export=$(sha256sum "$consumer_dir/work/destination.keys")
set +e
(
  cd "$consumer_dir/work"
  SSPF_NETWORK_PROBE="$consumer_dir/network.log" \
  LD_PRELOAD="$consumer_dir/network-deny.so" \
  PROVIDER_TOKEN="unused-test-token" \
  "$binary" check --manifest preflight.toml > "$consumer_dir/output.txt" 2> "$consumer_dir/error.txt"
)
status=$?
set -e

test "$status" -eq 0
test ! -e "$consumer_dir/network.log"
test ! -s "$consumer_dir/error.txt"
grep -q '^PASS' "$consumer_dir/output.txt"
test "$before_manifest" = "$(sha256sum "$consumer_dir/work/preflight.toml")"
test "$before_export" = "$(sha256sum "$consumer_dir/work/destination.keys")"
test "$(find "$consumer_dir/work" -maxdepth 1 -type f | wc -l)" -eq 2

echo "Installed package passed without network calls or input changes"
