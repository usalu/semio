//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
    use crate::standards::mpeg1_layer3::subsets::any::io::Mp3Analyzer;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Mp3ComposerComposition;

    impl ArtifactComposition for Mp3ComposerComposition {
        type Snapshot = Mp3Snapshot;
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
                return Err(ComposeError { message: "Mp3ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = Mp3Analyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "Mp3ComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::mpeg1_layer3::subsets::any::schema::mp3_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.mp3", standard: semio_framework_artifact_reference::StandardId("mpeg1-layer3"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::bare::<Mp3Snapshot, crate::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation>(crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::STDIO_MP3_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.mp3.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::mpeg1_layer3::subsets::any::schema::inferences::mp3_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Sniff
use crate::apply_mutation;
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Content, Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader, Mp3Snapshot, STDIO_MP3_DOCUMENT_SCHEMA};

/// 🔍 Real magic sniff: an ID3v2 header at the front, OR a valid MPEG frame sync anywhere in the
/// buffer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    bytes.starts_with(b"ID3") || find_frame_sync(bytes).is_some()
}
//#endregion 🔖️Sniff

//#region 🔖️Syncsafe
#[path = "🏷️metadata/🦀️.rs"]
pub mod metadata;
use metadata::{encode_syncsafe,encode_id3v2,encode_id3v1,decode_id3v1,id3_frame_body_len,id3_frame_body_slice};

//#region 🔖️FrameHeader
/// 🔍 Real 11-bit MPEG sync-word scan: `0xFFE` in the top 11 bits, plus a sanity check that the
/// version (bits 19-20) and layer (bits 17-18) fields are not the reserved values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_frame_sync(bytes: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    while i + 1 < bytes.len() {
        if bytes[i] == 0xFF && (bytes[i + 1] & 0xE0) == 0xE0 {
            let version = (bytes[i + 1] >> 3) & 0x03;
            let layer = (bytes[i + 1] >> 1) & 0x03;
            if version != 0x01 && layer != 0x00 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// 📐️ MPEG1/2/2.5 bitrate table (kbps), keyed by `(version_id, layer, index)`. `version_id`:
/// `0`=2.5, `2`=2, `3`=1 (`1` is the reserved value, never reached here). `layer`: `1`=III,
/// `2`=II, `3`=I. Index `0` = "free" bitrate (unsupported — no frame-size formula applies) and
/// `15` = reserved; both are honest decode failures, not silently substituted.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bitrate_kbps(version_id: u8, layer: u8, index: u8) -> Option<u16> {
    const V1_L1: [u16; 16] = [0, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 0];
    const V1_L2: [u16; 16] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384, 0];
    const V1_L3: [u16; 16] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0];
    const V2_L1: [u16; 16] = [0, 32, 48, 56, 64, 80, 96, 112, 128, 144, 160, 176, 192, 224, 256, 0];
    const V2_L23: [u16; 16] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160, 0];
    if index == 0 || index == 15 {
        return None;
    }
    let table = match (version_id, layer) {
        (3, 3) => &V1_L1,
        (3, 2) => &V1_L2,
        (3, 1) => &V1_L3,
        (0 | 2, 3) => &V2_L1,
        (0 | 2, 2 | 1) => &V2_L23,
        _ => return None,
    };
    Some(table[index as usize])
}

