//! System exclusive events

use crate::parser::util::parse_var_length_bytes;
use crate::types::{EscapeSequence, SystemExclusiveEvent};
use nom::IResult;

pub fn parse_sysex_message(i: &[u8]) -> IResult<&[u8], SystemExclusiveEvent> {
    use nom::bytes::streaming::tag;
    let (i, _) = tag([0xF0])(i)?;
    let (i, data) = parse_var_length_bytes(i)?;
    Ok((i, SystemExclusiveEvent(data)))
}

pub fn parse_escape_sequence(i: &[u8]) -> IResult<&[u8], EscapeSequence> {
    use nom::bytes::streaming::tag;
    let (i, _) = tag([0xF7])(i)?;
    let (i, data) = parse_var_length_bytes(i)?;
    Ok((i, EscapeSequence(data)))
}
