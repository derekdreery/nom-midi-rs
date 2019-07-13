#[macro_use]
pub mod util;
pub mod event;
pub mod header;
pub mod track;

use nom::IResult;

use self::header::parse_header_chunk;
use self::track::parse_track_chunk;
use Midi;

pub fn parse_midi(i: &[u8]) -> IResult<&[u8], Midi> {
    let (mut i, header) = try_parse!(i, parse_header_chunk);
    let mut tracks = vec![];
    for _ in 0..(header.format.count()) {
        let (i_after, track) = try_parse!(i, parse_track_chunk);
        i = i_after;
        tracks.push(track);
    }
    Ok((
        i,
        Midi {
            header: header,
            tracks: tracks,
        },
    ))
}
