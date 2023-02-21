# ktc32-emu

[![Rust](https://github.com/kinpoko/ktc32-emu/actions/workflows/rust.yml/badge.svg)](https://github.com/kinpoko/ktc32-emu/actions/workflows/rust.yml)
![License](https://img.shields.io/github/license/kinpoko/ktc32-emu?color=blue)

ktc32-emu is an emulator written in Rust for [KTC32](https://github.com/kinpoko/ktc32), a hobby 32-bit CPU implemented in SystemVerilog.

## Build

```bash
git clone https://github.com/kinpoko/ktc32-emu.git
cargo build --release
```

## Usage

```bash
ktc32-emu -h
ktc32-emu 0.1
kinpoko
KTC32 emulator

USAGE:
    ktc32-emu <FILE_PATH>

ARGS:
    <FILE_PATH>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Demo

![demo](gif/demo.gif)