/// 📐️ Parses the 4-byte frame header at `bytes[pos..pos+4]` and computes the real total frame
/// size (header + payload, per Layer I/II/III's own formula). Returns `None` on any reserved
/// field or a size that would overrun the buffer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_frame_header(bytes: &[u8], pos: usize) -> Option<(Mp3FrameHeader, usize)> {
    if pos + 4 > bytes.len() {
        return None;
    }
    let b1 = bytes[pos + 1];
    let b2 = bytes[pos + 2];
    let b3 = bytes[pos + 3];
    let mpeg_version_id = (b1 >> 3) & 0x03;
    let layer = (b1 >> 1) & 0x03;
    if mpeg_version_id == 0x01 || layer == 0x00 {
        return None; // reserved
    }
    let protection_bit = (b1 & 0x01) != 0;
    let bitrate_index = (b2 >> 4) & 0x0F;
    let sample_rate_index = (b2 >> 2) & 0x03;
    let padding = ((b2 >> 1) & 0x01) != 0;
    let private_bit = (b2 & 0x01) != 0;
    let channel_mode = (b3 >> 6) & 0x03;
    let mode_extension = (b3 >> 4) & 0x03;
    let copyright = ((b3 >> 3) & 0x01) != 0;
    let original = ((b3 >> 2) & 0x01) != 0;
    let emphasis = b3 & 0x03;

    let bitrate_bps = bitrate_kbps(mpeg_version_id, layer, bitrate_index)? as u32 * 1000;
    let sample_rate = crate::standards::mpeg1_layer3::subsets::any::schema::sample_rate_hz(mpeg_version_id, sample_rate_index)?;
    let pad = if padding { 1u32 } else { 0 };
    let frame_size = if layer == 3 {
        // Layer I: slots are 4 bytes.
        (12 * bitrate_bps / sample_rate + pad) * 4
    } else {
        // Layer II/III: slots are 1 byte.
        144 * bitrate_bps / sample_rate + pad
    } as usize;
    if frame_size < 4 || pos + frame_size > bytes.len() {
        return None;
    }
    Some((Mp3FrameHeader { mpeg_version_id, layer, protection_bit, bitrate_index, sample_rate_index, padding, private_bit, channel_mode, mode_extension, copyright, original, emphasis }, frame_size))
}

/// 📐️ Re-encodes a frame header's typed fields back to the real 4 header bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_frame_header(h: &Mp3FrameHeader) -> [u8; 4] {
    let b0 = 0xFFu8;
    let b1 = 0xE0 | (h.mpeg_version_id << 3) | (h.layer << 1) | (h.protection_bit as u8);
    let b2 = (h.bitrate_index << 4) | (h.sample_rate_index << 2) | ((h.padding as u8) << 1) | (h.private_bit as u8);
    let b3 = (h.channel_mode << 6) | (h.mode_extension << 4) | ((h.copyright as u8) << 3) | ((h.original as u8) << 2) | h.emphasis;
    [b0, b1, b2, b3]
}
//#endregion 🔖️FrameHeader

//#region 🔖️Codec
/// 🚶 Decodes a full `.mp3` byte stream: optional leading ID3v2 tag, a sequence of real MPEG
/// frames (sync-scanned + header-decoded + sized by the real bitrate/sample-rate formula), and
/// an optional trailing 128-byte ID3v1 tag (`TAG` magic).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_mp3(bytes: &[u8]) -> Result<Mp3Snapshot, String> {
    let mut pos = 0usize;
    let id3v2 = if bytes.starts_with(b"ID3") {let(tag,consumed)=metadata::decode_id3v2(bytes)?;pos=consumed;Some(tag)} else {None};

    let mut frames = Vec::new();
    while pos < bytes.len() {
        if bytes.len()-pos==128&&bytes[pos..].starts_with(b"TAG"){break;}
        let Some(offset)=find_frame_sync(&bytes[pos..])else{break;};
        if offset!=0{return Err("mp3.unadmitted-prefix".into());}
        let frame_pos = pos + offset;
        let Some((header, frame_size)) = parse_frame_header(bytes, frame_pos) else { break };
        let payload = bytes[frame_pos + 4..frame_pos + frame_size].to_vec();
        frames.push(Mp3Frame { header, payload });
        pos = frame_pos + frame_size;
    }

    let id3v1 = if bytes.len() - pos == 128 && &bytes[pos..pos + 3] == b"TAG" {
        let tag = decode_id3v1(&bytes[pos..pos + 128])?;
        pos += 128;
        Some(tag)
    } else {
        None
    };
    if pos!=bytes.len(){return Err("mp3.unadmitted-tail".into());}

    Ok(Mp3Snapshot { schema: STDIO_MP3_DOCUMENT_SCHEMA.into(), id3v2, frames, id3v1 })
}

