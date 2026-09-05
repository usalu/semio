//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::WavAnalyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct WavComposerComposition;

    impl ArtifactComposition for WavComposerComposition {
        type Snapshot = WavSnapshot;
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
                return Err(ComposeError { message: "WavComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = WavAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "WavComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<WavSnapshot, crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation>(
            crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::STDIO_WAV_DOCUMENT_SCHEMA,
        ));
    }

    /// 💡️ Registers `s.stdio.wav.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::inferences::wav_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

// ⚙️ Wav (riff-pcm) engine — a REAL RIFF/WAVE chunk walker: typed `fmt ` chunk decode/encode,
// typed `data` chunk sample interpretation (`Pcm16`/`Pcm8`/`Float32`/`Raw`), verbatim retention
// of any other RIFF chunk (`LIST`/`INFO`/`fact`/`cue `/…), and a magic sniff. No type sharing
// with `avi` — this walker is wav's own (a shared private helper across the two RIFF-based
// artifacts was considered per the master plan's own allowance, but wav's chunk shape (fixed
// `fmt `+`data` roles) is small enough that duplicating the ~15-line walk loop keeps each
// artifact's engine self-contained without a cross-artifact dependency).

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavData, WavFmt, WavSnapshot, STDIO_WAV_DOCUMENT_SCHEMA};

//#region 🔖️Sniff
/// 🔍 Real magic sniff: `RIFF` fourcc at byte 0 + `WAVE` fourcc at byte 8 (RIFF's own type tag).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE"
}
//#endregion 🔖️Sniff

//#region 🔖️FmtChunk
/// 📐️ Decodes a `fmt ` chunk body (already sliced to exactly `chunk_size` bytes). PCM's plain
/// 16-byte form has no `cbSize`; the extensible/non-PCM form carries a `cbSize` (u16) at byte 16
/// followed by `cbSize` bytes of extension data, retained verbatim in `WavFmt::ext`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_fmt_chunk(body: &[u8]) -> Result<WavFmt, String> {
    if body.len() < 16 {
        return Err(format!("wav: fmt chunk too short ({} bytes)", body.len()));
    }
    let audio_format = u16::from_le_bytes([body[0], body[1]]);
    let channels = u16::from_le_bytes([body[2], body[3]]);
    let sample_rate = u32::from_le_bytes([body[4], body[5], body[6], body[7]]);
    let byte_rate = u32::from_le_bytes([body[8], body[9], body[10], body[11]]);
    let block_align = u16::from_le_bytes([body[12], body[13]]);
    let bits_per_sample = u16::from_le_bytes([body[14], body[15]]);
    let ext = if body.len() > 16 {
        if body.len() < 18 {
            return Err("wav: fmt chunk has trailing bytes but no cbSize".into());
        }
        let cb_size = u16::from_le_bytes([body[16], body[17]]) as usize;
        let end = 18 + cb_size;
        if body.len() < end {
            return Err(format!("wav: fmt cbSize {cb_size} overruns chunk body"));
        }
        Some(body[18..end].to_vec())
    } else {
        None
    };
    Ok(WavFmt { audio_format, channels, sample_rate, byte_rate, block_align, bits_per_sample, ext })
}

/// 📐️ Encodes a `fmt ` chunk body: the plain 16-byte PCM form when `ext` is `None`, else the
/// extensible form (16 bytes + `cbSize`(u16) + `ext` bytes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_fmt_chunk(fmt: &WavFmt) -> Vec<u8> {
    let mut body = Vec::with_capacity(16);
    body.extend_from_slice(&fmt.audio_format.to_le_bytes());
    body.extend_from_slice(&fmt.channels.to_le_bytes());
    body.extend_from_slice(&fmt.sample_rate.to_le_bytes());
    body.extend_from_slice(&fmt.byte_rate.to_le_bytes());
    body.extend_from_slice(&fmt.block_align.to_le_bytes());
    body.extend_from_slice(&fmt.bits_per_sample.to_le_bytes());
    if let Some(ext) = &fmt.ext {
        body.extend_from_slice(&(ext.len() as u16).to_le_bytes());
        body.extend_from_slice(ext);
    }
    body
}
//#endregion 🔖️FmtChunk

