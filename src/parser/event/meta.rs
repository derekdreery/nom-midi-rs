//! Meta events
use {MetaEvent, KeySignature, Fps};
use super::super::util::parse_var_length_bytes;
use nom::*;

fn from_utf8_owned(s: &[u8]) -> String {
     String::from_utf8_lossy(s).into_owned()
}

/// Takes the data from the parser and turns it into a key signature
///
/// Returns none if conversion not valid
fn parse_to_key(sharps: i8, minor: u8) -> Option<KeySignature> {
    use KeySignature::*;

    let minor = match minor {
        0 => false,
        1 => true,
        _ => { return None }
    };
    match sharps {
        0 => if minor { Some(AMinor) } else { Some(CMajor) },
        1 => if minor { Some(EMinor) } else { Some(GMajor) },
        2 => if minor { Some(BMinor) } else { Some(DMajor) },
        3 => if minor { Some(FSharpMinor) } else { Some(AMajor) },
        4 => if minor { Some(CSharpMinor) } else { Some(EMajor) },
        5 => if minor { Some(GSharpMinor) } else { Some(BMajor) },
        6 => if minor { Some(DSharpMinor) } else { Some(FSharpMajor) },
        7 => if minor { Some(ASharpMinor) } else { Some(CSharpMajor) },
        -1 => if minor { Some(DMinor) } else { Some(FMajor) },
        -2 => if minor { Some(GMinor) } else { Some(BFlatMajor) },
        -3 => if minor { Some(CMinor) } else { Some(EFlatMajor) },
        -4 => if minor { Some(FMinor) } else { Some(AFlatMajor) },
        -5 => if minor { Some(BFlatMinor) } else { Some(DFlatMajor) },
        -6 => if minor { Some(EFlatMinor) } else { Some(GFlatMajor) },
        -7 => if minor { Some(AFlatMinor) } else { Some(CFlatMajor) },
        _ => { return None }
    }
}

pub fn parse_meta_event(i: &[u8]) -> IResult<&[u8], MetaEvent> {
    use super::super::super::MetaEvent::*;
    use super::super::super::SMPTEOffset as SMPTEOffsetStruct;
    use super::super::super::TimeSignature as TimeSignatureStruct;

    let (i, _) = try_parse!(i, tag!([0xFF]));
    let (i, code) = try_parse!(i, be_u8);
    let (i, data) = try_parse!(i, parse_var_length_bytes);
    let evt = match code {
        0x00 => {
            let (_, sq_num) = try_parse!(data, be_u16);
            SequenceNumber(sq_num)
        }
        0x01 => Text(from_utf8_owned(data)),
        0x02 => Copyright(from_utf8_owned(data)),
        0x03 => SequenceOrTrackName(from_utf8_owned(data)),
        0x04 => InstrumentName(from_utf8_owned(data)),
        0x05 => Lyric(from_utf8_owned(data)),
        0x06 => Marker(from_utf8_owned(data)),
        0x07 => CuePoint(from_utf8_owned(data)),
        0x08 => ProgramName(from_utf8_owned(data)),
        0x09 => DeviceName(from_utf8_owned(data)),
        0x20 => MidiChannelPrefix(try_parse!(data, be_u8).1),
        0x21 => MidiPort(try_parse!(data, be_u8).1),
        0x2F => EndOfTrack,
        0x51 => {
            let (_, data) = try_parse!(data, complete!(take!(3)));
            // 24-bit big-endian unsigned int
            Tempo((data[0] as u32) << 16 | (data[1] as u32) << 8 | (data[2] as u32))
        },
        0x54 => {
            let (_, data) = try_parse!(data, complete!(take!(5)));
            // Check top 2 bits
            let fps = match data[0] & 0xC0 {
                0x00 => Fps::TwentyFour,
                0x40 => Fps::TwentyFive,
                0x80 => Fps::TwentyNine,
                0xC0 => Fps::Thirty,
                _ => { return Err(::nom::Err::Error(error_position!(i, ErrorKind::Custom(0)))) }
            };
            SMPTEOffset(SMPTEOffsetStruct {
                fps: fps,
                hour: data[0] & 0x3F, // complement of 0xC0
                minute: data[1],
                second: data[2],
                no_frames: data[3],
                no_fractional_frames: data[4]
            })
        },
        0x58 => {
            let (_, data) = try_parse!(data, complete!(take!(4)));
            TimeSignature(TimeSignatureStruct {
                top: data[0],
                bottom: data[1],
                ticks_per_metronome_click: data[2],
                number_32nd_in_quarter: data[3]
            })
        },
        0x59 => {
            let (data, sharps) = try_parse!(data, be_i8);
            let (_, major) = try_parse!(data, be_u8);
            match parse_to_key(sharps, major) {
                Some(a) => KeySignature(a),
                None => { return Err(::nom::Err::Error(error_position!(i, ErrorKind::Custom(0)))) }
            }
        },
        0x7F => SequencerSpecificEvent(From::from(data)),
        other => Unknown(other, From::from(data))
    };
    Ok((i, evt))
}
