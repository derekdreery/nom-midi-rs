pub mod meta;
pub mod midi;
pub mod sysex;

use {Event, EventType};

use self::meta::parse_meta_event;
use self::midi::parse_midi_event;
use self::sysex::{parse_escape_sequence, parse_sysex_message};
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
