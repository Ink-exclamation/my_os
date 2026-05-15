#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

use core::{panic::PanicInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Success = 0x10,
	Failed = 0x11,
}

pub trait Testable {
	fn run(&self) -> ();
}

impl<T> Testable for T
where T: Fn(),
{
	fn run(&self){
		serial_print!("{:.<60}\t", core::any::type_name::<T>());
		self();
		serial_println!("[ok]");
	}
}

pub fn exit_qemu(exit_code: QemuExitCode) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}

pub fn test_runner(tests: &[&dyn Testable]) {
	serial_println!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}

	exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
	serial_println!("[failed]\n");
	serial_println!("Error: {}\n", info);
	exit_qemu(QemuExitCode::Failed);
	loop{}
}

pub fn init() {
	gdt::init();
	interrupts::init_idt();
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	init();
	test_main();
	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	test_panic_handler(info)
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

