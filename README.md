# zOS

## OS is overly generous

I'm learning about the underlying structure of operating systems/kernels. Right now, I'm learning by building it up from scratch, with guidance from https://os.phil-opp.com

At the moment (Tuesday, February 27, 2024), `cargo build` compiles the kernal and builds a disk image connected with a bootloadre.
It can be run in a QEMU VM with the following command:

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-zos/debug/bootimage-zos.bin
```