/// 🚶 Re-encodes a `Mp3Snapshot` to real bytes: `id3v2` (if present) + every frame's header
/// (reconstructed from typed fields) + its retained payload + `id3v1` (if present) — for a
/// snapshot decoded from a real file, this reproduces the original bytes exactly (see
/// `codec_retention_law` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_mp3(snapshot: &Mp3Snapshot) -> Result<Vec<u8>,String> {
    let mut out = Vec::new();
    if let Some(tag) = &snapshot.id3v2 {
        out.extend_from_slice(&encode_id3v2(tag)?);
    }
    for frame in &snapshot.frames {
        out.extend_from_slice(&encode_frame_header(&frame.header));
        out.extend_from_slice(&frame.payload);
    }
    if let Some(tag) = &snapshot.id3v1 {
        out.extend_from_slice(&encode_id3v1(tag)?);
    }
    Ok(out)
}

/// 🧵️ One bounded advance of the native MP3 serializer.
#[derive(Debug, PartialEq, Eq)]
pub enum Mp3EncodeAdvance {
    Progress,
    Chunk(Vec<u8>),
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mp3EncodePhase {
    MeasureId3v2,
    Id3v2Header,
    Id3v2FrameHeader,
    Id3v2FrameData,
    AudioFrameHeader,
    AudioFrameData,
    Id3v1,
    Complete,
}

/// 🎚️ Incrementally serializes retained MP3 structure without materializing the encoded file.
pub struct Mp3EncodeCursor {
    phase: Mp3EncodePhase,
    index: usize,
    offset: usize,
    id3v2_body_bytes: usize,
    emitted_bytes: u64,
}

impl Mp3EncodeCursor {
    pub fn new(snapshot: &Mp3Snapshot) -> Self {
        let phase = if snapshot.id3v2.is_some() { Mp3EncodePhase::MeasureId3v2 } else if snapshot.frames.is_empty() { Mp3EncodePhase::Id3v1 } else { Mp3EncodePhase::AudioFrameHeader };
        Self { phase, index: 0, offset: 0, id3v2_body_bytes: 0, emitted_bytes: 0 }
    }

    pub fn emitted_bytes(&self) -> u64 {
        self.emitted_bytes
    }

