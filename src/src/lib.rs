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
    interrupts::install();
    interrupts::enable();

    vga::clear_console();
    kprintln!("Kernel initialized.");

    shell();
}

/// someday, this will be a real shell
fn shell() -> ! {
    loop {
        kprint!("> ");

		let mut buf = [0; 32];
		let result = keyboard::gets(&mut buf[..]);

        let s = core::str::from_utf8(result).unwrap();
        kprintln!("we got: '{}'", s);
    }
}
