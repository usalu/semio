//! riff-avi-codec — standalone AVI 1.0 hdrl/strl/movi/idx1 codec on top of `riff` 2.0's own
//! generic RIFF chunk reader/writer (`riff::Chunk`/`riff::ChunkContents`). Zero dependencies
//! beyond `riff` itself (see this crate's own Cargo.toml — its own `[workspace]`, isolated from
//! the repository's root workspace and Cargo.lock).
//!
//! This binary is the code the repository's own AVI 1.0/any oracle registration
//! (`🔣️oracle.json`, oracle id `riff-avi-1-0-mutate`) has always claimed exists: `riff`
//! supplies chunk id/size framing, LIST nesting and even-byte padding; everything below is this
//! module's own AVI-1.0-specific field layout against the format's public specification
//! (avih/strh/strf DWORD layouts, the movi-fourcc stream-index convention, the idx1
//! AVIIF_KEYFRAME/offset convention) — independent of, and never sharing code with, this
//! repository's own (currently broken, unrelated-peer-migration) `semio-s-plugin-stdio` crate.
//!
//! Two subcommands:
//!   build   <recipe-id> <fixture-dir> — writes <fixture-dir>/⬅️before.avi [and ➡️after.avi]
//!   project <path-to-avi>           — decodes a real AVI file and prints a typed JSON projection
//!                                     on stdout (main header, streams in order with full strh/strf,
//!                                     movi chunks as fourcc+keyframe+dataHex, unknown top-level
//!                                     chunks the same way — the caller hashes dataHex into a
//!                                     size+digest pair and drops the raw bytes, per this artifact's
//!                                     comparisonProfile).
//!
//! Every recipe's BEFORE and (where legal) AFTER document is authored directly as typed Rust
//! values below — never by executing this repository's own AviMutation dispatch/diff — then
//! handed whole to `riff::ChunkContents::write` to become real bytes. A "rejected" recipe writes
//! only `⬅️before.avi`: the attempted mutation is out-of-bounds/duplicate/missing-target by
//! construction (see each recipe's own comment), matching exactly what
//! `validate_indexed`/`AviDiff::apply` in the real diff component would refuse — this binary
//! never invokes that code, it only documents which recipes correspond to which refusal.

use riff::{Chunk, ChunkContents, ChunkId, LIST_ID, RIFF_ID};
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;

//#region 🔖️Types
#[derive(Clone)]
struct MainHeader {
    micro_sec_per_frame: u32,
    max_bytes_per_sec: u32,
    padding_granularity: u32,
    flags: u32,
    total_frames: u32,
    initial_frames: u32,
    streams: u32,
    suggested_buffer_size: u32,
    width: u32,
    height: u32,
    reserved: [u32; 4],
}

#[derive(Clone)]
struct StreamHeader {
    fcc_type: String,
    fcc_handler: String,
    flags: u32,
    priority: u16,
    language: u16,
    initial_frames: u32,
    scale: u32,
    rate: u32,
    start: u32,
    length: u32,
    suggested_buffer_size: u32,
    quality: i32,
    sample_size: u32,
    rc_frame: [i32; 4],
}

#[derive(Clone)]
enum StreamFormat {
    BitmapInfo { size: u32, width: i32, height: i32, planes: u16, bit_count: u16, compression: String, size_image: u32, x_pels_per_meter: i32, y_pels_per_meter: i32, colors_used: u32, colors_important: u32 },
    WaveFormat { format_tag: u16, channels: u16, samples_per_sec: u32, avg_bytes_per_sec: u32, block_align: u16, bits_per_sample: u16, extra: Vec<u8> },
    Raw { data: Vec<u8> },
}

#[derive(Clone)]
struct AviChunk {
    fourcc: String,
    data: Vec<u8>,
    keyframe: bool,
}

#[derive(Clone)]
struct Stream {
    strh: StreamHeader,
    strf: StreamFormat,
    chunks: Vec<AviChunk>,
}

#[derive(Clone)]
struct UnknownChunk {
    fourcc: String,
    data: Vec<u8>,
}

#[derive(Clone)]
struct AviDoc {
    main_header: MainHeader,
    streams: Vec<Stream>,
    idx1_present: bool,
    unknown_chunks: Vec<UnknownChunk>,
}
//#endregion 🔖️Types

//#region 🔖️ByteHelpers
fn fcc(s: &str) -> ChunkId {
    let mut bytes = [b' '; 4];
    for (i, b) in s.as_bytes().iter().take(4).enumerate() {
        bytes[i] = *b;
    }
    ChunkId { value: bytes }
}

