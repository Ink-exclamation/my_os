# my os(rust)
Learning to build an operating system in rust.

A minimal x86_64 kernel written in Rust. following the "Writing an OS in Rust" series.

## Current Progress
- Bare-metal `no_std` environment setup
- Custom target specification for 'x86_64'
- Direct VGA text buffer manipulation
- Successfully displayed "Hello World" in QEMU
- VGA text mode output
- Basic Writer abstraction
- Newline and screen scrolling

## Prerequisites
- Rust (Nightly channel)
- 'bootimage' tool: `cargo install bootimage`
- 'llvm-tools-preview': `rustup component add llvm-tools-preview`
- QEMU

## How to Run
```bash
cargo run
```

## Technical Details
- **Architecture**: x86_64
- **Executable Format**: ELF
- **Entry Point**: `_start` (via `#[no_mangle] extern "C"`)
- **Graphics**: VGA Text Mode (Memory-mapped at `0xb8000` memory-mapped buffer)

## Future Goals
- Implement `println!` macro support.
- Handle CPU exceptions and hardware interrupts.
- Implement basic paging and memory management.