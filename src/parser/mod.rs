pub mod event;
pub mod header;
pub mod track;
pub mod util;

use nom::IResult;

use crate::types::SimpleMidiFile;
use header::parse_header_chunk;
use track::parse_track_chunk;

pub fn parse_smf(i: &[u8]) -> IResult<&[u8], SimpleMidiFile> {
    let (mut i, header) = parse_header_chunk(i)?;
    let mut tracks = vec![];
    for _ in 0..(header.format.count()) {
        let (i_after, track) = parse_track_chunk(i)?;
        i = i_after;
        tracks.push(track);
    }
    Ok((
        i,
        SimpleMidiFile {
            header: header,
            tracks: tracks,
        },
    ))
}
