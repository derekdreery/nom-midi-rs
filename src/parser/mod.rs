mod event;
mod header;
mod track;
mod util;

pub use event::*;
pub use header::*;
pub use track::*;

use crate::types::SimpleMidiFile;
use nom::IResult;

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
