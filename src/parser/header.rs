use super::super::{MidiHeader, MidiFormat, Division, Fps};
use nom::*;

named!(parse_format<MidiFormat>,
    alt!(
        do_parse!(
            with_restriction!(be_u16, |v| v == 0) >>
            with_restriction!(be_u16, |c| c == 1) >>
            (MidiFormat::SingleTrack)
        )
        | do_parse!(
            with_restriction!(be_u16, |v| v == 1) >>
            no_tracks: be_u16 >>
            (MidiFormat::MultipleTrack(no_tracks))
        )
        | do_parse!(
            with_restriction!(be_u16, |v| v == 2) >>
            no_tracks: be_u16 >>
            (MidiFormat::MultipleSong(no_tracks))
        )
    )
);

fn parse_division(i: &[u8]) -> IResult<&[u8], Division> {
    let (_, bytes) = try_parse!(i, take!(2));

    // Test first bit for type
    let division = if bytes[0] & 0x80 == 0x80 {
        // we are using timecode (2's complement notation negative numbers)
        let fps = match bytes[1] {
            0xE8 => Fps::TwentyFour,
            0xE7 => Fps::TwentyFive,
            0xE3 => Fps::TwentyNine,
            0xE2 => Fps::Thirty,
            _ => return Err(::nom::Err::Error(error_position!(i, ErrorKind::Custom(0))))
        };
        let res = bytes[0] & 0x7F;
        Division::Timecode { fps, res }
    } else {
        // we are using metrical timing
        let (_, mut note_div) = try_parse!(bytes, be_u16);
        Division::Metrical(note_div & 0x7FFF)
    };
    Ok((&i[2..], division))
}

named!(pub parse_header_chunk<&[u8], MidiHeader>,
    do_parse!(
        tag!(b"MThd") >>
        with_restriction!(be_u32, |v| v == 6) >> // length (always 6)
        format: parse_format >>
        division: parse_division >>
        ({
            MidiHeader {
                format: format,
                division: division
            }
        })
    )
);

#[test]
fn test_header_chunk() {
    let midi_file = [77u8, 84, 104, 100, 0, 0, 0, 6, 0, 1, 0, 5, 1, 0];
    assert_eq!(
        parse_header_chunk(&midi_file[..]),
        Ok((&b""[..], MidiHeader {
            format: MidiFormat::MultipleTrack(5),
            division: Division::Metrical(256),
        }))
    );
}

