use nom::{
    error::{make_error, ErrorKind, ParseError},
    Err, IResult, Needed,
};

/// Similar to `be_u8` from `nom`, but checks the most significant bit is 0
pub fn be_u7<'a, E>(i: &'a [u8]) -> IResult<&'a [u8], u8, E>
where
    E: ParseError<&'a [u8]>,
{
    if i.is_empty() {
        Result::Err(Err::Incomplete(Needed::Size(1)))
    } else {
        let val = i[0];
        if val > 127 {
            Err(Err::Error(make_error(i, ErrorKind::TooLarge)))
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
pub fn parse_var_length<'a, E>(i: &'a [u8]) -> IResult<&'a [u8], u32, E>
where
    E: ParseError<&'a [u8]>,
{
    let mut pos = 0;
    let mut value = 0u32;

    if i.is_empty() {
        return Err(Err::Incomplete(Needed::Unknown));
    }

    while i[pos] & 0x80 > 0 {
        // True if the highest bit is set
        // shift existing bits and add any new bits (masking highest bit)
        value = (value << 7) | (i[pos] as u32) & 0x7F;
        pos = pos + 1;

        // If we can't fit the number in a u32, emit an error
        if pos >= 4 {
            return Err(Err::Error(make_error(i, ErrorKind::TooLarge)));
        }
        // check we have enough bytes to continue
        if i.len() <= pos {
            return Err(Err::Incomplete(Needed::Unknown));
        }
    }

    // add last bits
    value = (value << 7) | (i[pos] as u32); // No highest bit to mask on last number

    Ok((&i[pos + 1..], value))
}

/// This function parses a var_length length value, followed by that many bytes
pub fn parse_var_length_bytes<'a, E>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8], E>
where
    E: ParseError<&'a [u8]>,
{
    use nom::bytes::streaming::take;
    let (i, size) = parse_var_length(i)?;
    take(size)(i)
}

#[test]
fn test_var_length() {
    let length = [0x7F];
    assert_eq!(
        parse_var_length::<(&[u8], ErrorKind)>(&length[..]),
        Ok((&b""[..], 0x7F))
    );
    let length = [0x81, 0x7F];
    assert_eq!(
        parse_var_length::<(&[u8], ErrorKind)>(&length[..]),
        Ok((&b""[..], 0xFF))
    );
    let length = [0x82, 0x80, 0x00];
    assert_eq!(
        parse_var_length::<(&[u8], ErrorKind)>(&length[..]),
        Ok((&b""[..], 0x8000))
    );
    let length = [0x82, 0x80, 0x80, 0x80];
    assert_eq!(
        parse_var_length::<(&[u8], ErrorKind)>(&length[..]),
        Err(Err::Error(make_error(&length[..], ErrorKind::TooLarge)))
    );
}

#[test]
fn test_data_bytes() {
    let data = [0x04, b'c', b'h', b'a', b'r', b's'];
    assert_eq!(
        parse_var_length_bytes::<(&[u8], ErrorKind)>(&data[..]),
        Ok((&b"s"[..], &b"char"[..]))
    );
}
