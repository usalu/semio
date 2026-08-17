//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::AviAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactAnalyzer as _, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.avi", standard: StandardId("1.0"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct AviComposerComposition;

    impl ArtifactComposition for AviComposerComposition {
        type Snapshot = AviSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::avi::standards::v1_0::subsets::any::schema::avi_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<AviSnapshot, crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation>(
            crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA,
        ));
    }

    /// 💡️ Registers `s.stdio.avi.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::avi::standards::v1_0::subsets::any::schema::inferences::avi_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk, STDIO_AVI_DOCUMENT_SCHEMA};

//#region 🔖️Riff
struct RiffEntry<'a> {
    fourcc: [u8; 4],
    payload: &'a [u8],
}

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

fn fourcc_str(f: &[u8; 4]) -> String {
    String::from_utf8_lossy(f).into_owned()
}

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

fn write_list(list_type: &[u8; 4], children: &[u8]) -> Vec<u8> {
    let mut payload = list_type.to_vec();
    payload.extend_from_slice(children);
    write_chunk(b"LIST", &payload)
}

fn fourcc4(s: &str) -> [u8; 4] {
    let mut out = [b' '; 4];
    for (i, b) in s.as_bytes().iter().take(4).enumerate() {
        out[i] = *b;
    }
    out
}
//#endregion 🔖️Riff

//#region 🔖️Sniff
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"AVI "
}
//#endregion 🔖️Sniff

//#region 🔖️Header
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

fn parse_strh(payload: &[u8]) -> Result<AviStreamHeader, String> {
    if payload.len() < 64 {
        return Err("avi: strh shorter than 64 bytes".into());
    }
    let u32le = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let i32le = |o: usize| i32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
    let u16le = |o: usize| u16::from_le_bytes(payload[o..o + 2].try_into().unwrap());
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
        rc_frame_left: i32le(48),
        rc_frame_top: i32le(52),
        rc_frame_right: i32le(56),
        rc_frame_bottom: i32le(60),
    })
}

fn write_strh(h: &AviStreamHeader) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
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
    for v in [h.rc_frame_left, h.rc_frame_top, h.rc_frame_right, h.rc_frame_bottom] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

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
/// 📥️ Real RIFF/AVI decode: `hdrl` (`avih` + every `strl`'s `strh`/`strf`), `movi` (every chunk,
/// assigned to its owning stream by the leading 2-digit stream number in its fourcc), `idx1`
/// (positionally matched to `movi` chunks for the keyframe flag — see module doc comment).
pub fn decode_avi(bytes: &[u8]) -> Result<AviSnapshot, String> {
    if !sniff_real_bytes(bytes) {
        return Err("avi: missing RIFF/AVI magic".into());
    }
    let body = &bytes[12..bytes.len().min(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize + 8)];

    let mut main_header = None;
    let mut stream_headers: Vec<(AviStreamHeader, AviStreamFormat)> = Vec::new();
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
                            for sitem in iter_riff(strl_body) {
                                let s = sitem?;
                                if &s.fourcc == b"strh" {
                                    strh = Some(parse_strh(s.payload)?);
                                }
                                if &s.fourcc == b"strf" {
                                    strf_bytes = Some(s.payload);
                                }
                            }
                            let strh = strh.ok_or("avi: strl missing strh")?;
                            let strf = parse_strf(&strh.fcc_type, strf_bytes.ok_or("avi: strl missing strf")?);
                            stream_headers.push((strh, strf));
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

    let mut streams: Vec<AviStream> = stream_headers.into_iter().map(|(strh, strf)| AviStream { strh, strf, chunks: Vec::new() }).collect();
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

    Ok(AviSnapshot { schema: STDIO_AVI_DOCUMENT_SCHEMA.into(), main_header: main_header.ok_or("avi: hdrl missing avih")?, streams, idx1_present, unknown_chunks })
}
//#endregion 🔖️Decode

