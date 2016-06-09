#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;

#[macro_use]
extern crate vga;

extern crate interrupts;
extern crate pic;
extern crate keyboard;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    pic::remap();

    vga::clear_console();

    unsafe {
        interrupts::install();
        interrupts::enable();
    };

    kprintln!("Kernel initialized.");

    loop {
		// this will be a shell someday...
        kprint!("> ");

		let mut buf = [0; 32];
		let result = keyboard::gets(&mut buf[..]);

        let s = core::str::from_utf8(result).unwrap();
        kprintln!("we got: '{}'", s);
    }
}
