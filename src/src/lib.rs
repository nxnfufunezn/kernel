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
		keyboard::gets(&mut buf[..]);

        let s = parse(&buf[..]);
        kprintln!("we got: '{}'", s);
    }
}

fn parse(buf: &[u8]) -> &str {
    // These unwraps should go away.
    let index = buf.iter().position(|&c| c == b'\n').unwrap();
    core::str::from_utf8(&buf[0..index]).unwrap()
}
