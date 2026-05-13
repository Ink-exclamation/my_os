#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::{panic::PanicInfo};

mod vga_buffer;
mod serial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_menu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_menu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[unsafe(export_name = "memcpy")]
pub unsafe extern "C" fn memcpy(
    dest: *mut u8,
    src: *const u8,
    n: usize,
) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

#[unsafe(export_name = "memset")]
pub unsafe extern "C" fn memset(
    s: *mut u8,
    c: i32,
    n: usize,
) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *s.add(i) = c as u8;
        }
    }
    s
}

#[unsafe(export_name = "memcmp")]
pub unsafe extern "C" fn memcmp(
    s1: *const u8,
    s2: *const u8,
    n: usize,
) -> i32 {
    unsafe {
        for i in 0..n {
            let a = *s1.add(i);
            let b = *s2.add(i);

            if a != b {
                return a as i32 - b as i32;
            }
        }
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("HELL O AGAIN").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    
    loop{}
}
