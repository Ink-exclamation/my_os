# my os(rust)
Learning to build an operating system in rust.

A minimal x86_64 kernel written in Rust. following the "Writing an OS in Rust" series.

## Current Progress
- [x] Bare-metal `no_std` environment setup.
- [x] Custom target specification for 'x86_64'
- [x] Direct VGA text buffer manipulation.
- [x] Successfully displayed "Hello World" in QEMU.

## Prerequisites
- Rust (Nightly channel)
- 'bootimage' tool: `cargo install bootimage`
- 'llvm-tools-preview': `rustup component add llvm-tools-preview`
- `QEMU`

## How to Run
First, build the kernel and create a bootable dis image:
```bash
cargo bootimage
```
Then, run it using QEMU:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-my_os.bin
```

## Technical Details
- **Architecture**: x86_64
- **Executable Format**: ELF
- **Entry Point**: `_start` (via `#[no_mangle] extern "C"`)
- **Graphics**: VGA Text Mode (Memory-mapped at `0xb8000`)

##Future Goals
- Implement a VGA Buffer module with `println!` macro support.
- Handle CPU exceptions and hardware interrups.
- Implement basic paging and memory management.