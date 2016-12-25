//! System exclusive events

use {SystemExclusiveEvent, EscapeSequence};
use super::super::util::parse_var_length_bytes;

named!(pub parse_sysex_message<&[u8], SystemExclusiveEvent>,
    do_parse!(
        tag!([0xF0]) >>
        data: parse_var_length_bytes >>
        (SystemExclusiveEvent(From::from(data)))
    )
);

named!(pub parse_escape_sequence<&[u8], EscapeSequence>,
    do_parse!(
        tag!([0xF7]) >>
        data: parse_var_length_bytes >>
        (EscapeSequence(From::from(data)))
    )
);


