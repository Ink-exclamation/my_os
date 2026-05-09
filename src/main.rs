#![no_std]
#![no_main]


use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
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
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("HELL O AGAIN").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    
    loop{}
}
