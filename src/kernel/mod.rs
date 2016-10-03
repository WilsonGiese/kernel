use core;

use console::Vga;

use interrupts::IdtRef;

use serial::Serial;

use spin::Mutex;

#[macro_use]
mod kprint;
mod kdebug;

pub struct Context {
    pub vga: Mutex<Vga<&'static mut [u8]>>,
    pub idt: IdtRef,
    pub serial: Mutex<Serial>,
}

impl Context {
    pub fn new() -> Context {
        let slice = unsafe {
            core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
        };

        Context {
            vga: Mutex::new(Vga::new(slice)),
            idt: IdtRef::new(),
            serial: Mutex::new(Serial::new()),
        }
    }
}
