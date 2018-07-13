//! The note enum and associated helper methods

use std::mem;

/// A note representable in a 7 bit unsigned int. The subscript 's' to a note means sharp. The
/// subscript 'n' to an octave means negate, so `Cs2n` = C# in octave -2.
///
/// Because it only uses the least significant 7 bits, any value can be interpreted as either an i8
/// or a u8 for free (as the representation is the same in both)
///
/// This implements both From<u8>, Into<u8>, From<i8> and Into<i8> so the names can be completely
/// ignored if prefered
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Note {
    C1n = 0x00,
    Cs1n = 0x01,
    D1n = 0x02,
    Ds1n = 0x03,
    E1n = 0x04,
    F1n = 0x05,
    Fs1n = 0x06,
    G1n = 0x07,
    Gs1n = 0x08,
    A1n = 0x09,
    As1n = 0x0A,
    B1n = 0x0B,
    C0 = 0x0C,
    Cs0 = 0x0D,
    D0 = 0x0E,
    Ds0 = 0x0F,
    E0 = 0x10,
    F0 = 0x11,
    Fs0 = 0x12,
    G0 = 0x13,
    Gs0 = 0x14,
    /// Start of 88-note piano keyboard range
    A0 = 0x15,
    As0 = 0x16,
    B0 = 0x17,
    C1 = 0x18,
    Cs1 = 0x19,
    D1 = 0x1A,
    Ds1 = 0x1B,
    E1 = 0x1C,
    F1 = 0x1D,
    Fs1 = 0x1E,
    G1 = 0x1F,
    Gs1 = 0x20,
    A1 = 0x21,
    As1 = 0x22,
    B1 = 0x23,
    /// Start of 5 octave synth range
    C2 = 0x24,
    Cs2 = 0x25,
    D2 = 0x26,
    Ds2 = 0x27,
    E2 = 0x28,
    F2 = 0x29,
    Fs2 = 0x2A,
    G2 = 0x2B,
    Gs2 = 0x2C,
    A2 = 0x2D,
    As2 = 0x2E,
    B2 = 0x2F,
    C3 = 0x30,
    Cs3 = 0x31,
    D3 = 0x32,
    Ds3 = 0x33,
    E3 = 0x34,
    F3 = 0x35,
    Fs3 = 0x36,
    G3 = 0x37,
    Gs3 = 0x38,
    A3 = 0x39,
    As3 = 0x3A,
    B3 = 0x3B,
    /// Middle C
    C4 = 0x3C,
    Cs4 = 0x3D,
    D4 = 0x3E,
    Ds4 = 0x3F,
    E4 = 0x40,
    F4 = 0x41,
    Fs4 = 0x42,
    G4 = 0x43,
    Gs4 = 0x44,
    A4 = 0x45,
    As4 = 0x46,
    B4 = 0x47,
    C5 = 0x48,
    Cs5 = 0x49,
    D5 = 0x4A,
    Ds5 = 0x4B,
    E5 = 0x4C,
    F5 = 0x4D,
    Fs5 = 0x4E,
    G5 = 0x4F,
    Gs5 = 0x50,
    A5 = 0x51,
    As5 = 0x52,
    B5 = 0x53,
    C6 = 0x54,
    Cs6 = 0x55,
    D6 = 0x56,
    Ds6 = 0x57,
    E6 = 0x58,
    F6 = 0x59,
    Fs6 = 0x5A,
    G6 = 0x5B,
    Gs6 = 0x5C,
    A6 = 0x5D,
    As6 = 0x5E,
    B6 = 0x5F,
    /// end of 5 octave synth range
    C7 = 0x60,
    Cs7 = 0x61,
    D7 = 0x62,
    Ds7 = 0x63,
    E7 = 0x64,
    F7 = 0x65,
    Fs7 = 0x66,
    G7 = 0x67,
    Gs7 = 0x68,
    A7 = 0x69,
    As7 = 0x6A,
    B7 = 0x6B,
    /// end of 88-note piano keyboard range
    C8 = 0x6C,
    Cs8 = 0x6D,
    D8 = 0x6E,
    Ds8 = 0x6F,
    E8 = 0x70,
    F8 = 0x71,
    Fs8 = 0x72,
    G8 = 0x73,
    Gs8 = 0x74,
    A8 = 0x75,
    As8 = 0x76,
    B8 = 0x77,
    C9 = 0x78,
    Cs9 = 0x79,
    D9 = 0x7A,
    Ds9 = 0x7B,
    E9 = 0x7C,
    F9 = 0x7D,
    Fs9 = 0x7E,
    G9 = 0x7F
}

impl From<u8> for Note {
    #[inline(always)]
    fn from(n: u8) -> Note {
        // Could alternatively mask with 0x7F to remove top bit (is this better?)
        if n > 0x7F {
            panic!("Not valid note");
        }
        unsafe { mem::transmute(n) }
    }
}

impl Into<u8> for Note {
    #[inline(always)]
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<i8> for Note {
    #[inline(always)]
    fn from(n: i8) -> Note {
        // Could alternatively mask with 0x7F to remove top bit (is this better?)
        if n < 0x00 {
            panic!("Not valid note");
        }
        unsafe { mem::transmute(n) }
    }
}

impl Into<i8> for Note {
    #[inline(always)]
    fn into(self) -> i8 {
        // may be faster to use mem::transmute
        self as i8
    }
}

