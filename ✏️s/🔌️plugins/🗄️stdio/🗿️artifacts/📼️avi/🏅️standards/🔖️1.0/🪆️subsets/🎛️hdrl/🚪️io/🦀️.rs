//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
    use crate::standards::v1_0::subsets::any::io::AviAnalyzer;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct AviComposerComposition;

    impl ArtifactComposition for AviComposerComposition {
        type Snapshot = AviSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "AviComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = AviAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "AviComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1_0::subsets::any::schema::avi_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences();
        semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.avi", standard: semio_framework_artifact_reference::StandardId("1.0"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::bare::<AviSnapshot, crate::standards::v1_0::subsets::any::schema::mutations::AviMutation>(crate::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.avi.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1_0::subsets::any::schema::inferences::avi_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

use crate::apply_mutation;
use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk, STDIO_AVI_DOCUMENT_SCHEMA};

//#region 🔖️Riff
struct RiffEntry<'a> {
    fourcc: [u8; 4],
    payload: &'a [u8],
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn iter_riff(data: &[u8]) -> impl Iterator<Item = Result<RiffEntry<'_>, String>> {
    struct It<'a> {
        data: &'a [u8],
        pos: usize,
    }
    impl<'a> Iterator for It<'a> {
        type Item = Result<RiffEntry<'a>, String>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.pos + 8 > self.data.len() {
                return None;
            }
            let fourcc: [u8; 4] = self.data[self.pos..self.pos + 4].try_into().unwrap();
            let size = u32::from_le_bytes(self.data[self.pos + 4..self.pos + 8].try_into().unwrap()) as usize;
            let payload_start = self.pos + 8;
            let Some(payload) = self.data.get(payload_start..payload_start + size) else { return Some(Err("avi: truncated chunk".into())) };
            let padded = size + (size % 2);
            self.pos = payload_start + padded;
            Some(Ok(RiffEntry { fourcc, payload }))
        }
    }
    It { data, pos: 0 }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fourcc_str(f: &[u8; 4]) -> String {
    String::from_utf8_lossy(f).into_owned()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_chunk(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len() + 1);
    out.extend_from_slice(fourcc);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        out.push(0);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_list(list_type: &[u8; 4], children: &[u8]) -> Vec<u8> {
    let mut payload = list_type.to_vec();
    payload.extend_from_slice(children);
    write_chunk(b"LIST", &payload)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fourcc4(s: &str) -> [u8; 4] {
    let mut out = [b' '; 4];
    for (i, b) in s.as_bytes().iter().take(4).enumerate() {
        out[i] = *b;
    }
    out
}

/// ✍️ One typed-raw [`RiffChunk`] re-serialized — a `LIST` of unknown type (tagged `"LIST:<type>"`
/// on decode) writes back as a `LIST`, everything else as a plain chunk. Shared by top-level
/// `unknown_chunks`, `hdrl_extra`, and every stream's `strl_extra` — the SAME typed-raw convention
/// one level down.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_riff_chunk(item: &RiffChunk) -> Vec<u8> {
    if let Some(list_type) = item.fourcc.strip_prefix("LIST:") {
        write_list(&fourcc4(list_type), &item.data)
    } else {
        write_chunk(&fourcc4(&item.fourcc), &item.data)
    }
}
//#endregion 🔖️Riff

//#region 🔖️Sniff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"AVI "
}
//#endregion 🔖️Sniff

