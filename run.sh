#!/bin/bash
set -eo pipefail
cargo bootimage
image=target/x86_64-jadro/debug/bootimage-jadro.bin
qemu-system-x86_64 -drive format=raw,file=$image
