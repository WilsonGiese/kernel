#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate rlibc;

#[macro_use]
extern crate vga;

extern crate interrupts;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    interrupts::install();

    vga::clear_console();

    kprintln!("Hello from Rust world!");
    kprint!("Hello");
    kprintln!(" again!");

    let x = 5;
    let p = &x;

    kprintln!("Hello a final time: {:p}", p);


    loop { }
}
