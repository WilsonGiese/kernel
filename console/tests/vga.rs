extern crate console;
extern crate core;

use core::fmt::Write;
use console::Vga;
use console::CursorType;

struct MockCursor;

impl CursorType for MockCursor {
    fn new() -> MockCursor { MockCursor { } }
    fn set(&mut self, _: u16) { }
}

#[test]
fn create() {
    let mut mock_memory = [0u16; 25 * 80];
    let mock_cursor = MockCursor::new();

    Vga::new(&mut mock_memory[..], mock_cursor);
}

fn check_write<T: Write>(_: T) { }

#[test]
fn write() {
    let mut mock_memory = vec![0u16; 25 * 80];
    let mock_cursor = MockCursor::new();

    let vga = Vga::new(&mut mock_memory, mock_cursor);
    check_write(vga);
}

#[test]
fn flush() {
    let mut mock_memory = vec![0u16; 25 * 80];
    let mock_cursor = MockCursor::new();

    {
        let mut vga = Vga::new(&mut mock_memory, mock_cursor);

        vga.write_str("hello").unwrap();

        vga.flush();
    }

    assert_eq!(mock_memory[0], 0x0200 | 'h' as u16);
    assert_eq!(mock_memory[1], 0x0200 | 'e' as u16);
    assert_eq!(mock_memory[2], 0x0200 | 'l' as u16);
    assert_eq!(mock_memory[3], 0x0200 | 'l' as u16);
    assert_eq!(mock_memory[4], 0x0200 | 'o' as u16);
}
