
//! Dealing with the VGA cursor.
//!
//! The VGA cursor is handled through a system of 'indexed registers.' This means that you take a
//! location, `CRT_PORT` in this case, and write the index you'd like to access to it. You can then
//! access `CRT_PORT + 1` to read/write to that index. Within these functions, we've called these
//! two ports `crt_index` and `crt_io`, respectively. We create these ports within each function,
//! rather than trying to safe some kind of static value, because statics aren't safe, and it's not
//! like the `Port`s are expensive to create in the first place.
//!
//! This module contains the type Cursor, which has two methods, one which creates the cursor in
//! the first place, and the second which updates it to a particular location.
//!
//! References:
//!
//! * [OSDev wiki](http://wiki.osdev.org/Text_Mode_Cursor)
//! * [Wikipedia](http://wiki.osdev.org/VGA_Hardware)

use cpuio::Port;

const CRT_PORT: u16 = 0x3D4;

pub struct Cursor {
    crt_index: Port<u8>,
    crt_io: Port<u8>,
}

impl Cursor {
    pub fn new() -> Cursor {

        let mut crt_index = unsafe { Port::new(CRT_PORT) };
        let mut crt_io = unsafe { Port::new(CRT_PORT + 1) };

        crt_index.write(0b1010);
        crt_io.write(0b0000);

        crt_index.write(0b1011);
        crt_io.write(0b1111);

        Cursor {
            crt_index: crt_index,
            crt_io: crt_io,
        }
    }

    /// Sets the cursor to a given position.
    ///
    /// Instead of an X/Y coordinate, this position is a 0-(CONSOLE_ROWS * CONSOLE_COLS) position.
    pub fn set(&mut self, position: u16) {
        self.crt_index.write(0b1111);
        self.crt_io.write(position as u8);

        self.crt_index.write(0b1110);
        self.crt_io.write((position >> 8) as u8);
    }
}
