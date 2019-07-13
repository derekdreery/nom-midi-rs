//! Midi events

use crate::{
    parser::util::be_u7,
    types::{MidiEvent, MidiEventType},
};
use nom::{
    error::{make_error, ErrorKind},
    Err, IResult,
};

pub fn parse_midi_event(i: &[u8]) -> IResult<&[u8], MidiEvent> {
    use nom::number::streaming::be_u8;

    let (i, code_chan) = be_u8(i)?;
    let (i, evt_type) = match code_chan >> 4 {
        0x8 => {
            let (i, note_code) = be_u7(i)?;
            let (i, velocity) = be_u7(i)?;
            (i, MidiEventType::NoteOff(note_code.into(), velocity))
        }
        0x9 => {
            let (i, note_code) = be_u7(i)?;
            let (i, velocity) = be_u7(i)?;
            (i, MidiEventType::NoteOn(note_code.into(), velocity))
        }
        0xA => {
            let (i, note_code) = be_u7(i)?;
            let (i, pressure) = be_u7(i)?;
            (
                i,
                MidiEventType::PolyphonicPressure(note_code.into(), pressure),
            )
        }
        0xB => {
            let (i, controller) = be_u7(i)?;
            let (i, value) = be_u7(i)?;
            (i, MidiEventType::Controller(controller, value))
        }
        0xC => {
            let (i, program) = be_u7(i)?;
            (i, MidiEventType::ProgramChange(program))
        }
        0xD => {
            let (i, pressure) = be_u7(i)?;
            (i, MidiEventType::ChannelPressure(pressure))
        }
        0xE => {
            let (i, lsb) = be_u7(i)?;
            let (i, msb) = be_u7(i)?;
            (i, MidiEventType::PitchBend(lsb, msb))
        }
        _ => return Err(Err::Error(make_error(i, ErrorKind::Digit))),
    };
    Ok((
        i,
        MidiEvent {
            channel: code_chan & 0x0F,
            event: evt_type,
        },
    ))
}