//#region 🔖️DataChunk
/// 📐️ Interprets a `data` chunk body against the already-decoded `fmt`: PCM 16-bit → `Pcm16`,
/// PCM 8-bit → `Pcm8` (8-bit PCM is unsigned bytes on the wire — no conversion needed), IEEE
/// float 32-bit → `Float32`; every other `(audio_format, bits_per_sample)` combination (24-bit
/// PCM, ADPCM, WAVE_FORMAT_EXTENSIBLE payloads, …) is retained as `Raw` — an honest boundary,
/// not a silent misinterpretation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_data_chunk(fmt: &WavFmt, body: &[u8]) -> WavData {
    match (fmt.audio_format, fmt.bits_per_sample) {
        (1, 16) if body.len() % 2 == 0 => WavData::Pcm16(body.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]])).collect()),
        (1, 8) => WavData::Pcm8(body.to_vec()),
        (3, 32) if body.len() % 4 == 0 => WavData::Float32(body.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()),
        _ => WavData::Raw(body.to_vec()),
    }
}

/// 📐️ Encodes a `data` chunk body from the typed sample vocabulary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_data_chunk(data: &WavData) -> Vec<u8> {
    match data {
        WavData::Pcm16(samples) => samples.iter().flat_map(|s| s.to_le_bytes()).collect(),
        WavData::Pcm8(bytes) => bytes.clone(),
        WavData::Float32(samples) => samples.iter().flat_map(|s| s.to_le_bytes()).collect(),
        WavData::Raw(bytes) => bytes.clone(),
    }
}
//#endregion 🔖️DataChunk

//#region 🔖️RiffWalk
/// 🚶 Walks every top-level chunk under `RIFF …/WAVE`, routing `fmt `/`data` into their typed
/// slots and retaining everything else (`LIST`/`INFO`/`fact`/`cue `/…) verbatim in
/// `other_chunks`, in on-disk order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_wav(bytes: &[u8]) -> Result<WavSnapshot, String> {
    if !sniff_real_bytes(bytes) {
        return Err("wav: missing RIFF/WAVE magic".into());
    }
    let mut pos = 12usize;
    let mut fmt: Option<WavFmt> = None;
    let mut data: Option<WavData> = None;
    let mut other_chunks = Vec::new();
    // 🪆️ `data` is decoded lazily against `fmt` — real RIFF/WAVE files always place `fmt ` before
    // `data`, but a malformed/reordered file would otherwise silently mis-type; we buffer the raw
    // `data` body until `fmt` is known instead of assuming ordering.
    let mut pending_data_body: Option<Vec<u8>> = None;
    while pos + 8 <= bytes.len() {
        let fourcc = &bytes[pos..pos + 4];
        let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().map_err(|_| "wav: bad chunk size".to_string())?) as usize;
        let body_start = pos + 8;
        let body_end = body_start + size;
        if body_end > bytes.len() {
            return Err(format!("wav: chunk {:?} overruns file ({} > {})", String::from_utf8_lossy(fourcc), body_end, bytes.len()));
        }
        let body = &bytes[body_start..body_end];
        match fourcc {
            b"fmt " => fmt = Some(decode_fmt_chunk(body)?),
            b"data" => pending_data_body = Some(body.to_vec()),
            other => other_chunks.push(RiffChunk { fourcc: String::from_utf8_lossy(other).into_owned(), data: body.to_vec() }),
        }
        pos = body_end + (size % 2); // 🧮️ RIFF chunks are word-aligned: a 1-byte pad after odd-sized bodies.
    }
    let fmt = fmt.ok_or_else(|| "wav: no fmt chunk found".to_string())?;
    if let Some(body) = pending_data_body {
        data = Some(decode_data_chunk(&fmt, &body));
    }
    let data = data.ok_or_else(|| "wav: no data chunk found".to_string())?;
    Ok(WavSnapshot { schema: STDIO_WAV_DOCUMENT_SCHEMA.into(), fmt, data, other_chunks })
}

