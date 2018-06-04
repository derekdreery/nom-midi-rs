use nom::{IResult, ErrorKind, Needed};
use nom::Err;
use nom::simple_errors::Context;

macro_rules! with_restriction (
    // Internal parser, do not use directly
    (__impl $i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
        {
            match $submac!($i, $($args)*) {
                Err(::nom::Err::Error(e))
                    => Err(::nom::Err::Error(e)),
                Err(::nom::Err::Incomplete(::nom::Needed::Unknown))
                    => Err(::nom::Err::Incomplete(::nom::Needed::Unknown)),
                Err(::nom::Err::Incomplete(::nom::Needed::Size(i)))
                    => Err(::nom::Err::Incomplete(::nom::Needed::Size(i))),
                Err(::nom::Err::Failure(e))
                    => Err(::nom::Err::Failure(e)),
                Ok((i, o))
                    => if $submac2!(o, $($args2)*) {
                        Ok((i, o))
                    } else {
                        Err(::nom::Err::Error(error_position!($i, ::nom::ErrorKind::MapRes)))
                    }
            }
        }
    );
    ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
        with_restriction!(__impl $i, $submac!($($args)*), call!($g));
    );
    ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
        with_restriction!(__impl $i, $submac!($($args)*), $submac2!($($args2)*));
    );
    ($i:expr, $f:expr, $g:expr) => (
        with_restriction!(__impl $i, call!($f), call!($g));
    );
    ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
        with_restriction!(__impl $i, call!($f), $submac!($($args)*));
    );
);

/// Similar to `be_u8` from `nom`, but checks the most significant bit is 0
pub fn be_u7(i: &[u8]) -> IResult<&[u8], u8> {
    if i.len() < 1 {
        Result::Err(Err::Incomplete(Needed::Size(1)))
    } else {
        let val = i[0];
        if val > 127 {
            Result::Err(Err::Error(Context::Code(i, ErrorKind::Custom(0))))
        } else {
            Ok((&i[1..], val))
        }
    }
}

/// This is a complex variable length format.
///
/// Each byte is processed in turn, the lowest 7 bits are part of the number and
/// the top bit tells us if there are more bits to come.
///
pub fn parse_var_length(i: &[u8]) -> IResult<&[u8], u32> {
    let mut pos = 0;
    let mut value = 0u32;

    if i.len() == 0 {
        return Result::Err(Err::Incomplete(Needed::Unknown));
    }

    while i[pos] & 0x80 > 0 { // True if the highest bit is set
        // shift existing bits and add any new bits (masking highest bit)
        value = (value << 7) | (i[pos] as u32) & 0x7F;
        pos = pos + 1;

        // If we can't fit the number in a u32, emit an error
        if pos >= 4 {
            return Result::Err(Err::Error(error_position!(i, ErrorKind::Custom(0))));
        }
        // check we have enough bytes to continue
        if i.len() <= pos {
            return Result::Err(Err::Incomplete(Needed::Unknown));
        }
    }

    // add last bits
    value = (value << 7) | (i[pos] as u32); // No highest bit to mask on last number
    Ok((&i[pos+1..], value))
}

/// This function parses a var_length length value, followed by that many bytes
pub fn parse_var_length_bytes(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, size) = match parse_var_length(i) {
        Ok((i, size)) => (i, size),
        Result::Err(Err::Error(e)) => { return Result::Err(Err::Error(e)) },
        Result::Err(Err::Incomplete(n)) => { return Result::Err(Err::Incomplete(n)) },
        Result::Err(Err::Failure(n)) => { return Result::Err(Err::Failure(n)) },
    };
    take!(i, size)
}

#[test]
fn test_var_length() {
    let length = [0x7F];
    assert_eq!(parse_var_length(&length[..]), Ok((&b""[..], 0x7F)));
    let length = [0x81, 0x7F];
    assert_eq!(parse_var_length(&length[..]), Ok((&b""[..], 0xFF)));
    let length = [0x82, 0x80, 0x00];
    assert_eq!(parse_var_length(&length[..]), Ok((&b""[..], 0x8000)));
    let length = [0x82, 0x80, 0x80, 0x80];
    assert_eq!(parse_var_length(&length[..]),
               Result::Err(Err::Error(Context::Code(&length[..], ErrorKind::Custom(0)))));
}

#[test]
fn test_data_bytes() {
    let data = [0x04, b'c', b'h', b'a', b'r', b's'];
    assert_eq!(parse_var_length_bytes(&data[..]), Ok((&b"s"[..], &b"char"[..])));
}
