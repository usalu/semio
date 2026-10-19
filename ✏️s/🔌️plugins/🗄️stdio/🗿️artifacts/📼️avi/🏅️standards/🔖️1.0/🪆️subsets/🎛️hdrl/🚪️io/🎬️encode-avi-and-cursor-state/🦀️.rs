//! 🎬️ Encoder entry and cursor state of the AVI codec: the one-shot encoder, the advance result, the phase machine and the resumable encode cursor record. See [`super`].

use super::*;

//#region 🔖️Encode
/// ✍️ Real RIFF/AVI encode — layout mirrors this ticket's own `make_avi.py` fixture generator
/// exactly (`hdrl(avih + strl(strh,strf,strl_extra*)* + hdrl_extra*)`, `movi(chunk*)`, `idx1` with
/// offsets relative to the `movi` LIST's payload start INCLUDING its own `movi` tag, per the
/// OpenDML convention that generator documents) — byte-identical for the untouched round trip on a
/// single-stream fixture with no nested auxiliaries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_avi(snapshot: &AviSnapshot) -> Vec<u8> {
    let avih = write_chunk(b"avih", &write_avih(&snapshot.main_header));
    let strls: Vec<u8> = snapshot
        .streams
        .iter()
        .flat_map(|s| {
            let strh = write_chunk(b"strh", &write_strh(&s.strh));
            let strf = write_chunk(b"strf", &write_strf(&s.strf));
            let extra: Vec<u8> = s.strl_extra.iter().flat_map(write_riff_chunk).collect();
            write_list(b"strl", &[strh, strf, extra].concat())
        })
        .collect();
    let hdrl_extra: Vec<u8> = snapshot.hdrl_extra.iter().flat_map(write_riff_chunk).collect();
    let hdrl = write_list(b"hdrl", &[avih, strls, hdrl_extra].concat());

    let movi_chunks: Vec<u8> = snapshot.streams.iter().flat_map(|s| s.chunks.iter().flat_map(|c| write_chunk(&fourcc4(&c.fourcc), &c.data))).collect();
    let movi = write_list(b"movi", &movi_chunks);

    let mut idx1_payload = Vec::new();
    if snapshot.idx1_present {
        let mut offset = 4u32;
        for stream in &snapshot.streams {
            for c in &stream.chunks {
                idx1_payload.extend_from_slice(&fourcc4(&c.fourcc));
                idx1_payload.extend_from_slice(&(if c.keyframe { 0x10u32 } else { 0 }).to_le_bytes());
                idx1_payload.extend_from_slice(&offset.to_le_bytes());
                idx1_payload.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
                offset += 8 + c.data.len() as u32 + (c.data.len() as u32 % 2);
            }
        }
    }

    let mut unknown: Vec<u8> = Vec::new();
    for u in &snapshot.unknown_chunks {
        if !u.fourcc.starts_with("movi:") {
            unknown.extend(write_riff_chunk(u));
        }
    }

    let mut riff_body = Vec::new();
    riff_body.extend_from_slice(b"AVI ");
    riff_body.extend(hdrl);
    riff_body.extend(movi);
    if snapshot.idx1_present {
        riff_body.extend(write_chunk(b"idx1", &idx1_payload));
    }
    riff_body.extend(unknown);

    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(riff_body.len() as u32).to_le_bytes());
    out.extend(riff_body);
    out
}

/// 🧵️ One bounded advance of the native AVI serializer.
#[derive(Debug, PartialEq, Eq)]
pub enum AviEncodeAdvance {
    Progress,
    Chunk(Vec<u8>),
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub(super) enum AviEncodePhase {
    MeasureStreams,
    MeasureStreamExtras,
    MeasureHdrlExtras,
    MeasureMovi,
    MeasureUnknown,
    RiffHeader,
    HdrlHeader,
    AvihHeader,
    AvihData,
    MeasureEmittedStream,
    MeasureEmittedStreamExtras,
    StreamListHeader,
    StreamHeaderHeader,
    StreamHeaderData,
    StreamHeaderExtra,
    StreamHeaderPad,
    StreamFormatHeader,
    StreamFormatData,
    StreamFormatExtra,
    StreamFormatPad,
    StreamFormatDone,
    StreamExtraHeader,
    StreamExtraData,
    StreamExtraPad,
    StreamExtraDone,
    HdrlExtraHeader,
    HdrlExtraData,
    HdrlExtraPad,
    HdrlExtraDone,
    MoviHeader,
    MoviSeek,
    MoviChunkHeader,
    MoviChunkData,
    MoviChunkPad,
    MoviChunkDone,
    Idx1Header,
    Idx1Seek,
    Idx1Entry,
    Idx1EntryDone,
    UnknownSeek,
    UnknownHeader,
    UnknownData,
    UnknownPad,
    UnknownDone,
    Complete,
}

/// 🎚️ Incrementally serializes retained AVI structure without materializing the encoded file.
#[derive(semio_framework_value::RetireOwned)]
pub struct AviEncodeCursor {
    pub(super) phase: AviEncodePhase,
    pub(super) stream_index: usize,
    pub(super) item_index: usize,
    pub(super) offset: usize,
    pub(super) hdrl_children_bytes: usize,
    pub(super) movi_children_bytes: usize,
    pub(super) unknown_bytes: usize,
    pub(super) total_chunks: usize,
    pub(super) current_stream_children_bytes: usize,
    pub(super) idx1_offset: u32,
    pub(super) emitted_bytes: u64,
}
//#endregion 🔖️Encode
