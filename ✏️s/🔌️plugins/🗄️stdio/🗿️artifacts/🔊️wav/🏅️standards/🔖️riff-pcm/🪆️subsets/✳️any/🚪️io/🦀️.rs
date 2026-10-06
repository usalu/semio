//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
    use crate::standards::riff_pcm::subsets::any::io::WavAnalyzer;
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
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor()).expect("schema descriptor publication");
        register_artifact_inferences();
        semio_framework_plugin::io::register_native_document_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.wav", standard: semio_framework_plugin::StandardId("riff-pcm"), subset: semio_framework_plugin::SubsetId("*") }, store::ArtifactCodec::bare::<WavSnapshot, crate::standards::riff_pcm::subsets::any::schema::mutations::WavMutation>(crate::standards::riff_pcm::subsets::any::schema::snapshot::STDIO_WAV_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
    }

    /// 💡️ Registers `s.stdio.wav.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::riff_pcm::subsets::any::schema::inferences::wav_artifact_inference_descriptor()).expect("schema descriptor publication");
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

use crate::standards::riff_pcm::subsets::any::schema::snapshot::{validate_wav_serialization, RiffChunk, WavChunkRef, WavData, WavFmt, WavSnapshot, STDIO_WAV_DOCUMENT_SCHEMA};

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
fn encode_fmt_chunk(fmt: &WavFmt) -> Result<Vec<u8>, String> {
    let mut body = Vec::with_capacity(16);
    body.extend_from_slice(&fmt.audio_format.to_le_bytes());
    body.extend_from_slice(&fmt.channels.to_le_bytes());
    body.extend_from_slice(&fmt.sample_rate.to_le_bytes());
    body.extend_from_slice(&fmt.byte_rate.to_le_bytes());
    body.extend_from_slice(&fmt.block_align.to_le_bytes());
    body.extend_from_slice(&fmt.bits_per_sample.to_le_bytes());
    if let Some(ext) = &fmt.ext {
        let cb_size = u16::try_from(ext.len()).map_err(|_| format!("wav: fmt.ext contains {} bytes; cbSize is u16", ext.len()))?;
        body.extend_from_slice(&cb_size.to_le_bytes());
        body.extend_from_slice(ext);
    }
    Ok(body)
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
        (1, 16) if body.len().is_multiple_of(2) => WavData::Pcm16(body.as_chunks::<2>().0.iter().map(|c| i16::from_le_bytes([c[0], c[1]])).collect()),
        (1, 8) => WavData::Pcm8(body.to_vec()),
        (3, 32) if body.len().is_multiple_of(4) => WavData::Float32(body.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()),
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
/// 🚶 Walks every top-level chunk under `RIFF …/WAVE`. The first `fmt ` and `data` chunks
/// become the typed primary values. Every other chunk, including duplicate `fmt `/`data` chunks,
/// remains verbatim in `other_chunks`; `chunk_order` records the complete on-disk sequence.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_wav(bytes: &[u8]) -> Result<WavSnapshot, String> {
    if !sniff_real_bytes(bytes) {
        return Err("wav: missing RIFF/WAVE magic".into());
    }
    let mut pos = 12usize;
    let mut fmt: Option<WavFmt> = None;
    let mut data: Option<WavData> = None;
    let mut fmt_pad_byte = 0;
    let mut data_pad_byte = 0;
    let mut other_chunks = Vec::new();
    let mut chunk_order = Vec::new();
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
        let pad_byte = if size % 2 == 1 { *bytes.get(body_end).ok_or_else(|| format!("wav: odd chunk {:?} is missing its pad byte", String::from_utf8_lossy(fourcc)))? } else { 0 };
        match fourcc {
            b"fmt " if fmt.is_none() => {
                fmt = Some(decode_fmt_chunk(body)?);
                fmt_pad_byte = pad_byte;
                chunk_order.push(WavChunkRef::Format);
            }
            b"data" if pending_data_body.is_none() => {
                pending_data_body = Some(body.to_vec());
                data_pad_byte = pad_byte;
                chunk_order.push(WavChunkRef::Samples);
            }
            other => {
                let index = other_chunks.len() as u64;
                other_chunks.push(RiffChunk { fourcc: String::from_utf8_lossy(other).into_owned(), data: body.to_vec(), pad_byte });
                chunk_order.push(WavChunkRef::Other(index));
            }
        }
        pos = body_end + (size % 2); // 🧮️ RIFF chunks are word-aligned: a 1-byte pad after odd-sized bodies.
    }
    let fmt = fmt.ok_or_else(|| "wav: no fmt chunk found".to_string())?;
    if let Some(body) = pending_data_body {
        data = Some(decode_data_chunk(&fmt, &body));
    }
    let data = data.ok_or_else(|| "wav: no data chunk found".to_string())?;
    Ok(WavSnapshot { schema: STDIO_WAV_DOCUMENT_SCHEMA.into(), fmt, data, fmt_pad_byte, data_pad_byte, other_chunks, chunk_order })
}