    pub fn advance(&mut self, snapshot: &Mp3Snapshot, maximum_bytes: usize) -> Result<Mp3EncodeAdvance, String> {
        if maximum_bytes == 0 {
            return Err("mp3.encode.zero-byte-grant".into());
        }
        loop {
            match self.phase {
                Mp3EncodePhase::MeasureId3v2 => {
                    let tag = snapshot.id3v2.as_ref().ok_or("mp3.encode.id3v2-owner-missing")?;
                    if let Some(frame) = tag.frames.get(self.index) {
                        let body_bytes = id3_frame_body_len(frame)?;
                        u32::try_from(body_bytes).map_err(|_| "mp3.encode.id3v2-frame-too-large")?;
                        self.id3v2_body_bytes = self.id3v2_body_bytes.checked_add(10).and_then(|bytes| bytes.checked_add(body_bytes)).ok_or("mp3.encode.id3v2-size-overflow")?;
                        self.index += 1;
                        return Ok(Mp3EncodeAdvance::Progress);
                    }
                    if self.id3v2_body_bytes >= 1 << 28 {
                        return Err("mp3.encode.id3v2-body-too-large".into());
                    }
                    self.phase = Mp3EncodePhase::Id3v2Header;
                    self.index = 0;
                    self.offset = 0;
                    return Ok(Mp3EncodeAdvance::Progress);
                }
                Mp3EncodePhase::Id3v2Header => {
                    let tag = snapshot.id3v2.as_ref().ok_or("mp3.encode.id3v2-owner-missing")?;
                    let size = encode_syncsafe(self.id3v2_body_bytes as u32);
                    let bytes = [b'I', b'D', b'3', 4, 0, 0, size[0], size[1], size[2], size[3]];
                    let chunk = self.take_slice(&bytes, maximum_bytes);
                    if self.offset == bytes.len() {
                        self.phase = if tag.frames.is_empty() { self.phase_after_id3v2(snapshot) } else { Mp3EncodePhase::Id3v2FrameHeader };
                        self.offset = 0;
                    }
                    return Ok(Mp3EncodeAdvance::Chunk(chunk));
                }
                Mp3EncodePhase::Id3v2FrameHeader => {
                    let tag = snapshot.id3v2.as_ref().ok_or("mp3.encode.id3v2-owner-missing")?;
                    let frame = tag.frames.get(self.index).ok_or("mp3.encode.id3v2-frame-missing")?;
                    let mut bytes = [0u8; 10];
                    let id = frame.id.as_bytes();
                    let id_bytes = id.len().min(4);
                    bytes[..id_bytes].copy_from_slice(&id[..id_bytes]);
                    let size = u32::try_from(id3_frame_body_len(frame)?).map_err(|_| "mp3.encode.id3v2-frame-too-large")?;
                    let encoded_size = encode_syncsafe(size);
                    bytes[4..8].copy_from_slice(&encoded_size);
                    bytes[8..10].copy_from_slice(&[0,0]);
                    let chunk = self.take_slice(&bytes, maximum_bytes);
                    if self.offset == bytes.len() {
                        self.phase = if id3_frame_body_len(frame)? == 0 { self.advance_id3v2_frame(snapshot) } else { Mp3EncodePhase::Id3v2FrameData };
                        self.offset = 0;
                    }
                    return Ok(Mp3EncodeAdvance::Chunk(chunk));
                }
                Mp3EncodePhase::Id3v2FrameData => {
                    let frame = snapshot.id3v2.as_ref().and_then(|tag| tag.frames.get(self.index)).ok_or("mp3.encode.id3v2-frame-missing")?;
                    let chunk = id3_frame_body_slice(frame,self.offset,maximum_bytes)?;
                    self.offset += chunk.len();
                    self.emitted_bytes += chunk.len() as u64;
                    if self.offset == id3_frame_body_len(frame)? {
                        self.phase = self.advance_id3v2_frame(snapshot);
                        self.offset = 0;
                    }
                    return Ok(Mp3EncodeAdvance::Chunk(chunk));
                }
                Mp3EncodePhase::AudioFrameHeader => {
                    let frame = snapshot.frames.get(self.index).ok_or("mp3.encode.audio-frame-missing")?;
                    let bytes = encode_frame_header(&frame.header);
                    let chunk = self.take_slice(&bytes, maximum_bytes);
                    if self.offset == bytes.len() {
                        self.phase = if frame.payload.is_empty() { self.advance_audio_frame(snapshot) } else { Mp3EncodePhase::AudioFrameData };
                        self.offset = 0;
                    }
                    return Ok(Mp3EncodeAdvance::Chunk(chunk));
                }
                Mp3EncodePhase::AudioFrameData => {
                    let frame = snapshot.frames.get(self.index).ok_or("mp3.encode.audio-frame-missing")?;
                    let chunk = self.take_slice(&frame.payload, maximum_bytes);
                    if self.offset == frame.payload.len() {
                        self.phase = self.advance_audio_frame(snapshot);
                        self.offset = 0;
                    }
                    return Ok(Mp3EncodeAdvance::Chunk(chunk));
                }
                Mp3EncodePhase::Id3v1 => {
                    let Some(tag) = snapshot.id3v1.as_ref() else {
                        self.phase = Mp3EncodePhase::Complete;
                        continue;
                    };
                    let body = encode_id3v1(tag)?;
                    let chunk = self.take_slice(&body, maximum_bytes);
                    if self.offset == 128 {
                        self.phase = Mp3EncodePhase::Complete;
                        self.offset = 0;
                    }
                    if chunk.is_empty() {
                        continue;
                    }
                    return Ok(Mp3EncodeAdvance::Chunk(chunk));
                }
                Mp3EncodePhase::Complete => return Ok(Mp3EncodeAdvance::Complete),
            }
        }
    }