/// 🚶 Re-encodes a `WavSnapshot` into real RIFF/WAVE bytes: `fmt ` then `data` then
/// `other_chunks` in their stored order — for a snapshot decoded from a real file with no other
/// chunks, this reproduces the original bytes exactly (see `codec_retention_law` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_wav(snapshot: &WavSnapshot) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    let fmt_body = encode_fmt_chunk(&snapshot.fmt);
    body.extend_from_slice(b"fmt ");
    body.extend_from_slice(&(fmt_body.len() as u32).to_le_bytes());
    body.extend_from_slice(&fmt_body);
    if fmt_body.len() % 2 == 1 {
        body.push(0);
    }
    let data_body = encode_data_chunk(&snapshot.data);
    body.extend_from_slice(b"data");
    body.extend_from_slice(&(data_body.len() as u32).to_le_bytes());
    body.extend_from_slice(&data_body);
    if data_body.len() % 2 == 1 {
        body.push(0);
    }
    for chunk in &snapshot.other_chunks {
        let mut fourcc = chunk.fourcc.clone().into_bytes();
        fourcc.resize(4, b' ');
        body.extend_from_slice(&fourcc[0..4]);
        body.extend_from_slice(&(chunk.data.len() as u32).to_le_bytes());
        body.extend_from_slice(&chunk.data);
        if chunk.data.len() % 2 == 1 {
            body.push(0);
        }
    }
    let mut out = Vec::with_capacity(8 + body.len());
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}
//#endregion 🔖️RiffWalk

#[cfg(test)]
mod codec_tests {
    use super::*;

