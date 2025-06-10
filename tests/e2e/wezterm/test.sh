#!/usr/bin/env bash

#
# https://github.com/wezterm/wezterm/blob/main/ci/build-docs.sh

git clone --depth 1 https://github.com/wezterm/wezterm.git /tmp/wezterm

tracked_markdown=$(mktemp)
trap "rm -f ${tracked_markdown}" EXIT
find /tmp/wezterm/docs -type f | grep -E '\.(markdown|md)$' > "$tracked_markdown"

cargo run -- --language lua \
  --file-list "$tracked_markdown" \
  --language-config tests/e2e/wezterm/stylua.toml \
  --check
