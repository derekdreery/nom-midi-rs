use crate::types::{Division, Fps, MidiFormat, MidiHeader};
use nom::{
    error::{make_error, ErrorKind},
    Err, IResult,
};

pub fn parse_format(i: &[u8]) -> IResult<&[u8], MidiFormat> {
    use nom::number::streaming::be_u16;
    let (i, format) = be_u16(i)?;
    match format {
        0 => {
            let (i, num_tracks) = be_u16(i)?;
            if num_tracks != 1 {
                Err(Err::Error(make_error(i, ErrorKind::Digit)))
            } else {
                Ok((i, MidiFormat::SingleTrack))
            }
        }
        1 => {
            let (i, num_tracks) = be_u16(i)?;
            Ok((i, MidiFormat::MultipleTrack(num_tracks)))
        }
        2 => {
            let (i, num_tracks) = be_u16(i)?;
            Ok((i, MidiFormat::MultipleSong(num_tracks)))
        }
        _ => Err(Err::Error(make_error(i, ErrorKind::Digit))),
    }
}

pub fn parse_division(i: &[u8]) -> IResult<&[u8], Division> {
    use nom::{bytes::streaming::take, number::streaming::be_u16};
    let (i, bytes) = take(2usize)(i)?;

    // Test first bit for type
    let division = if bytes[0] & 0x80 == 0x80 {
        // we are using timecode (2's complement notation negative numbers)
        let fps = match bytes[1] {
            0xE8 => Fps::TwentyFour,
            0xE7 => Fps::TwentyFive,
            0xE3 => Fps::TwentyNine,
            0xE2 => Fps::Thirty,
            _ => return Err(Err::Error(make_error(i, ErrorKind::Digit))),
        };
        let res = bytes[0] & 0x7F;
        Division::Timecode { fps, res }
    } else {
        // we are using metrical timing
        let (_, note_div) = be_u16(bytes)?;
        Division::Metrical(note_div & 0x7FFF)
    };
    Ok((i, division))
}

pub fn parse_header_chunk(i: &[u8]) -> IResult<&[u8], MidiHeader> {
    use nom::bytes::streaming::tag;
    use nom::number::streaming::be_u32;
    let (i, _) = tag("MThd")(i)?;
    let (i, hdr_len) = be_u32(i)?;
    // The header length must always be 6
    if hdr_len != 6 {
        return Err(Err::Error(make_error(i, ErrorKind::Digit)));
    }
    let (i, format) = parse_format(i)?;
    let (i, division) = parse_division(i)?;
    Ok((i, MidiHeader { format, division }))
}

#[test]
fn test_header_chunk() {
    let midi_file = [77u8, 84, 104, 100, 0, 0, 0, 6, 0, 1, 0, 5, 1, 0];
    assert_eq!(
        parse_header_chunk(&midi_file[..]),
        Ok((
            &b""[..],
            MidiHeader {
                format: MidiFormat::MultipleTrack(5),
                division: Division::Metrical(256),
            }
        ))
    );
}
