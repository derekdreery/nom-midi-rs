//! Meta events
use crate::{
    parser::util::parse_var_length_bytes,
    types::{Fps, KeySignature, MetaEvent, SMPTEOffset, TimeSignature},
};
use nom::{
    error::{make_error, ErrorKind},
    Err, IResult,
};

/// Takes the data from the parser and turns it into a key signature
///
/// Returns none if conversion not valid
fn parse_to_key(sharps: i8, minor: u8) -> Option<KeySignature> {
    use KeySignature::*;

    let minor = match minor {
        0 => false,
        1 => true,
        _ => return None,
    };
    match (sharps, minor) {
        (0, true) => Some(AMinor),
        (0, false) => Some(CMajor),
        (1, true) => Some(EMinor),
        (1, false) => Some(GMajor),
        (2, true) => Some(BMinor),
        (2, false) => Some(DMajor),
        (3, true) => Some(FSharpMinor),
        (3, false) => Some(AMajor),
        (4, true) => Some(CSharpMinor),
        (4, false) => Some(EMajor),
        (5, true) => Some(GSharpMinor),
        (5, false) => Some(BMajor),
        (6, true) => Some(DSharpMinor),
        (6, false) => Some(FSharpMajor),
        (7, true) => Some(ASharpMinor),
        (7, false) => Some(CSharpMajor),
        (-1, true) => Some(DMinor),
        (-1, false) => Some(FMajor),
        (-2, true) => Some(GMinor),
        (-2, false) => Some(BFlatMajor),
        (-3, true) => Some(CMinor),
        (-3, false) => Some(EFlatMajor),
        (-4, true) => Some(FMinor),
        (-4, false) => Some(AFlatMajor),
        (-5, true) => Some(BFlatMinor),
        (-5, false) => Some(DFlatMajor),
        (-6, true) => Some(EFlatMinor),
        (-6, false) => Some(GFlatMajor),
        (-7, true) => Some(AFlatMinor),
        (-7, false) => Some(CFlatMajor),
        _ => None,
    }
}

pub fn parse_meta_event(i: &[u8]) -> IResult<&[u8], MetaEvent> {
    use nom::{
        bytes::{complete::take as complete_take, streaming::tag},
        number::{
            complete::{
                be_i8 as complete_be_i8, be_u16 as complete_be_u16, be_u8 as complete_be_u8,
            },
            streaming::be_u8,
        },
    };
    let (i, _) = tag([0xFF])(i)?;
    let (i, code) = be_u8(i)?;
    let (i, data) = parse_var_length_bytes(i)?;
    let evt = match code {
        0x00 => {
            let (_, sq_num) = complete_be_u16(data)?;
            MetaEvent::SequenceNumber(sq_num)
        }
        0x01 => MetaEvent::Text(data),
        0x02 => MetaEvent::Copyright(data),
        0x03 => MetaEvent::SequenceOrTrackName(data),
        0x04 => MetaEvent::InstrumentName(data),
        0x05 => MetaEvent::Lyric(data),
        0x06 => MetaEvent::Marker(data),
        0x07 => MetaEvent::CuePoint(data),
        0x08 => MetaEvent::ProgramName(data),
        0x09 => MetaEvent::DeviceName(data),
        0x20 => {
            let (_, val) = complete_be_u8(data)?;
            MetaEvent::MidiChannelPrefix(val)
        }
        0x21 => {
            let (_, val) = complete_be_u8(data)?;
            MetaEvent::MidiPort(val)
        }
        0x2F => MetaEvent::EndOfTrack,
        0x51 => {
            let (_, data) = complete_take(3usize)(data)?;
            // 24-bit big-endian unsigned int
            MetaEvent::Tempo((data[0] as u32) << 16 | (data[1] as u32) << 8 | (data[2] as u32))
        }
        0x54 => {
            let (_, data) = complete_take(5usize)(data)?;
            // Check top 2 bits
            let fps = match data[0] & 0xC0 {
                0x00 => Fps::TwentyFour,
                0x40 => Fps::TwentyFive,
                0x80 => Fps::TwentyNine,
                0xC0 => Fps::Thirty,
                _ => return Err(Err::Error(make_error(i, ErrorKind::Digit))),
            };
            MetaEvent::SMPTEOffset(SMPTEOffset {
                fps: fps,
                hour: data[0] & 0x3F, // complement of 0xC0
                minute: data[1],
                second: data[2],
                no_frames: data[3],
                no_fractional_frames: data[4],
            })
        }
        0x58 => {
            let (_, data) = complete_take(4usize)(data)?;
            MetaEvent::TimeSignature(TimeSignature {
                top: data[0],
                bottom: data[1],
                ticks_per_metronome_click: data[2],
                number_32nd_in_quarter: data[3],
            })
        }
        0x59 => {
            let (data, sharps) = complete_be_i8(data)?;
            let (_, major) = complete_be_u8(data)?;
            match parse_to_key(sharps, major) {
                Some(a) => MetaEvent::KeySignature(a),
                None => return Err(Err::Error(make_error(i, ErrorKind::Digit))),
            }
        }
        0x7F => MetaEvent::SequencerSpecificEvent(data),
        other => MetaEvent::Unknown(other, data),
    };
    Ok((i, evt))
}
