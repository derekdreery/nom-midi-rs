pub mod meta;
pub mod sysex;
pub mod midi;

use {Event, EventType};

use self::midi::parse_midi_event;
use self::sysex::{parse_sysex_message, parse_escape_sequence};
use self::meta::parse_meta_event;
use super::util::parse_var_length;

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