fn fcc_str(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

#[cfg(test)]
fn from_hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap_or(0)).collect()
}
//#endregion 🔖️ByteHelpers

//#region 🔖️Encode — this module's own AVI-1.0 field layout, matching the format's public spec
fn write_avih(h: &MainHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(56);
    for v in [h.micro_sec_per_frame, h.max_bytes_per_sec, h.padding_granularity, h.flags, h.total_frames, h.initial_frames, h.streams, h.suggested_buffer_size, h.width, h.height] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    for v in h.reserved {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn write_strh(h: &StreamHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(&fcc(&h.fcc_type).value);
    out.extend_from_slice(&fcc(&h.fcc_handler).value);
    out.extend_from_slice(&h.flags.to_le_bytes());
    out.extend_from_slice(&h.priority.to_le_bytes());
    out.extend_from_slice(&h.language.to_le_bytes());
    out.extend_from_slice(&h.initial_frames.to_le_bytes());
    out.extend_from_slice(&h.scale.to_le_bytes());
    out.extend_from_slice(&h.rate.to_le_bytes());
    out.extend_from_slice(&h.start.to_le_bytes());
    out.extend_from_slice(&h.length.to_le_bytes());
    out.extend_from_slice(&h.suggested_buffer_size.to_le_bytes());
    out.extend_from_slice(&h.quality.to_le_bytes());
    out.extend_from_slice(&h.sample_size.to_le_bytes());
    // 📏 Modern 4×LONG rcFrame form (16 bytes) — every recipe below is hand-authored, so there is
    // no legacy-producer quirk to preserve (contrast the repository's own reader, which must also
    // accept a real encoder's classic 4×SHORT form; this binary only ever WRITES the complete form).
    for v in h.rc_frame {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn write_strf(f: &StreamFormat) -> Vec<u8> {
    match f {
        StreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => {
            let mut out = Vec::with_capacity(40);
            out.extend_from_slice(&size.to_le_bytes());
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            out.extend_from_slice(&planes.to_le_bytes());
            out.extend_from_slice(&bit_count.to_le_bytes());
            out.extend_from_slice(&fcc(compression).value);
            out.extend_from_slice(&size_image.to_le_bytes());
            out.extend_from_slice(&x_pels_per_meter.to_le_bytes());
            out.extend_from_slice(&y_pels_per_meter.to_le_bytes());
            out.extend_from_slice(&colors_used.to_le_bytes());
            out.extend_from_slice(&colors_important.to_le_bytes());
            out
        }
        StreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, extra } => {
            let mut out = Vec::with_capacity(16 + extra.len());
            out.extend_from_slice(&format_tag.to_le_bytes());
            out.extend_from_slice(&channels.to_le_bytes());
            out.extend_from_slice(&samples_per_sec.to_le_bytes());
            out.extend_from_slice(&avg_bytes_per_sec.to_le_bytes());
            out.extend_from_slice(&block_align.to_le_bytes());
            out.extend_from_slice(&bits_per_sample.to_le_bytes());
            out.extend_from_slice(extra);
            out
        }
        StreamFormat::Raw { data } => data.clone(),
    }
}

/// ✍️ The whole document, handed to `riff::ChunkContents::write` — `riff` itself computes every
/// chunk/LIST length and even-byte pad; this function only decides WHICH typed AVI-1.0 chunks
/// exist and in what order (`hdrl(avih, strl(strh,strf)*)`, `movi(chunk*)`, `idx1?`, unknown*).
fn encode_avi(doc: &AviDoc) -> Vec<u8> {
    let avih = ChunkContents::Data(fcc("avih"), write_avih(&doc.main_header));
    let strls: Vec<ChunkContents> = doc
        .streams
        .iter()
        .map(|s| {
            let strh = ChunkContents::Data(fcc("strh"), write_strh(&s.strh));
            let strf = ChunkContents::Data(fcc("strf"), write_strf(&s.strf));
            ChunkContents::Children(LIST_ID, fcc("strl"), vec![strh, strf])
        })
        .collect();
    let mut hdrl_children = vec![avih];
    hdrl_children.extend(strls);
    let hdrl = ChunkContents::Children(LIST_ID, fcc("hdrl"), hdrl_children);

    let movi_children: Vec<ChunkContents> = doc.streams.iter().flat_map(|s| s.chunks.iter().map(|c| ChunkContents::Data(fcc(&c.fourcc), c.data.clone()))).collect();
    let movi = ChunkContents::Children(LIST_ID, fcc("movi"), movi_children);

    let mut top = vec![hdrl, movi];

    if doc.idx1_present {
        // 🧭 Offsets relative to the `movi` LIST payload start INCLUDING its own "movi" tag —
        // the same OpenDML convention the repository's own reader/writer documents and this
        // binary's sibling probe decodes against.
        let mut idx1_payload = Vec::new();
        let mut offset: u32 = 4;
        for stream in &doc.streams {
            for c in &stream.chunks {
                idx1_payload.extend_from_slice(&fcc(&c.fourcc).value);
                idx1_payload.extend_from_slice(&(if c.keyframe { 0x10u32 } else { 0 }).to_le_bytes());
                idx1_payload.extend_from_slice(&offset.to_le_bytes());
                idx1_payload.extend_from_slice(&(c.data.len() as u32).to_le_bytes());
                offset += 8 + c.data.len() as u32 + (c.data.len() as u32 % 2);
            }
        }
        top.push(ChunkContents::Data(fcc("idx1"), idx1_payload));
    }

    for u in &doc.unknown_chunks {
        top.push(ChunkContents::Data(fcc(&u.fourcc), u.data.clone()));
    }

    let riff = ChunkContents::Children(RIFF_ID, fcc("AVI "), top);
    let mut cursor = Cursor::new(Vec::<u8>::new());
    riff.write(&mut cursor).expect("riff::ChunkContents::write to an in-memory Cursor cannot fail");
    cursor.into_inner()
}
//#endregion 🔖️Encode

//#region 🔖️Decode — reads real bytes back with `riff::Chunk`, this module's own field typing
fn parse_avih(payload: &[u8]) -> MainHeader {
    let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    MainHeader {
        micro_sec_per_frame: u32le(0),
        max_bytes_per_sec: u32le(4),
        padding_granularity: u32le(8),
        flags: u32le(12),
        total_frames: u32le(16),
        initial_frames: u32le(20),
        streams: u32le(24),
        suggested_buffer_size: u32le(28),
        width: u32le(32),
        height: u32le(36),
        reserved: [u32le(40), u32le(44), u32le(48), u32le(52)],
    }
}

fn parse_strh(payload: &[u8]) -> StreamHeader {
    let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let i32le = |o: usize| i32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
    let rc_frame = if payload.len() >= 64 { [i32le(48), i32le(52), i32le(56), i32le(60)] } else { [0, 0, 0, 0] };
    StreamHeader {
        fcc_type: fcc_str(&payload[0..4]),
        fcc_handler: fcc_str(&payload[4..8]),
        flags: u32le(8),
        priority: u16le(12),
        language: u16le(14),
        initial_frames: u32le(16),
        scale: u32le(20),
        rate: u32le(24),
        start: u32le(28),
        length: u32le(32),
        suggested_buffer_size: u32le(36),
        quality: i32le(40),
        sample_size: u32le(44),
        rc_frame,
    }
}

fn parse_strf(fcc_type: &str, payload: &[u8]) -> StreamFormat {
    if fcc_type == "vids" && payload.len() >= 40 {
        let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let i32le = |o: usize| i32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
        return StreamFormat::BitmapInfo {
            size: u32le(0),
            width: i32le(4),
            height: i32le(8),
            planes: u16le(12),
            bit_count: u16le(14),
            compression: fcc_str(&payload[16..20]),
            size_image: u32le(20),
            x_pels_per_meter: i32le(24),
            y_pels_per_meter: i32le(28),
            colors_used: u32le(32),
            colors_important: u32le(36),
        };
    }
    if fcc_type == "auds" && payload.len() >= 16 {
        let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
        return StreamFormat::WaveFormat {
            format_tag: u16le(0),
            channels: u16le(2),
            samples_per_sec: u32le(4),
            avg_bytes_per_sec: u32le(8),
            block_align: u16le(12),
            bits_per_sample: u16le(14),
            extra: payload.get(16..).map(|s| s.to_vec()).unwrap_or_default(),
        };
    }
    StreamFormat::Raw { data: payload.to_vec() }
}

/// 📥 Decodes real bytes back into typed streams/chunks/main-header/unknown-chunks, walking the
/// tree with `riff::Chunk::read`/`.iter()`/`.read_type()`/`.read_contents()` — `riff` owns every
/// chunk/LIST boundary computation; this function only assigns AVI-1.0 meaning to what it finds.
fn decode_avi(bytes: &[u8]) -> AviDoc {
    let mut cursor = Cursor::new(bytes);
    let top = Chunk::read(&mut cursor, 0).expect("riff::Chunk::read the top-level RIFF chunk");
    assert_eq!(top.id().as_str(), "RIFF", "not a RIFF file");
    assert_eq!(top.read_type(&mut cursor).expect("read RIFF type"), fcc("AVI "), "not an AVI RIFF form");

    let mut main_header = None;
    let mut streams: Vec<Stream> = Vec::new();
    let mut movi_chunks: Vec<(String, Vec<u8>)> = Vec::new();
    let mut idx1_flags: Vec<u32> = Vec::new();
    let mut idx1_present = false;
    let mut unknown_chunks: Vec<UnknownChunk> = Vec::new();

    for child in top.iter(&mut cursor).collect::<Vec<_>>() {
        let child = child.expect("riff::Chunk iteration over the RIFF body");
        match child.id().as_str() {
            "LIST" => {
                let list_type = child.read_type(&mut cursor).expect("read LIST type");
                match list_type.as_str() {
                    "hdrl" => {
                        for hitem in child.iter(&mut cursor).collect::<Vec<_>>() {
                            let hitem = hitem.expect("riff::Chunk iteration over hdrl");
                            match hitem.id().as_str() {
                                "avih" => main_header = Some(parse_avih(&hitem.read_contents(&mut cursor).expect("read avih"))),
                                "LIST" => {
                                    let sub_type = hitem.read_type(&mut cursor).expect("read strl type");
                                    if sub_type.as_str() == "strl" {
                                        let mut strh = None;
                                        let mut strf_bytes: Option<Vec<u8>> = None;
                                        for sitem in hitem.iter(&mut cursor).collect::<Vec<_>>() {
                                            let sitem = sitem.expect("riff::Chunk iteration over strl");
                                            match sitem.id().as_str() {
                                                "strh" => strh = Some(parse_strh(&sitem.read_contents(&mut cursor).expect("read strh"))),
                                                "strf" => strf_bytes = Some(sitem.read_contents(&mut cursor).expect("read strf")),
                                                _ => {}
                                            }
                                        }
                                        let strh = strh.expect("strl missing strh");
                                        let strf = parse_strf(&strh.fcc_type, &strf_bytes.expect("strl missing strf"));
                                        streams.push(Stream { strh, strf, chunks: Vec::new() });
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    "movi" => {
                        for mitem in child.iter(&mut cursor).collect::<Vec<_>>() {
                            let mitem = mitem.expect("riff::Chunk iteration over movi");
                            let data = mitem.read_contents(&mut cursor).expect("read movi chunk contents");
                            movi_chunks.push((mitem.id().as_str().to_string(), data));
                        }
                    }
                    _ => {
                        let data = child.read_contents(&mut cursor).expect("read unknown LIST contents");
                        unknown_chunks.push(UnknownChunk { fourcc: format!("LIST:{}", list_type.as_str()), data: data[4..].to_vec() });
                    }
                }
            }
            "idx1" => {
                idx1_present = true;
                let payload = child.read_contents(&mut cursor).expect("read idx1 contents");
                let mut r = payload.as_slice();
                while r.len() >= 16 {
                    idx1_flags.push(u32::from_le_bytes(r[4..8].try_into().unwrap()));
                    r = &r[16..];
                }
            }
            other => {
                let data = child.read_contents(&mut cursor).expect("read unknown top-level chunk contents");
                unknown_chunks.push(UnknownChunk { fourcc: other.to_string(), data });
            }
        }
    }

    let matches_by_position = idx1_present && idx1_flags.len() == movi_chunks.len();
    for (i, (fourcc_val, data)) in movi_chunks.into_iter().enumerate() {
        let stream_index: usize = fourcc_val.get(0..2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let keyframe = if matches_by_position { idx1_flags[i] & 0x10 != 0 } else { true };
        if let Some(stream) = streams.get_mut(stream_index) {
            stream.chunks.push(AviChunk { fourcc: fourcc_val, data, keyframe });
        } else {
            unknown_chunks.push(UnknownChunk { fourcc: format!("movi:{fourcc_val}"), data });
        }
    }

    AviDoc { main_header: main_header.expect("hdrl missing avih"), streams, idx1_present, unknown_chunks }
}
//#endregion 🔖️Decode

//#region 🔖️Json
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main_header_json(h: &MainHeader) -> String {
    format!(
        "{{\"microSecPerFrame\":{},\"maxBytesPerSec\":{},\"paddingGranularity\":{},\"flags\":{},\"totalFrames\":{},\"initialFrames\":{},\"streams\":{},\"suggestedBufferSize\":{},\"width\":{},\"height\":{},\"reserved\":[{},{},{},{}]}}",
        h.micro_sec_per_frame, h.max_bytes_per_sec, h.padding_granularity, h.flags, h.total_frames, h.initial_frames, h.streams, h.suggested_buffer_size, h.width, h.height, h.reserved[0], h.reserved[1], h.reserved[2], h.reserved[3]
    )
}

fn strh_json(h: &StreamHeader) -> String {
    format!(
        "{{\"fccType\":{},\"fccHandler\":{},\"flags\":{},\"priority\":{},\"language\":{},\"initialFrames\":{},\"scale\":{},\"rate\":{},\"start\":{},\"length\":{},\"suggestedBufferSize\":{},\"quality\":{},\"sampleSize\":{},\"rcFrameLeft\":{},\"rcFrameTop\":{},\"rcFrameRight\":{},\"rcFrameBottom\":{}}}",
        json_str(&h.fcc_type), json_str(&h.fcc_handler), h.flags, h.priority, h.language, h.initial_frames, h.scale, h.rate, h.start, h.length, h.suggested_buffer_size, h.quality, h.sample_size, h.rc_frame[0], h.rc_frame[1], h.rc_frame[2], h.rc_frame[3]
    )
}

fn strf_json(f: &StreamFormat) -> String {
    match f {
        StreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => format!(
            "{{\"format\":\"bitmapInfo\",\"size\":{},\"width\":{},\"height\":{},\"planes\":{},\"bitCount\":{},\"compression\":{},\"sizeImage\":{},\"xPelsPerMeter\":{},\"yPelsPerMeter\":{},\"colorsUsed\":{},\"colorsImportant\":{}}}",
            size, width, height, planes, bit_count, json_str(compression), size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important
        ),
        StreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, extra } => format!(
            "{{\"format\":\"waveFormat\",\"formatTag\":{},\"channels\":{},\"samplesPerSec\":{},\"avgBytesPerSec\":{},\"blockAlign\":{},\"bitsPerSample\":{},\"extraHex\":{}}}",
            format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, json_str(&to_hex(extra))
        ),
        StreamFormat::Raw { data } => format!("{{\"format\":\"raw\",\"dataHex\":{}}}", json_str(&to_hex(data))),
    }
}

fn chunk_json(c: &AviChunk) -> String {
    format!("{{\"fourcc\":{},\"keyframe\":{},\"dataHex\":{}}}", json_str(&c.fourcc), c.keyframe, json_str(&to_hex(&c.data)))
}

fn stream_json(s: &Stream) -> String {
    let chunks: Vec<String> = s.chunks.iter().map(chunk_json).collect();
    format!("{{\"strh\":{},\"strf\":{},\"chunks\":[{}]}}", strh_json(&s.strh), strf_json(&s.strf), chunks.join(","))
}

fn unknown_json(u: &UnknownChunk) -> String {
    format!("{{\"fourcc\":{},\"dataHex\":{}}}", json_str(&u.fourcc), json_str(&to_hex(&u.data)))
}

fn doc_json(doc: &AviDoc) -> String {
    let streams: Vec<String> = doc.streams.iter().map(stream_json).collect();
    let unknowns: Vec<String> = doc.unknown_chunks.iter().map(unknown_json).collect();
    format!("{{\"mainHeader\":{},\"idx1Present\":{},\"streams\":[{}],\"unknownChunks\":[{}]}}", main_header_json(&doc.main_header), doc.idx1_present, streams.join(","), unknowns.join(","))
}
//#endregion 🔖️Json

//#region 🔖️BaseDocument
/// 🧬 The shared starting document every recipe clones from — 2 streams (vids/MJPG 3 chunks,
/// auds/PCM 2 chunks), idx1 present, 1 top-level unknown chunk (`JUNK`) — big enough to exercise
/// every one of the 13 declared mutation kinds meaningfully (multi-stream for insert/remove-stream,
/// multi-chunk for insert/remove/set-keyframe-chunk, a real unknown chunk for
/// add/remove-unknown-chunk).
fn base_doc() -> AviDoc {
    AviDoc {
        main_header: MainHeader { micro_sec_per_frame: 66_667, max_bytes_per_sec: 1_000_000, padding_granularity: 0, flags: 0x10, total_frames: 3, initial_frames: 0, streams: 2, suggested_buffer_size: 10_000, width: 64, height: 48, reserved: [0, 0, 0, 0] },
        streams: vec![
            Stream {
                strh: StreamHeader { fcc_type: "vids".into(), fcc_handler: "MJPG".into(), flags: 0, priority: 0, language: 0, initial_frames: 0, scale: 1, rate: 15, start: 0, length: 3, suggested_buffer_size: 10_000, quality: -1, sample_size: 0, rc_frame: [0, 0, 64, 48] },
                strf: StreamFormat::BitmapInfo { size: 40, width: 64, height: 48, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 9216, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
                chunks: vec![
                    AviChunk { fourcc: "00dc".into(), data: vec![0xAA, 0x00, 0x01], keyframe: true },
                    AviChunk { fourcc: "00dc".into(), data: vec![0xAA, 0x00, 0x02, 0x02], keyframe: false },
                    AviChunk { fourcc: "00dc".into(), data: vec![0xAA, 0x00, 0x03, 0x03, 0x03], keyframe: false },
                ],
            },
            Stream {
                strh: StreamHeader { fcc_type: "auds".into(), fcc_handler: "    ".into(), flags: 0, priority: 0, language: 0, initial_frames: 0, scale: 1, rate: 44_100, start: 0, length: 2, suggested_buffer_size: 4, quality: 0, sample_size: 2, rc_frame: [0, 0, 0, 0] },
                strf: StreamFormat::WaveFormat { format_tag: 1, channels: 1, samples_per_sec: 44_100, avg_bytes_per_sec: 88_200, block_align: 2, bits_per_sample: 16, extra: vec![] },
                chunks: vec![AviChunk { fourcc: "01wb".into(), data: vec![0x10, 0x11], keyframe: true }, AviChunk { fourcc: "01wb".into(), data: vec![0x12, 0x13], keyframe: true }],
            },
        ],
        idx1_present: true,
        unknown_chunks: vec![UnknownChunk { fourcc: "JUNK".into(), data: vec![0, 0, 0, 0] }],
    }
}
//#endregion 🔖️BaseDocument

//#region 🔖️Recipes
/// 🧪 One recipe: BEFORE always, AFTER only when the mutation is legal (`None` ⇒ a `-rejected-`
/// recipe — the generator writes only `⬅️before.avi`). Every AFTER state below touches EXACTLY the
/// fields the real `AviMutation::diff` match arm for that kind touches (see
/// `…/🧬️schema/🧬️mutations/🦀️.rs`) — including leaving `mainHeader.streams`/`strh.length` STALE
/// where the real dispatch would (e.g. `InsertStream`/`RemoveStream`/`InsertChunk`/`RemoveChunk`
/// never touch `mainHeader`/`strh.length`), so these fixtures assert what production actually
/// produces, not a hand-tidied idea of it.
fn recipe(id: &str) -> Option<(AviDoc, Option<AviDoc>)> {
    let base = base_doc();
    match id {
        "no-mutation-applied" => Some((base.clone(), Some(base))),

        // 🧬 SetSnapshot replaces the WHOLE document — a materially different doc throughout.
        "set-snapshot-applied" => {
            let mut after = base.clone();
            after.main_header.width = 128;
            after.main_header.height = 96;
            after.main_header.total_frames = 4;
            after.streams[0].strh.rate = 30;
            after.streams[0].strh.length = 4;
            after.streams[0].chunks.push(AviChunk { fourcc: "00dc".into(), data: vec![0xAA, 0x00, 0x04, 0x04, 0x04, 0x04], keyframe: false });
            Some((base, Some(after)))
        }

        "set-main-header-applied" => {
            let mut after = base.clone();
            after.main_header.total_frames = 5;
            after.main_header.max_bytes_per_sec = 2_000_000;
            Some((base, Some(after)))
        }

        "set-idx1-present-applied" => {
            let mut after = base.clone();
            after.idx1_present = false;
            Some((base, Some(after)))
        }

        // 🧬 InsertStream{index:2, ..} — append a third stream; mainHeader.streams stays 2 (stale).
        "insert-stream-applied" => {
            let mut after = base.clone();
            after.streams.push(Stream {
                strh: StreamHeader { fcc_type: "vids".into(), fcc_handler: "XVID".into(), flags: 0, priority: 0, language: 0, initial_frames: 0, scale: 1, rate: 15, start: 0, length: 0, suggested_buffer_size: 0, quality: -1, sample_size: 0, rc_frame: [0, 0, 64, 48] },
                strf: StreamFormat::Raw { data: vec![] },
                chunks: vec![],
            });
            Some((base, Some(after)))
        }
        // 🧬 InsertStream{index:10, ..} — 10 > final_len(3): `mutation.apply.invalid-index`, rejected.
        "insert-stream-rejected-out-of-bounds" => Some((base, None)),

        // 🧬 RemoveStream{index:1} — drops the audio stream; mainHeader.streams stays 2 (stale).
        "remove-stream-applied" => {
            let mut after = base.clone();
            after.streams.remove(1);
            Some((base, Some(after)))
        }
        // 🧬 RemoveStream{index:5} — 5 >= base.len()(2): `mutation.apply.missing-target`, rejected.
        "remove-stream-rejected-missing" => Some((base, None)),

        // 🧬 SetStreamHeader{stream_index:0, strh:<new>} — whole-value replace of stream 0's strh.
        "set-stream-header-applied" => {
            let mut after = base.clone();
            after.streams[0].strh.rate = 30;
            Some((base, Some(after)))
        }
        // 🧬 SetStreamHeader{stream_index:5, ..} — 5 >= base.streams.len()(2): missing-target, rejected.
        "set-stream-header-rejected-missing-stream" => Some((base, None)),

        // 🧬 SetStreamFormat{stream_index:1, strf:<new>} — whole-value replace of stream 1's strf.
        "set-stream-format-applied" => {
            let mut after = base.clone();
            after.streams[1].strf = StreamFormat::WaveFormat { format_tag: 1, channels: 1, samples_per_sec: 48_000, avg_bytes_per_sec: 96_000, block_align: 2, bits_per_sample: 16, extra: vec![] };
            Some((base, Some(after)))
        }
        "set-stream-format-rejected-missing-stream" => Some((base, None)),

        // 🧬 InsertChunk{stream_index:0, index:3, ..} — append a 4th chunk; strh.length stays 3 (stale).
        "insert-chunk-applied" => {
            let mut after = base.clone();
            after.streams[0].chunks.push(AviChunk { fourcc: "00dc".into(), data: vec![0xAA, 0x00, 0x09, 0x09], keyframe: false });
            Some((base, Some(after)))
        }
        // 🧬 InsertChunk{stream_index:9, ..} — 9 >= base.streams.len()(2): missing-target on the
        // STREAMS level (`chunk_diff_for` wraps a `stream_diff_for` IndexedModified at index 9).
        "insert-chunk-rejected-missing-stream" => Some((base, None)),

        // 🧬 RemoveChunk{stream_index:0, index:1} — drops the middle chunk of stream 0.
        "remove-chunk-applied" => {
            let mut after = base.clone();
            after.streams[0].chunks.remove(1);
            Some((base, Some(after)))
        }
        // 🧬 RemoveChunk{stream_index:0, index:99} — valid stream, 99 >= chunks.len()(3): missing-target
        // on the CHUNKS level (a different validation path than the missing-stream cases above).
        "remove-chunk-rejected-missing-chunk" => Some((base, None)),

        // 🧬 SetChunkKeyframe{stream_index:0, index:2, keyframe:true} — flips the last chunk's flag.
        "set-chunk-keyframe-applied" => {
            let mut after = base.clone();
            after.streams[0].chunks[2].keyframe = true;
            Some((base, Some(after)))
        }
        "set-chunk-keyframe-rejected-missing-chunk" => Some((base, None)),

        // 🧬 AddUnknownChunk{index:1, item:<XTRA>} — append after the existing JUNK.
        "add-unknown-chunk-applied" => {
            let mut after = base.clone();
            after.unknown_chunks.push(UnknownChunk { fourcc: "XTRA".into(), data: vec![7, 7, 7] });
            Some((base, Some(after)))
        }
        // 🧬 AddUnknownChunk{index:99, ..} — 99 > final_len(2): invalid-index, rejected.
        "add-unknown-chunk-rejected-out-of-bounds" => Some((base, None)),

        // 🧬 RemoveUnknownChunk{index:0} — drops the JUNK chunk, leaving none.
        "remove-unknown-chunk-applied" => {
            let mut after = base.clone();
            after.unknown_chunks.clear();
            Some((base, Some(after)))
        }
        // 🧬 RemoveUnknownChunk{index:99} — 99 >= unknown_chunks.len()(1): missing-target, rejected.
        "remove-unknown-chunk-rejected-missing" => Some((base, None)),

        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &[
    "no-mutation-applied",
    "set-snapshot-applied",
    "set-main-header-applied",
    "set-idx1-present-applied",
    "insert-stream-applied",
    "insert-stream-rejected-out-of-bounds",
    "remove-stream-applied",
    "remove-stream-rejected-missing",
    "set-stream-header-applied",
    "set-stream-header-rejected-missing-stream",
    "set-stream-format-applied",
    "set-stream-format-rejected-missing-stream",
    "insert-chunk-applied",
    "insert-chunk-rejected-missing-stream",
    "remove-chunk-applied",
    "remove-chunk-rejected-missing-chunk",
    "set-chunk-keyframe-applied",
    "set-chunk-keyframe-rejected-missing-chunk",
    "add-unknown-chunk-applied",
    "add-unknown-chunk-rejected-out-of-bounds",
    "remove-unknown-chunk-applied",
    "remove-unknown-chunk-rejected-missing",
];
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, fixture_dir: &str) -> i32 {
    let Some((before, after)) = recipe(id) else {
        eprintln!("[riff-avi-codec] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(fixture_dir);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("⬅️before.avi"), encode_avi(&before)).expect("write ⬅️before.avi");
    if let Some(after) = after {
        fs::write(dir.join("➡️after.avi"), encode_avi(&after)).expect("write ➡️after.avi");
        eprintln!("[riff-avi-codec] {id}: ⬅️before.avi + ➡️after.avi -> {}", dir.display());
    } else {
        eprintln!("[riff-avi-codec] {id}: ⬅️before.avi only (rejected) -> {}", dir.display());
    }
    0
}

fn cmd_project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[riff-avi-codec] cannot read {path}: {e}");
            return 1;
        }
    };
    let doc = decode_avi(&bytes);
    println!("{}", doc_json(&doc));
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(fixture_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: riff-avi-codec build <recipe-id> <fixture-dir>");
                std::process::exit(2);
            };
            cmd_build(id, fixture_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: riff-avi-codec project <path-to-avi>");
                std::process::exit(2);
            };
            cmd_project(path)
        }
        Some("list-recipes") => {
            for id in RECIPE_IDS {
                println!("{id}");
            }
            0
        }
        _ => {
            eprintln!("usage: riff-avi-codec build <recipe-id> <out-dir> | project <path-to-avi> | list-recipes");
            2
        }
    };
    std::process::exit(code);
}
//#endregion 🔖️Entry

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_recipe_id_resolves() {
        for id in RECIPE_IDS {
            assert!(recipe(id).is_some(), "recipe {id} must resolve");
        }
    }

    #[test]
    fn applied_recipes_have_an_after_state_rejected_recipes_do_not() {
        for id in RECIPE_IDS {
            let (_, after) = recipe(id).unwrap();
            assert_eq!(after.is_some(), id.contains("-applied"), "recipe {id} outcome must match its own id");
        }
    }

    #[test]
    fn encode_decode_round_trips_the_base_document() {
        let doc = base_doc();
        let bytes = encode_avi(&doc);
        let back = decode_avi(&bytes);
        assert_eq!(back.main_header.width, doc.main_header.width);
        assert_eq!(back.streams.len(), doc.streams.len());
        assert_eq!(back.streams[0].chunks.len(), doc.streams[0].chunks.len());
        assert_eq!(back.streams[0].chunks[0].keyframe, true);
        assert_eq!(back.streams[0].chunks[1].keyframe, false);
        assert_eq!(back.unknown_chunks.len(), 1);
        assert_eq!(back.unknown_chunks[0].fourcc, "JUNK");
    }

    #[test]
    fn no_idx1_omits_the_idx1_chunk_entirely() {
        let mut doc = base_doc();
        doc.idx1_present = false;
        let bytes = encode_avi(&doc);
        let back = decode_avi(&bytes);
        assert_eq!(back.idx1_present, false);
    }

    #[test]
    fn json_round_trips_via_hex_are_lossless() {
        let data = vec![0u8, 1, 254, 255, 16, 17];
        assert_eq!(from_hex(&to_hex(&data)), data);
    }
}
//#endregion 🔖️Tests