//#region 🔖️Header
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_avih(payload: &[u8]) -> Result<AviMainHeader, String> {
    if payload.len() < 56 {
        return Err("avi: avih shorter than 56 bytes".into());
    }
    let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    Ok(AviMainHeader {
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
        reserved: vec![u32le(40), u32le(44), u32le(48), u32le(52)],
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_avih(h: &AviMainHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(56);
    for v in [h.micro_sec_per_frame, h.max_bytes_per_sec, h.padding_granularity, h.flags, h.total_frames, h.initial_frames, h.streams, h.suggested_buffer_size, h.width, h.height] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    let mut reserved = h.reserved.clone();
    reserved.resize(4, 0);
    for v in reserved {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

/// 📥️ Parses `strh` (AVISTREAMHEADER). The 13 fixed fields up to `dwSampleSize` are always 48
/// bytes; the trailing `rcFrame` is NOT fixed-width on the wire — real encoders (ffmpeg's own
/// AVI-1.0 muxer included) still write the classic pre-Win32 form where `rcFrame` is 4 16-bit
/// `SHORT`s (56 bytes total), not only the modern 4 `LONG`s form (64 bytes) most docs describe.
/// Confirmed against the real committed fixture: its own `strh` is 56 bytes, and bytes 48..56 read
/// as 4 `SHORT`s decode to `(0, 0, 480, 432)` — the video's real frame rectangle, not garbage — so
/// this is a genuine, common, spec-legal producer behaviour, not bytes simply omitted. Anything
/// shorter than 48 bytes (missing a required fixed field) is still rejected; anything at or beyond
/// 48 is accepted, defaulting `rcFrame` to zero only when truly absent (48..56 bytes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_strh(payload: &[u8]) -> Result<AviStreamHeader, String> {
    if payload.len() < 48 {
        return Err(format!("avi: strh is {} byte(s), need at least 48", payload.len()));
    }
    let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let i32le = |o: usize| i32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
    let i16le = |o: usize| i16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
    let (rc_frame_left, rc_frame_top, rc_frame_right, rc_frame_bottom, rc_frame_width) = if payload.len() >= 64 {
        (i32le(48), i32le(52), i32le(56), i32le(60), 16u8)
    } else if payload.len() >= 56 {
        (i16le(48) as i32, i16le(50) as i32, i16le(52) as i32, i16le(54) as i32, 8u8)
    } else {
        (0, 0, 0, 0, 0u8)
    };
    let strh_extra = if payload.len() > 64 { payload[64..].to_vec() } else { Vec::new() };
    Ok(AviStreamHeader {
        fcc_type: fourcc_str(&payload[0..4].try_into().unwrap()),
        fcc_handler: fourcc_str(&payload[4..8].try_into().unwrap()),
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
        rc_frame_left,
        rc_frame_top,
        rc_frame_right,
        rc_frame_bottom,
        rc_frame_width,
        strh_extra,
    })
}

/// ✍️ Re-serializes `strh` at whichever `rcFrame` width [`parse_strh`] recorded (`h.rc_frame_width`)
/// — 0 (omitted), 8 (4 `SHORT`s, classic 56-byte form) or 16 (4 `LONG`s, modern 64-byte form) —
/// rather than always promoting to 64 bytes, so a real 56-byte source round-trips byte-for-byte.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_strh(h: &AviStreamHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(64 + h.strh_extra.len());
    out.extend_from_slice(&fourcc4(&h.fcc_type));
    out.extend_from_slice(&fourcc4(&h.fcc_handler));
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
    match h.rc_frame_width {
        16 => {
            for v in [h.rc_frame_left, h.rc_frame_top, h.rc_frame_right, h.rc_frame_bottom] {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        8 => {
            for v in [h.rc_frame_left, h.rc_frame_top, h.rc_frame_right, h.rc_frame_bottom] {
                out.extend_from_slice(&(v as i16).to_le_bytes());
            }
        }
        _ => {}
    }
    out.extend_from_slice(&h.strh_extra);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_strf(fcc_type: &str, payload: &[u8]) -> AviStreamFormat {
    if fcc_type == "vids" && payload.len() >= 40 {
        let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let i32le = |o: usize| i32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
        return AviStreamFormat::BitmapInfo {
            size: u32le(0),
            width: i32le(4),
            height: i32le(8),
            planes: u16le(12),
            bit_count: u16le(14),
            compression: fourcc_str(&payload[16..20].try_into().unwrap()),
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
        return AviStreamFormat::WaveFormat {
            format_tag: u16le(0),
            channels: u16le(2),
            samples_per_sec: u32le(4),
            avg_bytes_per_sec: u32le(8),
            block_align: u16le(12),
            bits_per_sample: u16le(14),
            extra: payload.get(16..).map(|s| s.to_vec()).unwrap_or_default(),
        };
    }
    AviStreamFormat::Raw { data: payload.to_vec() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_strf(f: &AviStreamFormat) -> Vec<u8> {
    match f {
        AviStreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => {
            let mut out = Vec::with_capacity(40);
            out.extend_from_slice(&size.to_le_bytes());
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            out.extend_from_slice(&planes.to_le_bytes());
            out.extend_from_slice(&bit_count.to_le_bytes());
            out.extend_from_slice(&fourcc4(compression));
            out.extend_from_slice(&size_image.to_le_bytes());
            out.extend_from_slice(&x_pels_per_meter.to_le_bytes());
            out.extend_from_slice(&y_pels_per_meter.to_le_bytes());
            out.extend_from_slice(&colors_used.to_le_bytes());
            out.extend_from_slice(&colors_important.to_le_bytes());
            out
        }
        AviStreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, extra } => {
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
        AviStreamFormat::Raw { data } => data.clone(),
    }
}
//#endregion 🔖️Header

//#region 🔖️Decode
/// 📥️ Real RIFF/AVI decode: `hdrl` (`avih` + every `strl`'s `strh`/`strf`, plus any nested
/// `hdrl`/`strl` auxiliary children such as `vprp`/`JUNK` typed-raw retained in `hdrl_extra`/
/// `strl_extra`), `movi` (every chunk, assigned to its owning stream by the leading 2-digit stream
/// number in its fourcc), `idx1` (positionally matched to `movi` chunks for the keyframe flag —
/// see module doc comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_avi(bytes: &[u8]) -> Result<AviSnapshot, String> {
    if !sniff_real_bytes(bytes) {
        return Err("avi: missing RIFF/AVI magic".into());
    }
    let body = &bytes[12..bytes.len().min(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize + 8)];

    let mut main_header = None;
    let mut hdrl_extra: Vec<RiffChunk> = Vec::new();
    let mut stream_headers: Vec<(AviStreamHeader, AviStreamFormat, Vec<RiffChunk>)> = Vec::new();
    let mut movi_chunks: Vec<(String, Vec<u8>)> = Vec::new();
    let mut idx1_entries: Vec<u32> = Vec::new();
    let mut idx1_present = false;
    let mut unknown_chunks = Vec::new();

    for item in iter_riff(body) {
        let entry = item?;
        if &entry.fourcc == b"LIST" {
            let list_type = &entry.payload[0..4];
            let list_body = &entry.payload[4..];
            match list_type {
                b"hdrl" => {
                    for hitem in iter_riff(list_body) {
                        let h = hitem?;
                        if &h.fourcc == b"avih" {
                            main_header = Some(parse_avih(h.payload)?);
                        } else if &h.fourcc == b"LIST" && &h.payload[0..4] == b"strl" {
                            let strl_body = &h.payload[4..];
                            let mut strh = None;
                            let mut strf_bytes: Option<&[u8]> = None;
                            let mut strl_extra: Vec<RiffChunk> = Vec::new();
                            for sitem in iter_riff(strl_body) {
                                let s = sitem?;
                                if &s.fourcc == b"strh" {
                                    strh = Some(parse_strh(s.payload)?);
                                } else if &s.fourcc == b"strf" {
                                    strf_bytes = Some(s.payload);
                                } else if &s.fourcc == b"LIST" {
                                    let sub_type = fourcc_str(&s.payload[0..4].try_into().unwrap());
                                    strl_extra.push(RiffChunk { fourcc: format!("LIST:{sub_type}"), data: s.payload[4..].to_vec() });
                                } else {
                                    // 📦 e.g. a real `vprp` (video properties) or `JUNK` padding chunk — no
                                    // typed slot of its own, retained verbatim (see snapshot's module doc).
                                    strl_extra.push(RiffChunk { fourcc: fourcc_str(&s.fourcc), data: s.payload.to_vec() });
                                }
                            }
                            let strh = strh.ok_or("avi: strl missing strh")?;
                            let strf = parse_strf(&strh.fcc_type, strf_bytes.ok_or("avi: strl missing strf")?);
                            stream_headers.push((strh, strf, strl_extra));
                        } else if &h.fourcc == b"LIST" {
                            let sub_type = fourcc_str(&h.payload[0..4].try_into().unwrap());
                            hdrl_extra.push(RiffChunk { fourcc: format!("LIST:{sub_type}"), data: h.payload[4..].to_vec() });
                        } else {
                            // 📦 e.g. a real `JUNK` padding chunk directly inside `hdrl` — no typed slot
                            // of its own, retained verbatim (see snapshot's module doc).
                            hdrl_extra.push(RiffChunk { fourcc: fourcc_str(&h.fourcc), data: h.payload.to_vec() });
                        }
                    }
                }
                b"movi" => {
                    for mitem in iter_riff(list_body) {
                        let m = mitem?;
                        movi_chunks.push((fourcc_str(&m.fourcc), m.payload.to_vec()));
                    }
                }
                other => unknown_chunks.push(RiffChunk { fourcc: format!("LIST:{}", String::from_utf8_lossy(other)), data: list_body.to_vec() }),
            }
        } else if &entry.fourcc == b"idx1" {
            idx1_present = true;
            let mut r = entry.payload;
            while r.len() >= 16 {
                idx1_entries.push(u32::from_le_bytes(r[4..8].try_into().unwrap()));
                r = &r[16..];
            }
        } else {
            unknown_chunks.push(RiffChunk { fourcc: fourcc_str(&entry.fourcc), data: entry.payload.to_vec() });
        }
    }

    let mut streams: Vec<AviStream> = stream_headers.into_iter().map(|(strh, strf, strl_extra)| AviStream { strh, strf, chunks: Vec::new(), strl_extra }).collect();
    let idx1_matches_by_position = idx1_present && idx1_entries.len() == movi_chunks.len();
    for (i, (fourcc, data)) in movi_chunks.into_iter().enumerate() {
        let stream_index: usize = fourcc.get(0..2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let keyframe = if idx1_matches_by_position { idx1_entries[i] & 0x10 != 0 } else { true };
        if let Some(stream) = streams.get_mut(stream_index) {
            stream.chunks.push(AviChunk { fourcc, data, keyframe });
        } else {
            unknown_chunks.push(RiffChunk { fourcc: format!("movi:{fourcc}"), data });
        }
    }

    Ok(AviSnapshot { schema: STDIO_AVI_DOCUMENT_SCHEMA.into(), main_header: main_header.ok_or("avi: hdrl missing avih")?, streams, idx1_present, unknown_chunks, hdrl_extra })
}
//#endregion 🔖️Decode

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
        let mut offset = 4u32; // 🧭 relative to the movi LIST payload start, including its own "movi" tag.
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
enum AviEncodePhase {
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
pub struct AviEncodeCursor {
    phase: AviEncodePhase,
    stream_index: usize,
    item_index: usize,
    offset: usize,
    hdrl_children_bytes: usize,
    movi_children_bytes: usize,
    unknown_bytes: usize,
    total_chunks: usize,
    current_stream_children_bytes: usize,
    idx1_offset: u32,
    emitted_bytes: u64,
}

impl AviEncodeCursor {
    pub fn new(_: &AviSnapshot) -> Self {
        Self {
            phase: AviEncodePhase::MeasureStreams,
            stream_index: 0,
            item_index: 0,
            offset: 0,
            hdrl_children_bytes: chunk_total(56).expect("fixed AVI main header fits"),
            movi_children_bytes: 0,
            unknown_bytes: 0,
            total_chunks: 0,
            current_stream_children_bytes: 0,
            idx1_offset: 4,
            emitted_bytes: 0,
        }
    }

    pub fn emitted_bytes(&self) -> u64 {
        self.emitted_bytes
    }

    pub fn advance(&mut self, snapshot: &AviSnapshot, maximum_bytes: usize) -> Result<AviEncodeAdvance, String> {
        if maximum_bytes == 0 {
            return Err("avi.encode.zero-byte-grant".into());
        }
        match self.phase {
            AviEncodePhase::MeasureStreams => self.measure_stream(snapshot),
            AviEncodePhase::MeasureStreamExtras => self.measure_stream_extra(snapshot),
            AviEncodePhase::MeasureHdrlExtras => self.measure_hdrl_extra(snapshot),
            AviEncodePhase::MeasureMovi => self.measure_movi(snapshot),
            AviEncodePhase::MeasureUnknown => self.measure_unknown(snapshot),
            AviEncodePhase::RiffHeader => {
                let hdrl = list_total(self.hdrl_children_bytes)?;
                let movi = list_total(self.movi_children_bytes)?;
                let idx1 = if snapshot.idx1_present { chunk_total(self.total_chunks.checked_mul(16).ok_or("avi.encode.idx1-size-overflow")?)? } else { 0 };
                let body = 4usize.checked_add(hdrl).and_then(|value| value.checked_add(movi)).and_then(|value| value.checked_add(idx1)).and_then(|value| value.checked_add(self.unknown_bytes)).ok_or("avi.encode.riff-size-overflow")?;
                let bytes = riff_root_header(body)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::HdrlHeader)
            }
            AviEncodePhase::HdrlHeader => {
                let bytes = list_header(b"hdrl", self.hdrl_children_bytes)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::AvihHeader)
            }
            AviEncodePhase::AvihHeader => {
                let bytes = chunk_header(b"avih", 56)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::AvihData)
            }
            AviEncodePhase::AvihData => {
                let bytes = avih_fixed(&snapshot.main_header);
                let next = self.phase_after_avih(snapshot);
                self.emit_fixed(&bytes, maximum_bytes, next)
            }
            AviEncodePhase::MeasureEmittedStream => self.measure_emitted_stream(snapshot),
            AviEncodePhase::MeasureEmittedStreamExtras => self.measure_emitted_stream_extra(snapshot),
            AviEncodePhase::StreamListHeader => {
                let bytes = list_header(b"strl", self.current_stream_children_bytes)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamHeaderHeader)
            }
            AviEncodePhase::StreamHeaderHeader => {
                let stream = self.stream(snapshot)?;
                let bytes = chunk_header(b"strh", stream_header_payload_len(stream)?)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamHeaderData)
            }
            AviEncodePhase::StreamHeaderData => {
                let stream = self.stream(snapshot)?;
                let bytes = stream_header_fixed(&stream.strh)?;
                let next = if stream.strh.strh_extra.is_empty() { self.phase_after_stream_header_data(stream) } else { AviEncodePhase::StreamHeaderExtra };
                self.emit_fixed(&bytes, maximum_bytes, next)
            }
            AviEncodePhase::StreamHeaderExtra => {
                let stream = self.stream(snapshot)?;
                let extra = &stream.strh.strh_extra;
                let next = self.phase_after_stream_header_data(stream);
                self.emit_borrowed(extra, maximum_bytes, next)
            }
            AviEncodePhase::StreamHeaderPad => self.emit_padding(maximum_bytes, AviEncodePhase::StreamFormatHeader),
            AviEncodePhase::StreamFormatHeader => {
                let stream = self.stream(snapshot)?;
                let bytes = chunk_header(b"strf", stream_format_payload_len(&stream.strf))?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamFormatData)
            }
            AviEncodePhase::StreamFormatData => {
                let stream = self.stream(snapshot)?;
                let bytes = stream_format_fixed(&stream.strf);
                let next = if stream_format_extra(&stream.strf).is_empty() {
                    if stream_format_payload_len(&stream.strf) % 2 == 1 { AviEncodePhase::StreamFormatPad } else { AviEncodePhase::StreamFormatDone }
                } else {
                    AviEncodePhase::StreamFormatExtra
                };
                if bytes.is_empty() {
                    self.phase = next;
                    return Ok(AviEncodeAdvance::Progress);
                }
                self.emit_fixed(&bytes, maximum_bytes, next)
            }
            AviEncodePhase::StreamFormatExtra => {
                let stream = self.stream(snapshot)?;
                let bytes = stream_format_extra(&stream.strf);
                let next = if stream_format_payload_len(&stream.strf) % 2 == 1 { AviEncodePhase::StreamFormatPad } else { AviEncodePhase::StreamFormatDone };
                self.emit_borrowed(bytes, maximum_bytes, next)
            }
            AviEncodePhase::StreamFormatPad => self.emit_padding(maximum_bytes, AviEncodePhase::StreamFormatDone),
            AviEncodePhase::StreamFormatDone => {
                self.phase = self.phase_after_stream_format(snapshot)?;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::StreamExtraHeader => {
                let item = self.stream_extra(snapshot)?;
                let bytes = raw_header(item)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamExtraData)
            }
            AviEncodePhase::StreamExtraData => {
                let item = self.stream_extra(snapshot)?;
                let next = if raw_payload_len(item)? % 2 == 1 { AviEncodePhase::StreamExtraPad } else { AviEncodePhase::StreamExtraDone };
                self.emit_borrowed(&item.data, maximum_bytes, next)
            }
            AviEncodePhase::StreamExtraPad => self.emit_padding(maximum_bytes, AviEncodePhase::StreamExtraDone),
            AviEncodePhase::StreamExtraDone => {
                self.phase = self.phase_after_stream_extra(snapshot)?;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::HdrlExtraHeader => {
                let item = snapshot.hdrl_extra.get(self.item_index).ok_or("avi.encode.hdrl-extra-missing")?;
                let bytes = raw_header(item)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::HdrlExtraData)
            }
            AviEncodePhase::HdrlExtraData => {
                let item = snapshot.hdrl_extra.get(self.item_index).ok_or("avi.encode.hdrl-extra-missing")?;
                let next = if raw_payload_len(item)? % 2 == 1 { AviEncodePhase::HdrlExtraPad } else { AviEncodePhase::HdrlExtraDone };
                self.emit_borrowed(&item.data, maximum_bytes, next)
            }
            AviEncodePhase::HdrlExtraPad => self.emit_padding(maximum_bytes, AviEncodePhase::HdrlExtraDone),
            AviEncodePhase::HdrlExtraDone => {
                self.phase = self.phase_after_hdrl_extra(snapshot);
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::MoviHeader => {
                self.stream_index = 0;
                self.item_index = 0;
                let bytes = list_header(b"movi", self.movi_children_bytes)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::MoviSeek)
            }
            AviEncodePhase::MoviSeek => {
                self.phase = self.seek_movi(snapshot, false);
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::MoviChunkHeader => {
                let chunk = self.movi_chunk(snapshot)?;
                let bytes = chunk_header(&fourcc4(&chunk.fourcc), chunk.data.len())?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::MoviChunkData)
            }
            AviEncodePhase::MoviChunkData => {
                let chunk = self.movi_chunk(snapshot)?;
                let next = if chunk.data.len() % 2 == 1 { AviEncodePhase::MoviChunkPad } else { AviEncodePhase::MoviChunkDone };
                self.emit_borrowed(&chunk.data, maximum_bytes, next)
            }
            AviEncodePhase::MoviChunkPad => self.emit_padding(maximum_bytes, AviEncodePhase::MoviChunkDone),
            AviEncodePhase::MoviChunkDone => {
                self.item_index += 1;
                self.phase = AviEncodePhase::MoviSeek;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::Idx1Header => {
                let bytes = chunk_header(b"idx1", self.total_chunks.checked_mul(16).ok_or("avi.encode.idx1-size-overflow")?)?;
                self.idx1_offset = 4;
                self.stream_index = 0;
                self.item_index = 0;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::Idx1Seek)
            }
            AviEncodePhase::Idx1Seek => {
                self.phase = self.seek_movi(snapshot, true);
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::Idx1Entry => {
                let chunk = self.movi_chunk(snapshot)?;
                let mut bytes = Vec::with_capacity(16);
                bytes.extend_from_slice(&fourcc4(&chunk.fourcc));
                bytes.extend_from_slice(&(if chunk.keyframe { 0x10u32 } else { 0 }).to_le_bytes());
                bytes.extend_from_slice(&self.idx1_offset.to_le_bytes());
                bytes.extend_from_slice(&u32::try_from(chunk.data.len()).map_err(|_| "avi.encode.chunk-too-large")?.to_le_bytes());
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::Idx1EntryDone)
            }
            AviEncodePhase::Idx1EntryDone => {
                let bytes = self.movi_chunk(snapshot)?.data.len();
                let total = chunk_total(bytes)?;
                self.idx1_offset = self.idx1_offset.checked_add(u32::try_from(total).map_err(|_| "avi.encode.movi-offset-overflow")?).ok_or("avi.encode.movi-offset-overflow")?;
                self.item_index += 1;
                self.phase = AviEncodePhase::Idx1Seek;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::UnknownSeek => {
                self.phase = match snapshot.unknown_chunks.get(self.item_index) {
                    Some(item) if item.fourcc.starts_with("movi:") => {
                        self.item_index += 1;
                        AviEncodePhase::UnknownSeek
                    }
                    Some(_) => AviEncodePhase::UnknownHeader,
                    None => AviEncodePhase::Complete,
                };
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::UnknownHeader => {
                let item = self.unknown(snapshot)?;
                let bytes = raw_header(item)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::UnknownData)
            }
            AviEncodePhase::UnknownData => {
                let item = self.unknown(snapshot)?;
                let next = if raw_payload_len(item)? % 2 == 1 { AviEncodePhase::UnknownPad } else { AviEncodePhase::UnknownDone };
                self.emit_borrowed(&item.data, maximum_bytes, next)
            }
            AviEncodePhase::UnknownPad => self.emit_padding(maximum_bytes, AviEncodePhase::UnknownDone),
            AviEncodePhase::UnknownDone => {
                self.item_index += 1;
                self.phase = AviEncodePhase::UnknownSeek;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::Complete => Ok(AviEncodeAdvance::Complete),
        }
    }

    fn measure_stream(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let Some(stream) = snapshot.streams.get(self.stream_index) else {
            self.phase = AviEncodePhase::MeasureHdrlExtras;
            self.item_index = 0;
            return Ok(AviEncodeAdvance::Progress);
        };
        self.current_stream_children_bytes = chunk_total(stream_header_payload_len(stream)?)?.checked_add(chunk_total(stream_format_payload_len(&stream.strf))?).ok_or("avi.encode.strl-size-overflow")?;
        self.item_index = 0;
        self.phase = AviEncodePhase::MeasureStreamExtras;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_stream_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let stream = self.stream(snapshot)?;
        if let Some(item) = stream.strl_extra.get(self.item_index) {
            self.current_stream_children_bytes = self.current_stream_children_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.strl-size-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.hdrl_children_bytes = self.hdrl_children_bytes.checked_add(list_total(self.current_stream_children_bytes)?).ok_or("avi.encode.hdrl-size-overflow")?;
        self.stream_index += 1;
        self.phase = AviEncodePhase::MeasureStreams;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_hdrl_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        if let Some(item) = snapshot.hdrl_extra.get(self.item_index) {
            self.hdrl_children_bytes = self.hdrl_children_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.hdrl-size-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.phase = AviEncodePhase::MeasureMovi;
        self.stream_index = 0;
        self.item_index = 0;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_movi(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let Some(stream) = snapshot.streams.get(self.stream_index) else {
            self.phase = AviEncodePhase::MeasureUnknown;
            self.item_index = 0;
            return Ok(AviEncodeAdvance::Progress);
        };
        if let Some(chunk) = stream.chunks.get(self.item_index) {
            self.movi_children_bytes = self.movi_children_bytes.checked_add(chunk_total(chunk.data.len())?).ok_or("avi.encode.movi-size-overflow")?;
            self.total_chunks = self.total_chunks.checked_add(1).ok_or("avi.encode.chunk-count-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.stream_index += 1;
        self.item_index = 0;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_unknown(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        if let Some(item) = snapshot.unknown_chunks.get(self.item_index) {
            if !item.fourcc.starts_with("movi:") {
                self.unknown_bytes = self.unknown_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.unknown-size-overflow")?;
            }
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.phase = AviEncodePhase::RiffHeader;
        self.stream_index = 0;
        self.item_index = 0;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_emitted_stream(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let Some(stream) = snapshot.streams.get(self.stream_index) else {
            self.phase = AviEncodePhase::HdrlExtraHeader;
            self.item_index = 0;
            if snapshot.hdrl_extra.is_empty() {
                self.phase = AviEncodePhase::MoviHeader;
            }
            return Ok(AviEncodeAdvance::Progress);
        };
        self.current_stream_children_bytes = chunk_total(stream_header_payload_len(stream)?)?.checked_add(chunk_total(stream_format_payload_len(&stream.strf))?).ok_or("avi.encode.strl-size-overflow")?;
        self.item_index = 0;
        self.phase = AviEncodePhase::MeasureEmittedStreamExtras;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_emitted_stream_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let stream = self.stream(snapshot)?;
        if let Some(item) = stream.strl_extra.get(self.item_index) {
            self.current_stream_children_bytes = self.current_stream_children_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.strl-size-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.item_index = 0;
        self.phase = AviEncodePhase::StreamListHeader;
        Ok(AviEncodeAdvance::Progress)
    }

    fn phase_after_avih(&mut self, snapshot: &AviSnapshot) -> AviEncodePhase {
        self.stream_index = 0;
        self.item_index = 0;
        if snapshot.streams.is_empty() {
            if snapshot.hdrl_extra.is_empty() { AviEncodePhase::MoviHeader } else { AviEncodePhase::HdrlExtraHeader }
        } else {
            AviEncodePhase::MeasureEmittedStream
        }
    }

    fn phase_after_stream_header_data(&self, stream: &AviStream) -> AviEncodePhase {
        if stream_header_payload_len(stream).unwrap_or(0) % 2 == 1 { AviEncodePhase::StreamHeaderPad } else { AviEncodePhase::StreamFormatHeader }
    }

    fn phase_after_stream_format(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodePhase, String> {
        let stream = self.stream(snapshot)?;
        Ok(if stream.strl_extra.is_empty() { self.stream_index += 1; AviEncodePhase::MeasureEmittedStream } else { self.item_index = 0; AviEncodePhase::StreamExtraHeader })
    }

    fn phase_after_stream_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodePhase, String> {
        let stream = self.stream(snapshot)?;
        self.item_index += 1;
        if self.item_index < stream.strl_extra.len() {
            Ok(AviEncodePhase::StreamExtraHeader)
        } else {
            self.stream_index += 1;
            self.item_index = 0;
            Ok(AviEncodePhase::MeasureEmittedStream)
        }
    }

    fn phase_after_hdrl_extra(&mut self, snapshot: &AviSnapshot) -> AviEncodePhase {
        self.item_index += 1;
        if self.item_index < snapshot.hdrl_extra.len() { AviEncodePhase::HdrlExtraHeader } else { self.item_index = 0; AviEncodePhase::MoviHeader }
    }

    fn stream<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a AviStream, String> {
        snapshot.streams.get(self.stream_index).ok_or_else(|| "avi.encode.stream-missing".into())
    }

    fn stream_extra<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a RiffChunk, String> {
        self.stream(snapshot)?.strl_extra.get(self.item_index).ok_or_else(|| "avi.encode.stream-extra-missing".into())
    }

    fn movi_chunk<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a AviChunk, String> {
        snapshot.streams.get(self.stream_index).and_then(|stream| stream.chunks.get(self.item_index)).ok_or_else(|| "avi.encode.movi-chunk-missing".into())
    }

    fn unknown<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a RiffChunk, String> {
        snapshot.unknown_chunks.get(self.item_index).filter(|item| !item.fourcc.starts_with("movi:")).ok_or_else(|| "avi.encode.unknown-chunk-missing".into())
    }

    fn seek_movi(&mut self, snapshot: &AviSnapshot, idx1: bool) -> AviEncodePhase {
        match snapshot.streams.get(self.stream_index) {
            Some(stream) if self.item_index < stream.chunks.len() => if idx1 { AviEncodePhase::Idx1Entry } else { AviEncodePhase::MoviChunkHeader },
            Some(_) => {
                self.stream_index += 1;
                self.item_index = 0;
                if idx1 { AviEncodePhase::Idx1Seek } else { AviEncodePhase::MoviSeek }
            }
            None => {
                self.item_index = 0;
                if idx1 || !snapshot.idx1_present { AviEncodePhase::UnknownSeek } else { AviEncodePhase::Idx1Header }
            }
        }
    }

    fn emit_fixed(&mut self, bytes: &[u8], maximum_bytes: usize, next: AviEncodePhase) -> Result<AviEncodeAdvance, String> {
        self.emit_borrowed(bytes, maximum_bytes, next)
    }

    fn emit_borrowed(&mut self, bytes: &[u8], maximum_bytes: usize, next: AviEncodePhase) -> Result<AviEncodeAdvance, String> {
        let end = self.offset.checked_add(maximum_bytes).unwrap_or(usize::MAX).min(bytes.len());
        let chunk = bytes.get(self.offset..end).ok_or("avi.encode.source-shape-changed")?.to_vec();
        self.offset = end;
        self.emitted_bytes = self.emitted_bytes.checked_add(chunk.len() as u64).ok_or("avi.encode.emitted-size-overflow")?;
        if self.offset == bytes.len() {
            self.offset = 0;
            self.phase = next;
        }
        if chunk.is_empty() { Ok(AviEncodeAdvance::Progress) } else { Ok(AviEncodeAdvance::Chunk(chunk)) }
    }

    fn emit_padding(&mut self, maximum_bytes: usize, next: AviEncodePhase) -> Result<AviEncodeAdvance, String> {
        self.emit_fixed(&[0], maximum_bytes, next)
    }
}

fn chunk_total(payload: usize) -> Result<usize, String> {
    u32::try_from(payload).map_err(|_| "avi.encode.chunk-too-large")?;
    8usize.checked_add(payload).and_then(|value| value.checked_add(payload % 2)).ok_or_else(|| "avi.encode.chunk-size-overflow".into())
}

fn list_total(children: usize) -> Result<usize, String> {
    let payload = 4usize.checked_add(children).ok_or("avi.encode.list-size-overflow")?;
    chunk_total(payload)
}

fn raw_payload_len(item: &RiffChunk) -> Result<usize, String> {
    if item.fourcc.starts_with("LIST:") { 4usize.checked_add(item.data.len()).ok_or_else(|| "avi.encode.list-size-overflow".into()) } else { Ok(item.data.len()) }
}

fn raw_total(item: &RiffChunk) -> Result<usize, String> {
    chunk_total(raw_payload_len(item)?)
}

fn chunk_header(fourcc: &[u8; 4], payload: usize) -> Result<[u8; 8], String> {
    let mut bytes = [0u8; 8];
    bytes[..4].copy_from_slice(fourcc);
    bytes[4..].copy_from_slice(&u32::try_from(payload).map_err(|_| "avi.encode.chunk-too-large")?.to_le_bytes());
    Ok(bytes)
}

fn list_header(list_type: &[u8; 4], children: usize) -> Result<[u8; 12], String> {
    let payload = 4usize.checked_add(children).ok_or("avi.encode.list-size-overflow")?;
    let chunk = chunk_header(b"LIST", payload)?;
    let mut bytes = [0u8; 12];
    bytes[..8].copy_from_slice(&chunk);
    bytes[8..].copy_from_slice(list_type);
    Ok(bytes)
}

fn riff_root_header(body: usize) -> Result<[u8; 12], String> {
    let mut bytes = [0u8; 12];
    bytes[..4].copy_from_slice(b"RIFF");
    bytes[4..8].copy_from_slice(&u32::try_from(body).map_err(|_| "avi.encode.riff-too-large")?.to_le_bytes());
    bytes[8..].copy_from_slice(b"AVI ");
    Ok(bytes)
}

fn raw_header(item: &RiffChunk) -> Result<Vec<u8>, String> {
    if let Some(list_type) = item.fourcc.strip_prefix("LIST:") {
        Ok(list_header(&fourcc4(list_type), item.data.len())?.to_vec())
    } else {
        Ok(chunk_header(&fourcc4(&item.fourcc), item.data.len())?.to_vec())
    }
}

fn avih_fixed(header: &AviMainHeader) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(56);
    for value in [header.micro_sec_per_frame, header.max_bytes_per_sec, header.padding_granularity, header.flags, header.total_frames, header.initial_frames, header.streams, header.suggested_buffer_size, header.width, header.height] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    for index in 0..4 {
        bytes.extend_from_slice(&header.reserved.get(index).copied().unwrap_or(0).to_le_bytes());
    }
    bytes
}

fn stream_header_payload_len(stream: &AviStream) -> Result<usize, String> {
    let width = match stream.strh.rc_frame_width { 0 => 0, 8 => 8, 16 => 16, _ => return Err("avi.encode.rc-frame-width-invalid".into()) };
    48usize.checked_add(width).and_then(|value| value.checked_add(stream.strh.strh_extra.len())).ok_or_else(|| "avi.encode.strh-size-overflow".into())
}

fn stream_header_fixed(header: &AviStreamHeader) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity(64);
    bytes.extend_from_slice(&fourcc4(&header.fcc_type));
    bytes.extend_from_slice(&fourcc4(&header.fcc_handler));
    bytes.extend_from_slice(&header.flags.to_le_bytes());
    bytes.extend_from_slice(&header.priority.to_le_bytes());
    bytes.extend_from_slice(&header.language.to_le_bytes());
    for value in [header.initial_frames, header.scale, header.rate, header.start, header.length, header.suggested_buffer_size] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes.extend_from_slice(&header.quality.to_le_bytes());
    bytes.extend_from_slice(&header.sample_size.to_le_bytes());
    match header.rc_frame_width {
        16 => for value in [header.rc_frame_left, header.rc_frame_top, header.rc_frame_right, header.rc_frame_bottom] { bytes.extend_from_slice(&value.to_le_bytes()); },
        8 => for value in [header.rc_frame_left, header.rc_frame_top, header.rc_frame_right, header.rc_frame_bottom] { bytes.extend_from_slice(&(value as i16).to_le_bytes()); },
        0 => {}
        _ => return Err("avi.encode.rc-frame-width-invalid".into()),
    }
    Ok(bytes)
}

fn stream_format_payload_len(format: &AviStreamFormat) -> usize {
    match format {
        AviStreamFormat::BitmapInfo { .. } => 40,
        AviStreamFormat::WaveFormat { extra, .. } => 16usize.saturating_add(extra.len()),
        AviStreamFormat::Raw { data } => data.len(),
    }
}

fn stream_format_fixed(format: &AviStreamFormat) -> Vec<u8> {
    match format {
        AviStreamFormat::BitmapInfo { size, width, height, planes, bit_count, compression, size_image, x_pels_per_meter, y_pels_per_meter, colors_used, colors_important } => {
            let mut bytes = Vec::with_capacity(40);
            bytes.extend_from_slice(&size.to_le_bytes());
            bytes.extend_from_slice(&width.to_le_bytes());
            bytes.extend_from_slice(&height.to_le_bytes());
            bytes.extend_from_slice(&planes.to_le_bytes());
            bytes.extend_from_slice(&bit_count.to_le_bytes());
            bytes.extend_from_slice(&fourcc4(compression));
            bytes.extend_from_slice(&size_image.to_le_bytes());
            bytes.extend_from_slice(&x_pels_per_meter.to_le_bytes());
            bytes.extend_from_slice(&y_pels_per_meter.to_le_bytes());
            bytes.extend_from_slice(&colors_used.to_le_bytes());
            bytes.extend_from_slice(&colors_important.to_le_bytes());
            bytes
        }
        AviStreamFormat::WaveFormat { format_tag, channels, samples_per_sec, avg_bytes_per_sec, block_align, bits_per_sample, .. } => {
            let mut bytes = Vec::with_capacity(16);
            bytes.extend_from_slice(&format_tag.to_le_bytes());
            bytes.extend_from_slice(&channels.to_le_bytes());
            bytes.extend_from_slice(&samples_per_sec.to_le_bytes());
            bytes.extend_from_slice(&avg_bytes_per_sec.to_le_bytes());
            bytes.extend_from_slice(&block_align.to_le_bytes());
            bytes.extend_from_slice(&bits_per_sample.to_le_bytes());
            bytes
        }
        AviStreamFormat::Raw { .. } => Vec::new(),
    }
}

fn stream_format_extra(format: &AviStreamFormat) -> &[u8] {
    match format {
        AviStreamFormat::BitmapInfo { .. } => &[],
        AviStreamFormat::WaveFormat { extra, .. } => extra,
        AviStreamFormat::Raw { data } => data,
    }
}
//#endregion 🔖️Encode

pub mod playback {
    use super::{AviEncodeAdvance, AviEncodeCursor, AviSnapshot, STDIO_AVI_DOCUMENT_SCHEMA};
    use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
    use semio_framework_plugin::{ArtifactApp, ArtifactOwnedToolJobFactory, ArtifactReservedToolJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Fault};
    use semio_s_artifact_stdio_contract::media_export::{IncrementalMediaAdvance, IncrementalMediaExportJob, IncrementalMediaExportSpec, PLAYBACK_CONTRACT, PLAYBACK_TOOL_ID};
    use std::marker::PhantomData;

    pub const MIME_TYPE: &str = "video/x-msvideo";
    pub const MEDIA_SCHEMA: &str = "stdio.avi";
    pub const PAYLOAD_SCHEMA: &str = "stdio.avi.playback-export.v1";
    pub const MEDIA_TYPE: semio_framework_plugin::MediaType = semio_s_artifact_stdio_contract::media_export::PLAYBACK_MEDIA_TYPE;

    pub struct AviPlaybackExport;

    impl IncrementalMediaExportSpec for AviPlaybackExport {
        type Snapshot = AviSnapshot;
        type Cursor = AviEncodeCursor;
        const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;
        const MEDIA_SCHEMA: &'static str = MEDIA_SCHEMA;
        const MIME_TYPE: &'static str = MIME_TYPE;
        const PAYLOAD_SCHEMA: &'static str = PAYLOAD_SCHEMA;
        const STAGE: &'static str = "encode-avi";
        const KIND_ID: &'static str = "s.stdio.avi";
        const ARTIFACT_ID: &'static str = "stdio.avi";
        const ARTIFACT_NAME: &'static str = "AVI Video";
        const COMPONENT_KIND: &'static str = "video";

        fn cursor(snapshot: &AviSnapshot) -> Result<AviEncodeCursor, Fault> {
            Ok(AviEncodeCursor::new(snapshot))
        }

        fn advance(cursor: &mut AviEncodeCursor, snapshot: &AviSnapshot, maximum_bytes: usize) -> Result<IncrementalMediaAdvance, Fault> {
            cursor.advance(snapshot, maximum_bytes).map(|advance| match advance {
                AviEncodeAdvance::Progress => IncrementalMediaAdvance::Progress,
                AviEncodeAdvance::Chunk(bytes) => IncrementalMediaAdvance::Chunk(bytes),
                AviEncodeAdvance::Complete => IncrementalMediaAdvance::Complete,
            }).map_err(Fault::from)
        }
    }

    pub type AviPlaybackExportJob = IncrementalMediaExportJob<AviPlaybackExport>;

    pub struct AviMediaExportJobFactory<A: ArtifactApp<Snapshot = AviSnapshot>> {
        keys: [ToolFactoryKey; 1],
        owner: PhantomData<fn() -> A>,
    }

    impl<A: ArtifactApp<Snapshot = AviSnapshot>> AviMediaExportJobFactory<A> {
        pub fn new(controller: &str) -> Self {
            Self { keys: [ToolFactoryKey::new(controller, PLAYBACK_TOOL_ID)], owner: PhantomData }
        }
    }

    impl<A: ArtifactApp<Snapshot = AviSnapshot>> ToolJobFactory for AviMediaExportJobFactory<A> {
        type Payload = ArtifactReservedToolJob;
        type Job = ArtifactReservedToolJob;
        fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
        fn payload_schema_id(&self) -> &str { PAYLOAD_SCHEMA }
        fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
        fn execution_contract(&self) -> ToolExecutionContract { PLAYBACK_CONTRACT }
        fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(payload) }
    }

    impl<A: ArtifactApp<Snapshot = AviSnapshot>> ArtifactOwnedToolJobFactory for AviMediaExportJobFactory<A> {
        type Owner = A;
        const TOOL_IDS: &'static [&'static str] = &[PLAYBACK_TOOL_ID];
        const DOCUMENT_SCHEMA: &'static str = STDIO_AVI_DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: PLAYBACK_TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1_0::subsets::any::io::AviComposer as AviRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<AviRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::standards::v1_0::subsets::any::schema::diff::AviDiff;
    use crate::standards::v1_0::subsets::any::schema::mutations::{AviMutation};

    use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct AviBuilderConstruction {
        snapshot: AviSnapshot,
    }

    impl ArtifactBuilder for AviBuilderConstruction {
        type Snapshot = AviSnapshot;
        type Mutation = AviMutation;
        type Diff = AviDiff;
        fn empty() -> Self {
            Self { snapshot: AviSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<AviSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<AviSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1_0::subsets::any::io;
    use crate::standards::v1_0::subsets::any::schema::snapshot::{AviSnapshot, STDIO_AVI_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct AviParts {
        pub snapshot: Option<AviSnapshot>,
    }

    pub struct AviAnalyzerAnalysis;

    impl ArtifactAnalysis for AviAnalyzerAnalysis {
        type Parts = AviParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return semio_framework_plugin::io::Confidence::High;
                    }
                    let marker = STDIO_AVI_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_AVI_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = AviParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <AviSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <AviSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec AviBuilderFacets {
        construction: AviBuilderConstruction,
        analysis: AviAnalyzerAnalysis,
        composition: crate::standards::v1_0::subsets::any::io::derived_composition::AviComposerComposition,
    }
    builder: AviBuilder,
    analyzer: AviAnalyzer,
    composer: AviComposer,
);
