#![no_std]

extern crate cpuio;

use core::fmt;
use core::fmt::Write;

mod color;
use color::Color;

mod cursor;
pub use cursor::{Cursor, CursorType};

const ROWS: usize = 25;
const COLS: usize = 80;

macro_rules! Cell {
    ($byte:expr, $color:expr) => ( $color | $byte as u16 );
}

pub struct Vga<T: AsMut<[u16]>, C: CursorType> {
    slice: T,
    buffer: [u16; ROWS * COLS],
    position: usize,
    cursor: C,
}

impl<T: AsMut<[u16]>, C: CursorType> Vga<T, C> {
    pub fn new(mut slice: T, cursor: C) -> Vga<T, C> {
        // we must have enough bytes of backing storage to make this work.
        assert_eq!(slice.as_mut().len(), ROWS * COLS);

        Vga {
            slice: slice,
            buffer: [0; ROWS * COLS],
            position: 0,
            cursor: cursor,
        }
    }

    pub fn flush(&mut self) {
        self.slice.as_mut().clone_from_slice(&self.buffer);
    }

    fn write_byte(&mut self, byte: u8) {

        if byte == '\n' as u8 {
            let current_line = self.position / (COLS);
            self.position = (current_line + 1) * COLS;
        } else {
            self.buffer[self.position] = Cell!(byte, color::colorcode(Color::Green, Color::Black));
            self.position += 1;
        }

        if self.position >= self.buffer.len() {
            self.scroll();
        }

        self.cursor.set(self.position as u16);
        self.buffer[self.position] = Cell!(0, color::colorcode(Color::Green, Color::Black));
    }

    fn scroll(&mut self) {
        for row in 1..ROWS {
            for cb in 0..COLS {
                let prev_position = ((row - 1) * COLS) + cb;
                let current_position = (row * COLS) + cb;
                self.buffer[prev_position] = self.buffer[current_position];
            }
        }

        for cb in ((ROWS - 1) * COLS)..(ROWS * COLS) {
            self.buffer[cb] = Cell!(0, color::colorcode(Color::Green, Color::Black));
        }

        self.position = (ROWS - 1) * COLS;
    }
}

impl<T: AsMut<[u16]>, C: CursorType> Write for Vga<T, C> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            self.write_byte(b);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use Vga;
    use cursor::CursorType;
    use core::fmt::Write;

    use ROWS;
    use COLS;

    struct MockCursor;

    impl CursorType for MockCursor {
        fn new() -> MockCursor { MockCursor { } }
        fn set(&mut self, _: u16) { }
    }

    #[test]
    fn write_a_letter() {
        let mut mock_memory = [0u16; ROWS * COLS];
        let mock_cursor = MockCursor::new();
        let mut vga = Vga::new(&mut mock_memory[..], mock_cursor);

        vga.write_str("a").unwrap();

        assert_eq!(vga.buffer[0], 0x0200 | 'a' as u16);
    }

    #[test]
    fn write_a_word() {
        let mut mock_memory = [0u16; ROWS * COLS];
        let mock_cursor = MockCursor::new();
        let mut vga = Vga::new(&mut mock_memory[..], mock_cursor);

        let word = "word";
        vga.write_str(word).unwrap();

        assert_eq!(vga.buffer[0], 0x0200 | 'w' as u16);
        assert_eq!(vga.buffer[1], 0x0200 | 'o' as u16);
        assert_eq!(vga.buffer[2], 0x0200 | 'r' as u16);
        assert_eq!(vga.buffer[3], 0x0200 | 'd' as u16);
    }

    #[test]
    fn write_multiple_words() {
        let mut mock_memory = [0u16; ROWS * COLS];
        let mock_cursor = MockCursor::new();
        let mut vga = Vga::new(&mut mock_memory[..], mock_cursor);

        vga.write_str("hello ").unwrap();
        vga.write_str("world").unwrap();

        assert_eq!(vga.buffer[0], 0x0200 | 'h' as u16);
        assert_eq!(vga.buffer[1], 0x0200 | 'e' as u16);
        assert_eq!(vga.buffer[2], 0x0200 | 'l' as u16);
        assert_eq!(vga.buffer[3], 0x0200 | 'l' as u16);
        assert_eq!(vga.buffer[4], 0x0200 | 'o' as u16);
        assert_eq!(vga.buffer[5], 0x0200 | ' ' as u16);
        assert_eq!(vga.buffer[6], 0x0200 | 'w' as u16);
        assert_eq!(vga.buffer[7], 0x0200 | 'o' as u16);
        assert_eq!(vga.buffer[8], 0x0200 | 'r' as u16);
        assert_eq!(vga.buffer[9], 0x0200 | 'l' as u16);
        assert_eq!(vga.buffer[10], 0x0200 | 'd' as u16);
    }

    #[test]
    fn write_newline() {
        let mut mock_memory = [0u16; ROWS * COLS];
        let mock_cursor = MockCursor::new();
        let mut vga = Vga::new(&mut mock_memory[..], mock_cursor);

        vga.write_str("hello\nworld\n!").unwrap();

        assert_eq!(vga.buffer[0], 0x0200 | 'h' as u16);
        assert_eq!(vga.buffer[1], 0x0200 | 'e' as u16);
        assert_eq!(vga.buffer[2], 0x0200 | 'l' as u16);
        assert_eq!(vga.buffer[3], 0x0200 | 'l' as u16);
        assert_eq!(vga.buffer[4], 0x0200 | 'o' as u16);
        assert_eq!(vga.buffer[80], 0x0200 | 'w' as u16);
        assert_eq!(vga.buffer[81], 0x0200 | 'o' as u16);
        assert_eq!(vga.buffer[82], 0x0200 | 'r' as u16);
        assert_eq!(vga.buffer[83], 0x0200 | 'l' as u16);
        assert_eq!(vga.buffer[84], 0x0200 | 'd' as u16);
        assert_eq!(vga.buffer[160], 0x0200 | '!' as u16);
    }

    #[test]
    fn write_scroll() {
        let mut mock_memory = [0u16; ROWS * COLS];
        let mock_cursor = MockCursor::new();
        let mut vga = Vga::new(&mut mock_memory[..], mock_cursor);

        for b in "abcdefghijklmnopqrstuvwxyz".bytes() {
            vga.write_byte(b);
            vga.write_byte('\n' as u8);
        }

        assert_eq!(vga.buffer[0], 0x0200 | 'c' as u16);
        for cb in ((ROWS - 1) * COLS)..(ROWS * COLS) {
            assert_eq!(vga.buffer[cb], 0x0200);
        }
    }
}
