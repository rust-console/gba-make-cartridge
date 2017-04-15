[![Build Status](https://travis-ci.org/russellmcc/gba-make-cartridge.svg?branch=master)](https://travis-ci.org/russellmcc/gba-make-cartridge) [![Crate version](https://img.shields.io/crates/v/gba-make-cartridge.svg)](https://crates.io/crates/gba-make-cartridge)

`gba-make-cartridge` is a simple application to make a gameboy advance cartridge from an ELF file.

# requirements

Needs `armv4t-none-eabi-objcopy` on the path.  To get this on mac, try homebrew:


```sh
brew install russellmcc/armv4t-toolchain/armv4t-none-eabi-binutils
```

# how to build

```
cargo build
```

# License

MIT