    /// 🌱 Real ~1s 440Hz mono 8kHz 16-bit PCM fixture — byte-identical to the artifact's own
    /// `📚️examples/🎬️demo/🖼️assets/🔊️example.wav` (per ticket `fixtures/wav/NOTES.md`), duplicated
    /// here as a literal so the test doesn't reach across an emoji-path `include_bytes!` boundary.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn real_fixture() -> Vec<u8> {
        include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav").to_vec()
    }

    #[semio_framework_async_macros::async_test]
    async fn sniffs_and_decodes_a_synthetic_fmt_chunk() {
        let snap = WavSnapshot { fmt: WavFmt { audio_format: 1, channels: 1, sample_rate: 8000, byte_rate: 16000, block_align: 2, bits_per_sample: 16, ext: None }, data: WavData::Pcm16(vec![0, 100, -100]), ..WavSnapshot::default() };
        let bytes = encode_wav(&snap);
        assert!(sniff_real_bytes(&bytes));
        let decoded = decode_wav(&bytes).expect("decode");
        assert_eq!(decoded.fmt, snap.fmt);
        assert_eq!(decoded.data, snap.data);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_rejects_non_wave_riff() {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend_from_slice(&4u32.to_le_bytes());
        bytes.extend_from_slice(b"AVI ");
        assert!(!sniff_real_bytes(&bytes));
    }

    //#region codec_retention_law
    /// 🧪️ `codec_retention_law`: decoding the REAL fixture, re-encoding, and decoding again must
    /// be byte-exact at every level — the on-disk bytes, the recovered PCM samples, AND (an
    /// independent confirmation, not reusing the decoder's own sample array) a freshly
    /// re-synthesized 440Hz reference tone, per `fixtures/wav/NOTES.md`'s own verification
    /// method.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let fixture = real_fixture();
        let decoded = decode_wav(&fixture).expect("decode real fixture");
        assert_eq!(decoded.fmt.audio_format, 1, "PCM");
        assert_eq!(decoded.fmt.channels, 1, "mono");
        assert_eq!(decoded.fmt.sample_rate, 8000);
        assert_eq!(decoded.fmt.byte_rate, 16000);
        assert_eq!(decoded.fmt.block_align, 2);
        assert_eq!(decoded.fmt.bits_per_sample, 16);
        assert_eq!(decoded.fmt.ext, None);
        assert!(decoded.other_chunks.is_empty(), "fixture has only fmt + data");

        let samples = match &decoded.data {
            WavData::Pcm16(s) => s.clone(),
            other => panic!("expected Pcm16, got {other:?}"),
        };
        assert_eq!(samples.len(), 8000, "1.0s at 8000Hz");

        // 🔬️ Independent re-synthesis (not reusing the writer's array): max abs diff must be 0.
        // `make_wav.py`'s own `max_amp = int(AMPLITUDE * 32767)` truncates to an integer BEFORE
        // multiplying by `sin(t)` — reproduced here bit-for-bit (not `0.5 * 32767.0` as a
        // continuous `f64`, which rounds a peak sample to 16384 instead of the fixture's 16383).
        let max_amp = (0.5 * 32767.0) as i32 as f64;
        let mut max_abs_diff = 0i32;
        for (n, &sample) in samples.iter().enumerate() {
            let reference = ((2.0 * std::f64::consts::PI * 440.0 * n as f64 / 8000.0).sin() * max_amp).round() as i32;
            max_abs_diff = max_abs_diff.max((sample as i32 - reference).abs());
        }
        assert_eq!(max_abs_diff, 0, "decoded samples must exactly match a freshly re-synthesized 440Hz sine");

        // 🔁️ Re-encode must reproduce the real fixture byte-for-byte (no other_chunks, no ext).
        let re_encoded = encode_wav(&decoded);
        assert_eq!(re_encoded, fixture, "encode(decode(real fixture)) must be byte-identical");

        // 🔁️ Second round trip (decode the re-encoded bytes) must also match exactly.
        let re_decoded = decode_wav(&re_encoded).expect("decode re-encoded");
        assert_eq!(re_decoded, decoded);
    }
    //#endregion codec_retention_law

    //#region 🔖️OtherChunksRetention
    #[semio_framework_async_macros::async_test]
    async fn other_chunks_round_trip_verbatim_in_order() {
        let snap = WavSnapshot {
            fmt: WavFmt { audio_format: 1, channels: 1, sample_rate: 44100, byte_rate: 88200, block_align: 2, bits_per_sample: 16, ext: None },
            data: WavData::Pcm16(vec![1, 2, 3]),
            other_chunks: vec![
                RiffChunk { fourcc: "fact".into(), data: vec![0x03, 0x00, 0x00, 0x00] },
                RiffChunk { fourcc: "LIST".into(), data: b"INFOICRDodd".to_vec() }, // 🧮️ odd length exercises pad-byte handling
            ],
            ..WavSnapshot::default()
        };
        let bytes = encode_wav(&snap);
        let decoded = decode_wav(&bytes).expect("decode");
        assert_eq!(decoded.other_chunks, snap.other_chunks);
        assert_eq!(decoded.other_chunks[0].fourcc, "fact");
        assert_eq!(decoded.other_chunks[1].fourcc, "LIST");
    }
    //#endregion 🔖️OtherChunksRetention

    //#region 🔖️ExtFmtRetention
    #[semio_framework_async_macros::async_test]
    async fn extensible_fmt_chunk_round_trips_ext_bytes() {
        let snap = WavSnapshot {
            fmt: WavFmt {
                audio_format: 0xFFFE,
                channels: 2,
                sample_rate: 48000,
                byte_rate: 192000,
                block_align: 4,
                bits_per_sample: 16,
                ext: Some(vec![0x16, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71]),
            },
            data: WavData::Raw(vec![0xAA, 0xBB]),
            ..WavSnapshot::default()
        };
        let bytes = encode_wav(&snap);
        let decoded = decode_wav(&bytes).expect("decode");
        assert_eq!(decoded.fmt, snap.fmt);
        assert_eq!(decoded.data, snap.data);
    }
    //#endregion 🔖️ExtFmtRetention
}
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::WavComposer as WavRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<WavRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
