mod meta;
mod midi;
mod sysex;

use crate::types::{Event, EventType};

pub use self::{
    meta::parse_meta_event,
    midi::parse_midi_event,
    sysex::{parse_escape_sequence, parse_sysex_message},
};
use super::util::parse_var_length;
use nom::IResult;

/*
named!(pub parse_event<Event>,
    do_parse!(
        delta_time: parse_var_length >>
        event: alt!(
            parse_midi_event => { |e| EventType::Midi(e) }
          | parse_sysex_message => { |e| EventType::SystemExclusive(e) }
          | parse_escape_sequence => { |e| EventType::EscapeSequence(e) }
          | parse_meta_event => { |e| EventType::Meta(e) }
        ) >>
        (Event {
            delta_time: delta_time,
            event: event
        })
    )
);
*/

pub fn parse_event(i: &[u8]) -> IResult<&[u8], Event> {
    use nom::{branch::alt, combinator::map};
    let (i, delta_time) = parse_var_length(i)?;
    let (i, event) = alt((
        map(parse_midi_event, EventType::Midi),
        map(parse_sysex_message, EventType::SystemExclusive),
        map(parse_escape_sequence, EventType::EscapeSequence),
        map(parse_meta_event, EventType::Meta),
    ))(i)?;
    Ok((
        i,
        Event {
            delta_time: delta_time,
            event: event,
        },
    ))
}
