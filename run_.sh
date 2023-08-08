#!/usr/bin/env bash
bin=$1
cargo b --bin $bin
qemu-riscv64 2>&1 target/riscv64gc-unknown-none-elf/debug/$bin
