use nom::*;

use {Track};
use super::event::parse_event;

named!(parse_track_chunk_header,
    do_parse!(
        tag!(b"MTrk") >>
        length: be_u32 >>
        content: take!(length) >>
        (content)
    )
);

pub fn parse_track_chunk(i: &[u8]) -> IResult<&[u8], Track> {
    let (i, mut data) = try_parse!(i, parse_track_chunk_header);
    let mut events = vec![];
    while data.len() > 0 {
        let (data_after, evt) = try_parse!(data, parse_event);
        data = data_after;
        events.push(evt);
    }
    Ok((i, Track { events: events }))
}

