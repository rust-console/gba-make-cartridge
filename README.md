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
