#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::{panic::PanicInfo};
#[allow(unused)]
use my_os::{print, println};
#[allow(unused)]
use my_os::vga_buffer::{WRITER, BUFFER_HEIGHT};


#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200{
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i];
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Hello World{}", "2");
    println!("Hello World{}", "3");
    println!("Hello World{}", "4");
    println!("Hello World{}", "5");
    println!("Hello World{}", "6");

    #[cfg(test)]
    test_main();

    loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info) 
}