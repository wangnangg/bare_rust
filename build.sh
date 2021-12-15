#!/usr/bin/env bash
set -euo pipefail

cargo build --release
llvm-objcopy -O binary target/aarch64/release/bare-rust target/aarch64/kernel8.img
