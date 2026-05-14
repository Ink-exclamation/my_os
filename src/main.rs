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

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");

	my_os::init();

	x86_64::instructions::interrupts::int3();

	#[cfg(test)]
	test_main();

	println!("It did not crash");
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