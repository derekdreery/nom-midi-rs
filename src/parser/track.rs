use nom::IResult;

use crate::{parser::event::parse_event, types::Track};

pub fn parse_track_chunk_header(i: &[u8]) -> IResult<&[u8], &[u8]> {
    use nom::{
        bytes::streaming::{tag, take},
        number::streaming::be_u32,
    };

    let (i, _) = tag("MTrk")(i)?;
    let (i, length) = be_u32(i)?;
    take(length)(i)
}

pub fn parse_track_chunk(i: &[u8]) -> IResult<&[u8], Track> {
    let (i, mut data) = parse_track_chunk_header(i)?;
    let mut events = vec![];
    while data.len() > 0 {
        let (data_after, evt) = parse_event(data)?;
        data = data_after;
        events.push(evt);
    }
    Ok((i, Track { events: events }))
}
