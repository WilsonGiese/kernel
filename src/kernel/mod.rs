use core;

use console::{Vga, Cursor, CursorType};

use interrupts::IdtRef;

use spin::Mutex;

#[macro_use]
mod kprint;

pub struct Context {
    pub vga: Mutex<Vga<&'static mut [u16], Cursor>>,
    pub idt: IdtRef,
}

impl Context {
    pub fn new() -> Context {
        let slice = unsafe {
            core::slice::from_raw_parts_mut(0xb8000 as *mut u16, 2000)
        };

        Context {
            vga: Mutex::new(Vga::new(slice, Cursor::new())),
            idt: IdtRef::new(),
        }
    }
}