//#region 🔖️Encode
/// ✍️ Real RIFF/AVI encode — layout mirrors this ticket's own `make_avi.py` fixture generator
/// exactly (`hdrl(avih + strl(strh,strf)*)`, `movi(chunk*)`, `idx1` with offsets relative to the
/// `movi` LIST's payload start INCLUDING its own `movi` tag, per the OpenDML convention that
/// generator documents) — byte-identical for the untouched round trip on a single-stream fixture.
pub fn encode_avi(snapshot: &AviSnapshot) -> Vec<u8> {
    let avih = write_chunk(b"avih", &write_avih(&snapshot.main_header));
    let strls: Vec<u8> = snapshot
        .streams
        .iter()
        .flat_map(|s| {
            let strh = write_chunk(b"strh", &write_strh(&s.strh));
            let strf = write_chunk(b"strf", &write_strf(&s.strf));
            write_list(b"strl", &[strh, strf].concat())
        })
        .collect();
    let hdrl = write_list(b"hdrl", &[avih, strls].concat());

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
        if let Some(list_type) = u.fourcc.strip_prefix("LIST:") {
            unknown.extend(write_list(&fourcc4(list_type), &u.data));
        } else if !u.fourcc.starts_with("movi:") {
            unknown.extend(write_chunk(&fourcc4(&u.fourcc), &u.data));
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
//#endregion 🔖️Encode

#[cfg(test)]
mod codec_tests {
    use super::*;

    fn synthetic_snapshot() -> AviSnapshot {
        AviSnapshot {
            schema: STDIO_AVI_DOCUMENT_SCHEMA.into(),
            main_header: AviMainHeader {
                micro_sec_per_frame: 100_000,
                max_bytes_per_sec: 1400,
                padding_granularity: 0,
                flags: 0x10,
                total_frames: 2,
                initial_frames: 0,
                streams: 1,
                suggested_buffer_size: 140,
                width: 16,
                height: 16,
                reserved: vec![0, 0, 0, 0],
            },
            streams: vec![AviStream {
                strh: AviStreamHeader {
                    fcc_type: "vids".into(),
                    fcc_handler: "MJPG".into(),
                    flags: 0,
                    priority: 0,
                    language: 0,
                    initial_frames: 0,
                    scale: 1,
                    rate: 10,
                    start: 0,
                    length: 2,
                    suggested_buffer_size: 140,
                    quality: -1,
                    sample_size: 0,
                    rc_frame_left: 0,
                    rc_frame_top: 0,
                    rc_frame_right: 16,
                    rc_frame_bottom: 16,
                },
                strf: AviStreamFormat::BitmapInfo { size: 40, width: 16, height: 16, planes: 1, bit_count: 24, compression: "MJPG".into(), size_image: 140, x_pels_per_meter: 0, y_pels_per_meter: 0, colors_used: 0, colors_important: 0 },
                chunks: vec![AviChunk { fourcc: "00dc".into(), data: vec![1, 2, 3, 4], keyframe: true }, AviChunk { fourcc: "00dc".into(), data: vec![5, 6, 7], keyframe: true }],
            }],
            idx1_present: true,
            unknown_chunks: vec![],
        }
    }

    #[test]
    fn sniff_recognizes_real_riff_avi_magic() {
        let bytes = encode_avi(&synthetic_snapshot());
        assert!(sniff_real_bytes(&bytes));
        assert!(!sniff_real_bytes(b"not an avi at all!!"));
        let mut wave = b"RIFF".to_vec();
        wave.extend_from_slice(&4u32.to_le_bytes());
        wave.extend_from_slice(b"WAVE");
        assert!(!sniff_real_bytes(&wave));
    }

    #[test]
    fn decode_encode_decode_round_trips_synthetic_snapshot() {
        let snap = synthetic_snapshot();
        let bytes = encode_avi(&snap);
        let back = decode_avi(&bytes).expect("decode");
        assert_eq!(back, snap);
    }

    #[test]
    fn audio_stream_round_trips_via_wave_format() {
        let mut snap = synthetic_snapshot();
        snap.streams.push(AviStream {
            strh: AviStreamHeader {
                fcc_type: "auds".into(),
                fcc_handler: "    ".into(),
                flags: 0,
                priority: 0,
                language: 0,
                initial_frames: 0,
                scale: 1,
                rate: 44100,
                start: 0,
                length: 4,
                suggested_buffer_size: 4,
                quality: 0,
                sample_size: 2,
                rc_frame_left: 0,
                rc_frame_top: 0,
                rc_frame_right: 0,
                rc_frame_bottom: 0,
            },
            strf: AviStreamFormat::WaveFormat { format_tag: 1, channels: 1, samples_per_sec: 44100, avg_bytes_per_sec: 88200, block_align: 2, bits_per_sample: 16, extra: vec![] },
            chunks: vec![AviChunk { fourcc: "01wb".into(), data: vec![9, 9], keyframe: true }],
        });
        snap.main_header.streams = 2;
        let bytes = encode_avi(&snap);
        let back = decode_avi(&bytes).expect("decode");
        assert_eq!(back, snap);
    }

    #[test]
    fn no_idx1_still_round_trips() {
        let mut snap = synthetic_snapshot();
        snap.idx1_present = false;
        let bytes = encode_avi(&snap);
        let back = decode_avi(&bytes).expect("decode");
        assert_eq!(back, snap);
    }

    //#region codec_retention_law — the REAL W0 fixture
    /// 🎬️ The handcrafted-but-real `📼️example.avi` fixture (`generators/w0-fixtures/make_avi.py`,
    /// see `fixtures/avi/NOTES.md`): 732 bytes, 16×16 MJPG, 3 `00dc` frames, real `idx1`.
    const REAL_EXAMPLE_AVI: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/📼️example.avi");

    #[test]
    fn codec_retention_law_decodes_the_real_fixture_with_expected_shape() {
        let snap = decode_avi(REAL_EXAMPLE_AVI).expect("decode the real fixture");
        assert_eq!(snap.main_header.width, 16);
        assert_eq!(snap.main_header.height, 16);
        assert_eq!(snap.main_header.total_frames, 3);
        assert!(snap.idx1_present);
        assert_eq!(snap.streams.len(), 1);
        let stream = &snap.streams[0];
        assert_eq!(stream.strh.fcc_type, "vids");
        assert_eq!(stream.strh.fcc_handler, "MJPG");
        assert_eq!(stream.chunks.len(), 3, "NOTES.md: 3 00dc frame chunks");
        assert!(stream.chunks.iter().all(|c| c.keyframe), "NOTES.md: idx1 AVIIF_KEYFRAME set on all 3 entries");
        match &stream.strf {
            AviStreamFormat::BitmapInfo { width, height, compression, .. } => {
                assert_eq!(*width, 16);
                assert_eq!(*height, 16);
                assert_eq!(compression, "MJPG");
            }
            other => panic!("expected BitmapInfo, got {other:?}"),
        }
    }

    #[test]
    fn codec_retention_law_round_trips_the_real_fixture_byte_identically() {
        // 🧪️ This fixture is simple enough (single stream, no untyped `hdrl` auxiliary fields
        // beyond what `AviMainHeader`/`AviStreamHeader`/`AviStreamFormat` fully type) that this
        // engine achieves LITERAL byte-for-byte round-tripping, not just documented-normal-form —
        // the strongest form of codec_retention_law.
        let snap = decode_avi(REAL_EXAMPLE_AVI).expect("decode");
        let re_encoded = encode_avi(&snap);
        assert_eq!(re_encoded, REAL_EXAMPLE_AVI, "decode(bytes) -> encode(..) must reproduce the real fixture byte-for-byte");

        let round_tripped = decode_avi(&re_encoded).expect("re-decode");
        assert_eq!(round_tripped, snap);

        for chunk in &snap.streams[0].chunks {
            assert!(REAL_EXAMPLE_AVI.windows(chunk.data.len().max(1)).any(|w| w == chunk.data.as_slice()), "chunk data must be a verbatim slice of the real source file");
        }
    }
    //#endregion codec_retention_law
}

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::AviComposer as AviRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<AviRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
