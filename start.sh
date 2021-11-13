#!/usr/bin/env bash
set -euo pipefail

cargo build
llvm-objcopy -O binary target/aarch64/debug/bare-rust target/aarch64/kernel8.img
qemu-system-aarch64 -M raspi3 -kernel target/aarch64/kernel8.img -d in_asm
