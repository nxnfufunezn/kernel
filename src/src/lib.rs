#![feature(lang_items)]
#![feature(unwind_attributes)]
#![feature(asm)]
#![no_std]

extern crate rlibc;

#[macro_use]
extern crate vga;

extern crate interrupts;
extern crate pic;

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

    loop { }
}
