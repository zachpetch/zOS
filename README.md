# zOS

I'm using this project to learn about the underlying structure of operating systems/kernels. Right now, I'm learning by building it up from scratch, with guidance from https://os.phil-opp.com

With Rust installed (see: ) you can clone this repo and run `cargo build` inside it to compile the kernal and build a disk image connected with a bootloader.
It can then be run in a QEMU VM with the following command:

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-zos/debug/bootimage-zos.bin
```