    fn take_slice(&mut self, bytes: &[u8], maximum_bytes: usize) -> Vec<u8> {
        let end = self.offset.saturating_add(maximum_bytes).min(bytes.len());
        let chunk = bytes[self.offset..end].to_vec();
        self.offset = end;
        self.emitted_bytes += chunk.len() as u64;
        chunk
    }

    fn phase_after_id3v2(&mut self, snapshot: &Mp3Snapshot) -> Mp3EncodePhase {
        self.index = 0;
        if snapshot.frames.is_empty() { Mp3EncodePhase::Id3v1 } else { Mp3EncodePhase::AudioFrameHeader }
    }

    fn advance_id3v2_frame(&mut self, snapshot: &Mp3Snapshot) -> Mp3EncodePhase {
        self.index += 1;
        let frame_count = snapshot.id3v2.as_ref().map_or(0, |tag| tag.frames.len());
        if self.index < frame_count { Mp3EncodePhase::Id3v2FrameHeader } else { self.phase_after_id3v2(snapshot) }
    }

    fn advance_audio_frame(&mut self, snapshot: &Mp3Snapshot) -> Mp3EncodePhase {
        self.index += 1;
        if self.index < snapshot.frames.len() { Mp3EncodePhase::AudioFrameHeader } else { self.index = 0; Mp3EncodePhase::Id3v1 }
    }
}

pub mod playback {
    use super::{Mp3EncodeAdvance, Mp3EncodeCursor, Mp3Snapshot, STDIO_MP3_DOCUMENT_SCHEMA};
    use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
    use semio_framework_job::Operation;
    use semio_framework_plugin::app::ArtifactOutputChunks;
    use semio_framework_plugin::{ArtifactApp, ArtifactOwnedToolJobFactory, ArtifactReservedToolJob, ArtifactSnapshotDisposer, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Fault, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, PortMultiplicity};
    use semio_s_artifact_stdio_contract::media_export::{IncrementalMediaAdvance, IncrementalMediaExportJob, IncrementalMediaExportSpec};
    use std::marker::PhantomData;
    use std::sync::Arc;

    pub const PORT_ID: &str = "playback:out";
    pub const TOOL_ID: &str = "export-media:playback:out";
    pub const PAYLOAD_SCHEMA: &str = "stdio.mp3.playback-export.v1";
    pub const MEDIA_SCHEMA: &str = "stdio.mp3";
    pub const MIME_TYPE: &str = "audio/mpeg";
    pub const MEDIA_TYPE: MediaType = MediaType { class: MediaClass::Presentation, form: MediaForm::Sequence };
    pub const CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(4_096, 4_096, 1, ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, 2_000, 64, 1);

