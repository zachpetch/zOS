# zOS

I'm using this project to learn about the underlying structure of operating systems/kernels. Right now, I'm learning by building it up from scratch, with guidance from https://os.phil-opp.com

My work has been done on an aarch64 version of macOS, a lot will need to be done differently if you're using a different system. But, for macOS on an m-series processor, this works.

## Prereqs

### Rust
see: https://www.rust-lang.org/tools/install

### QEMU
```
brew install qemu
```

### Rust Nightly
From the project directory
```
rustup override set nightly
```

### rust-src
From the project directory
```
rustup component add rust-src
```

### bootimage
**This must be run from the home directory** (or, at least, a directory above the project directory):
```
cargo install bootimage
```

### llvm-tools-preview
This must be run from back _inside_ the project directory.
```
rustup component add llvm-tools-preview
```

## Running

With the prerequisites installed and this repo cloned, you should be able to run `cargo build` inside it to compile the kernal and build a disk image connected with a bootloader.

Targets have been set up such that you can run this in Qemu using
```
cargo run
```

Alternatively, you can run it directly with the following command:
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-zos/debug/bootimage-zos.bin
```

