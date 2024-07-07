#!/usr/bin/env zsh
set -eu

cd "$(dirname "$0")"
cargo build

# DEBUGGER can be e.g. "gdb --args"
ROFI_PLUGIN_PATH=target/debug ${DEBUGGER:-} rofi -modi drun,power -show drun
