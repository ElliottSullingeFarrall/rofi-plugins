#!/usr/bin/env bash
set -eu

cd "$(dirname "$0")"
cargo build

# DEBUGGER can be e.g. "gdb --args"
ROFI_PLUGIN_PATH=target/debug ${DEBUGGER:-} rofi -modi logout -show logout "$@"
