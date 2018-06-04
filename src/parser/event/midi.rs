//! Midi events

use super::super::util::be_u7;
use MidiEvent;
use nom::*;

pub fn parse_midi_event(i: &[u8]) -> IResult<&[u8], MidiEvent> {
    use MidiEventType::*;

    let (i, code_chan) = try_parse!(i, be_u8);
    let (i, evt_type) = match code_chan >> 4 {
        0x8 => {
            let (i, note_code) = try_parse!(i, be_u7);
            let (i, velocity) = try_parse!(i, be_u7);
            (i, NoteOff(From::from(note_code), velocity))
        },
        0x9 => {
            let (i, note_code) = try_parse!(i, be_u7);
            let (i, velocity) = try_parse!(i, be_u7);
            (i, NoteOn(From::from(note_code), velocity))
        },
        0xA => {
            let (i, note_code) = try_parse!(i, be_u7);
            let (i, pressure) = try_parse!(i, be_u7);
            (i, PolyphonicPressure(From::from(note_code), pressure))
        },
        0xB => {
            let (i, controller) = try_parse!(i, be_u7);
            let (i, value) = try_parse!(i, be_u7);
            (i, Controller(controller, value))
        },
        0xC => {
            let (i, program) = try_parse!(i, be_u7);
            (i, ProgramChange(program))
        },
        0xD => {
            let (i, pressure) = try_parse!(i, be_u7);
            (i, ChannelPressure(pressure))
        },
        0xE => {
            let (i, lsb) = try_parse!(i, be_u7);
            let (i, msb) = try_parse!(i, be_u7);
            (i, PitchBend(lsb, msb))
        },
        _ => { return Err(::nom::Err::Error(error_position!(i, ErrorKind::Custom(0)))) }
    };
    Ok((
        i,
        MidiEvent {
            channel: code_chan & 0x0F,
            event: evt_type
        }
    ))
}