    pub fn app_io() -> semio_framework_plugin::AppIo {
        semio_framework_plugin::AppIo {
            artifact_schema: STDIO_MP3_DOCUMENT_SCHEMA.into(),
            artifact_media_type: MEDIA_TYPE,
            ports: vec![MediaPortSpec {
                id: PORT_ID.into(),
                label: "Playback".into(),
                direction: MediaPortDirection::Out,
                media_type: MEDIA_TYPE,
                kind_id: Some("s.stdio.mp3".into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            }],
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact: semio_framework_plugin::ArtifactPresentation { id: "stdio.mp3".into(), name: "MP3 Audio".into(), dimension: "time".into(), component_kind: "audio".into() },
        }
    }

    pub struct Mp3PlaybackExport;
    impl IncrementalMediaExportSpec for Mp3PlaybackExport {
        type Snapshot = Mp3Snapshot;
        type Cursor = Mp3EncodeCursor;
        const DOCUMENT_SCHEMA: &'static str = STDIO_MP3_DOCUMENT_SCHEMA;
        const MEDIA_SCHEMA: &'static str = MEDIA_SCHEMA;
        const MIME_TYPE: &'static str = MIME_TYPE;
        const PAYLOAD_SCHEMA: &'static str = PAYLOAD_SCHEMA;
        const STAGE: &'static str = "encode-mp3";
        const KIND_ID: &'static str = "s.stdio.mp3";
        const ARTIFACT_ID: &'static str = "stdio.mp3";
        const ARTIFACT_NAME: &'static str = "MP3 Audio";
        const COMPONENT_KIND: &'static str = "audio";
        fn cursor(snapshot: &Mp3Snapshot) -> Result<Mp3EncodeCursor, Fault> { Ok(Mp3EncodeCursor::new(snapshot)) }
        fn advance(cursor: &mut Mp3EncodeCursor, snapshot: &Mp3Snapshot, maximum_bytes: usize) -> Result<IncrementalMediaAdvance, Fault> {
            cursor.advance(snapshot, maximum_bytes).map(|advance| match advance {
                Mp3EncodeAdvance::Progress => IncrementalMediaAdvance::Progress,
                Mp3EncodeAdvance::Chunk(bytes) => IncrementalMediaAdvance::Chunk(bytes),
                Mp3EncodeAdvance::Complete => IncrementalMediaAdvance::Complete,
            }).map_err(Fault::from)
        }
    }
    pub type Mp3PlaybackExportJob = IncrementalMediaExportJob<Mp3PlaybackExport>;

    pub struct Mp3MediaExportJobFactory<A: ArtifactApp<Snapshot = Mp3Snapshot>> {
        keys: [ToolFactoryKey; 1],
        owner: PhantomData<fn() -> A>,
    }

    impl<A: ArtifactApp<Snapshot = Mp3Snapshot>> Mp3MediaExportJobFactory<A> {
        pub fn new(controller: &str) -> Self {
            Self { keys: [ToolFactoryKey::new(controller, TOOL_ID)], owner: PhantomData }
        }
    }

    impl<A: ArtifactApp<Snapshot = Mp3Snapshot>> ToolJobFactory for Mp3MediaExportJobFactory<A> {
        type Payload = ArtifactReservedToolJob;
        type Job = ArtifactReservedToolJob;

        fn keys(&self) -> &[ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            PAYLOAD_SCHEMA
        }

        fn classification(&self) -> InteractiveJobClassification {
            InteractiveJobClassification::Migrated
        }

        fn execution_contract(&self) -> ToolExecutionContract {
            CONTRACT
        }

        fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
            Ok(payload)
        }
    }

    impl<A: ArtifactApp<Snapshot = Mp3Snapshot>> ArtifactOwnedToolJobFactory for Mp3MediaExportJobFactory<A> {
        type Owner = A;
        const TOOL_IDS: &'static [&'static str] = &[TOOL_ID];
        const DOCUMENT_SCHEMA: &'static str = STDIO_MP3_DOCUMENT_SCHEMA;
        const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
    }

    #[derive(Default)]
    pub struct Mp3ExportSnapshotDisposer {
        pending: std::mem::ManuallyDrop<Option<Mp3Snapshot>>,
        retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<Mp3Snapshot>>>,
    }