fn append_chunk(body: &mut Vec<u8>, fourcc: &[u8; 4], payload: &[u8], pad_byte: u8) {
    body.extend_from_slice(fourcc);
    body.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    body.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        body.push(pad_byte);
    }
}

/// 🚶 Re-encodes a `WavSnapshot` in `chunk_order`. Missing primary or unreferenced auxiliary
/// chunks are appended once so direct schema edits cannot accidentally discard payloads. A typed
/// primary is emitted before a verbatim duplicate with the same fourcc, keeping primary edits
/// authoritative even when a hand-edited sequence omits or misorders its primary reference.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn try_encode_wav(snapshot: &WavSnapshot) -> Result<Vec<u8>, String> {
    validate_wav_serialization(snapshot).map_err(|issue| format!("{}: {}", issue.code, issue.message))?;
    let mut body = Vec::new();
    body.extend_from_slice(b"WAVE");
    let fmt_body = encode_fmt_chunk(&snapshot.fmt)?;
    let data_body = encode_data_chunk(&snapshot.data);
    let mut format_emitted = false;
    let mut samples_emitted = false;
    let mut other_emitted = vec![false; snapshot.other_chunks.len()];
    for reference in &snapshot.chunk_order {
        match reference {
            WavChunkRef::Format => {
                if format_emitted { continue; }
                append_chunk(&mut body, b"fmt ", &fmt_body, snapshot.fmt_pad_byte);
                format_emitted = true;
            }
            WavChunkRef::Samples => {
                if samples_emitted { continue; }
                append_chunk(&mut body, b"data", &data_body, snapshot.data_pad_byte);
                samples_emitted = true;
            }
            WavChunkRef::Other(index) => {
                let Ok(index) = usize::try_from(*index) else { continue };
                let Some(chunk) = snapshot.other_chunks.get(index) else { continue };
                if other_emitted[index] { continue; }
                if chunk.fourcc.as_bytes() == b"fmt " && !format_emitted {
                    append_chunk(&mut body, b"fmt ", &fmt_body, snapshot.fmt_pad_byte);
                    format_emitted = true;
                }
                if chunk.fourcc.as_bytes() == b"data" && !samples_emitted {
                    append_chunk(&mut body, b"data", &data_body, snapshot.data_pad_byte);
                    samples_emitted = true;
                }
                let mut fourcc = chunk.fourcc.clone().into_bytes();
                fourcc.resize(4, b' ');
                append_chunk(&mut body, fourcc[0..4].try_into().expect("fourcc resized to four bytes"), &chunk.data, chunk.pad_byte);
                other_emitted[index] = true;
            }
        }
    }
    if !format_emitted { append_chunk(&mut body, b"fmt ", &fmt_body, snapshot.fmt_pad_byte); }
    if !samples_emitted { append_chunk(&mut body, b"data", &data_body, snapshot.data_pad_byte); }
    for (index, chunk) in snapshot.other_chunks.iter().enumerate() {
        if other_emitted[index] { continue; }
        let mut fourcc = chunk.fourcc.clone().into_bytes();
        fourcc.resize(4, b' ');
        append_chunk(&mut body, fourcc[0..4].try_into().expect("fourcc resized to four bytes"), &chunk.data, chunk.pad_byte);
    }
    let mut out = Vec::with_capacity(8 + body.len());
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    Ok(out)
}

/// 🚶 Re-encodes an already validated WAV snapshot for infallible framework call sites.
pub fn encode_wav(snapshot: &WavSnapshot) -> Vec<u8> {
    try_encode_wav(snapshot).expect("WAV snapshot must be exactly representable before encoding")
}
//#endregion 🔖️RiffWalk

#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::riff_pcm::subsets::any::io::WavComposer as WavRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<WavRawAnyComposer>()]).as_slice()
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
    use crate::standards::riff_pcm::subsets::any::schema::diff::WavDiff;
    use crate::standards::riff_pcm::subsets::any::schema::mutations::{apply_wav_mutation, WavMutation};
    use crate::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct WavBuilderConstruction {
        snapshot: WavSnapshot,
    }

    impl ArtifactBuilder for WavBuilderConstruction {
        type Snapshot = WavSnapshot;
        type Mutation = WavMutation;
        type Diff = WavDiff;
        fn empty() -> Self {
            Self { snapshot: WavSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<WavSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<WavSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_wav_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <WavDiff as protocol::MutationDiff<WavSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::riff_pcm::subsets::any::io;
    use crate::standards::riff_pcm::subsets::any::schema::snapshot::{WavSnapshot, STDIO_WAV_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct WavParts {
        pub snapshot: Option<WavSnapshot>,
    }

    pub struct WavAnalyzerAnalysis;

    impl ArtifactAnalysis for WavAnalyzerAnalysis {
        type Parts = WavParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_WAV_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_WAV_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = WavParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <WavSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <WavSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
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
    pub spec WavBuilderFacets {
        construction: WavBuilderConstruction,
        analysis: WavAnalyzerAnalysis,
        composition: crate::standards::riff_pcm::subsets::any::io::derived_composition::WavComposerComposition,
    }
    builder: WavBuilder,
    analyzer: WavAnalyzer,
    composer: WavComposer,
);
