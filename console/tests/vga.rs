extern crate console;
extern crate core;

use core::fmt::Write;
use console::Vga;

#[test]
fn create() {
    let mut mock_memory = [0u16; 25 * 80];

    Vga::new(&mut mock_memory[..]);
}

fn check_write<T: Write>(_: T) { }

#[test]
fn write() {
    let mut mock_memory = vec![0u16; 25 * 80];
    let vga = Vga::new(&mut mock_memory);
    check_write(vga);
}

#[test]
fn flush() {
    let mut mock_memory = vec![0u16; 25 * 80];

    {
        let mut vga = Vga::new(&mut mock_memory);

        vga.write_str("hello").unwrap();

        vga.flush();
    }

    assert_eq!(mock_memory[0], 0x0200 | 'h' as u16);
    assert_eq!(mock_memory[1], 0x0200 | 'e' as u16);
    assert_eq!(mock_memory[2], 0x0200 | 'l' as u16);
    assert_eq!(mock_memory[3], 0x0200 | 'l' as u16);
    assert_eq!(mock_memory[4], 0x0200 | '0' as u16);
    assert_eq!(mock_memory[5], 0);
}