    impl ArtifactSnapshotDisposer<Mp3Snapshot> for Mp3ExportSnapshotDisposer {
        fn retirement_demands(&self, snapshot: &Option<Arc<Mp3Snapshot>>, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if let Some(owner) = self.retirement.as_ref() {
                if owner.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<Mp3Snapshot>>>(), depth: 1, ..Default::default() }); }
                let copy = owner.next_copy_byte_demand()?;
                let release = owner.next_release_byte_demand()?;
                return Ok(semio_framework_value::RetirementDemand { copy_bytes: copy, capacity_bytes: owner.next_capacity_byte_demand(body.max(copy).max(release))?, release_bytes: release, depth: owner.next_depth_demand()? });
            }
            Ok(semio_framework_value::RetirementDemand { copy_bytes: if self.pending.is_some() || snapshot.is_some() { std::mem::size_of::<Mp3Snapshot>() } else { 0 }, capacity_bytes: 0, release_bytes: if snapshot.is_some() { semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Mp3Snapshot>() } else { 0 }, depth: usize::from(self.pending.is_some() || snapshot.is_some()) })
        }

        fn close_step(&mut self, snapshot: &mut Option<Arc<Mp3Snapshot>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin::PluginLifecycleStep, Fault> {
            use semio_framework_plugin::PluginLifecycleStep;
            use semio_framework_value::retained_clone::RetainedCloneProgress;
            if self.terminal_is_empty(snapshot) { return Ok(PluginLifecycleStep::Complete(Default::default())); }
            let demand = self.retirement_demands(snapshot, grant.maximum_copy_bytes).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::Framework, error.kind.as_str(), error.into_message()))?;
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(PluginLifecycleStep::Progress(Default::default())); }
            if let Some(owner) = self.retirement.as_mut() {
                if owner.terminal_is_empty() { *self.retirement = None; return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
                let step = owner.step(grant).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::Framework, error.kind.as_str(), error.into_message()))?;
                return Ok(PluginLifecycleStep::retained(step, self.terminal_is_empty(snapshot)));
            }
            if let Some(value) = self.pending.take() {
                match semio_framework_value::retirement::controlled::ControlledRetirement::new(value) {
                    Ok(owner) => { *self.retirement = Some(owner); return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
                    Err((error, value)) => { *self.pending = Some(value); return Err(Fault::new(semio_framework_plugin::FaultOrigin::Framework, error.kind.as_str(), error.into_message())); }
                }
            }
            let Some(owner) = snapshot.as_mut() else { return Ok(PluginLifecycleStep::Complete(Default::default())); };
            if Arc::get_mut(owner).is_none() { return Ok(PluginLifecycleStep::AwaitingInput { reason: "MP3 original snapshot observers must return before retirement" }); }
            match Arc::try_unwrap(snapshot.take().unwrap()) {
                Ok(value) => { *self.pending = Some(value); Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() })) }
                Err(owner) => { *snapshot = Some(owner); Ok(PluginLifecycleStep::AwaitingInput { reason: "MP3 original snapshot custody remains shared" }) }
            }
        }

        fn terminal_is_empty(&self, snapshot: &Option<Arc<Mp3Snapshot>>) -> bool { snapshot.is_none() && self.pending.is_none() && self.retirement.is_none() }
    }

    impl Drop for Mp3ExportSnapshotDisposer {
        fn drop(&mut self) {
            let empty = self.pending.is_none() && self.retirement.is_none();
            assert!(std::thread::panicking() || empty, "MP3 snapshot disposer abandoned original ownership");
            if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.pending); std::mem::ManuallyDrop::drop(&mut self.retirement); } }
        }
    }
}
//#endregion 🔖️Codec

#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::mpeg1_layer3::subsets::any::io::Mp3Composer as Mp3RawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Mp3RawAnyComposer>()]).as_slice()
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
    use crate::standards::mpeg1_layer3::subsets::any::schema::diff::Mp3Diff;
    use crate::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation;
    use crate::apply_mutation;

    use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Mp3BuilderConstruction {
        snapshot: Mp3Snapshot,
    }

    impl ArtifactBuilder for Mp3BuilderConstruction {
        type Snapshot = Mp3Snapshot;
        type Mutation = Mp3Mutation;
        type Diff = Mp3Diff;
        fn empty() -> Self {
            Self { snapshot: Mp3Snapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Mp3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Mp3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
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
    use crate::standards::mpeg1_layer3::subsets::any::io;
    use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Mp3Snapshot, STDIO_MP3_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Mp3Parts {
        pub snapshot: Option<Mp3Snapshot>,
    }

    pub struct Mp3AnalyzerAnalysis;

    impl ArtifactAnalysis for Mp3AnalyzerAnalysis {
        type Parts = Mp3Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return semio_framework_plugin::io::Confidence::High;
                    }
                    let marker = STDIO_MP3_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_MP3_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Mp3Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Mp3Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Mp3Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Mp3BuilderFacets {
        construction: Mp3BuilderConstruction,
        analysis: Mp3AnalyzerAnalysis,
        composition: crate::standards::mpeg1_layer3::subsets::any::io::derived_composition::Mp3ComposerComposition,
    }
    builder: Mp3Builder,
    analyzer: Mp3Analyzer,
    composer: Mp3Composer,
);
