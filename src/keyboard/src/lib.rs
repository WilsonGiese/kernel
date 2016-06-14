#![no_std]

#[macro_use]
extern crate vga;

static KBDUS: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?";

pub enum Key {
	Escape = 0x00,
	One = 0x01,
	Two = 0x03,
	Three = 0x4,
	Four = 0x05,
	Five = 0x06,
	Six = 0x07,
	Seven = 0x08,
	Eight = 0x09,
	Nine = 0x0A,
 	Zero = 0x0B,
	Dash = 0x0C,
	Equals = 0x0D,
	Backspace = 0x0E,
	Tab = 0x0F,
	Q = 0x10,
	W = 0x11,
	E = 0x12,
	R = 0x13,
	T = 0x14,
	Y = 0x15,
	U = 0x16,
	I = 0x17,
	O = 0x18,
	P = 0x19,
	OpenBracket = 0x1A,
	CloseBracket = 0x1B,
	Enter = 0x1C,
	LeftControl = 0x1D,
	A = 0x1E,
	S = 0x1F,
	D = 0x20,
	F = 0x21,
	G = 0x22,
	H = 0x23,
	J = 0x24,
	K = 0x25,
	L = 0x26,
	Semicolon = 0x27,
	SingleQuote = 0x28,
	BackTick = 0x29,
	LeftShift = 0x2A,
	Backslash = 0x2B,
	Z = 0x2C,
	X = 0x2D,
	C = 0x2E,
	V = 0x2F,
	B = 0x30,
	N = 0x31,
	M = 0x32,
	Comma = 0x33,
	Period = 0x34,
	ForwardSlash = 0x35,
	RightShift = 0x36,
	Asterisk = 0x37,
	LeftAlt = 0x38,
	Space = 0x39,
	CapsLock = 0x3A,
	F1 = 0x3B,
	F2 = 0x3C,
	F3 = 0x3D,
	F4 = 0x3E,
	F5 = 0x3F,
	F6 = 0x40,
	F7 = 0x41,
	F8 = 0x42,
	F9 = 0x43,
	F10 = 0x44,
	NumberLock = 0x45,
	ScrollLock = 0x46,
	KeypadSeven = 0x47,
	KeypadEight = 0x48,
	KeypadNine = 0x49,
	KeypadMinus = 0x4A,
	KeypadFour = 0x4B,
	KeypadFive = 0x4C,
	KeypadSix = 0x4D,
	KeypadPlus = 0x4E,
	KeypadOne = 0x4F,
	KeypadTwo= 0x50,
	KeypadThree = 0x51,
	KeypadZero = 0x52,
	KeypadPeriod = 0x53,
	F11 = 0x57,
	F12 = 0x58, 
}

pub enum KeyboardEvent {
	KeyPressed (Key),
	KeyReleased (Key)
}

pub struct Keyboard;

impl Keyboard {
	pub fn handle_keys(&self, scancode: usize) {
		if scancode <= 59 {
			kprint!("{}", KBDUS[scancode] as char);
		}
	}
}
