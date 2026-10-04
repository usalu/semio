//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
    use crate::standards::mpeg1_layer3::subsets::any::schema::Mp3Analyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

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
        semio_framework_plugin::io::register_native_document_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.mp3", standard: semio_framework_plugin::StandardId("mpeg1-layer3"), subset: semio_framework_plugin::SubsetId("*") }, store::ArtifactCodec::bare::<Mp3Snapshot, crate::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation>(crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::STDIO_MP3_DOCUMENT_SCHEMA))
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
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader, Mp3Snapshot, STDIO_MP3_DOCUMENT_SCHEMA};

/// 🔍 Real magic sniff: an ID3v2 header at the front, OR a valid MPEG frame sync anywhere in the
/// buffer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    detect_id3v2_header(bytes).is_some() || find_frame_sync(bytes).is_some()
}
//#endregion 🔖️Sniff

//#region 🔖️Syncsafe
/// 📐️ Decodes a 4-byte ID3v2 synchsafe integer (7 significant bits per byte).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_syncsafe(bytes: &[u8; 4]) -> u32 {
    bytes.iter().fold(0u32, |acc, &b| (acc << 7) | (b as u32 & 0x7F))
}
/// 📐️ Encodes a `u32` (must be `< 2^28`) as a 4-byte ID3v2 synchsafe integer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_syncsafe(mut value: u32) -> [u8; 4] {
    let mut out = [0u8; 4];
    for slot in out.iter_mut().rev() {
        *slot = (value & 0x7F) as u8;
        value >>= 7;
    }
    out
}
//#endregion 🔖️Syncsafe

//#region 🔖️Id3v2
struct Id3v2HeaderRaw {
    major_version: u8,
    minor_version: u8,
    flags: u8,
    size: u32,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn detect_id3v2_header(bytes: &[u8]) -> Option<Id3v2HeaderRaw> {
    if bytes.len() < 10 || &bytes[0..3] != b"ID3" {
        return None;
    }
    let size_bytes: [u8; 4] = bytes[6..10].try_into().ok()?;
    Some(Id3v2HeaderRaw { major_version: bytes[3], minor_version: bytes[4], flags: bytes[5], size: decode_syncsafe(&size_bytes) })
}

/// 🏷️ Parses the ID3v2 tag (10-byte header + `size` bytes of frames, stopping at padding — a
/// frame id of all-zero bytes). ID3v2.3 frame sizes are a plain big-endian `u32`; ID3v2.4 frame
/// sizes are themselves synchsafe (spec difference honored here).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_id3v2(bytes: &[u8]) -> Option<(Id3v2Tag, usize)> {
    let header = detect_id3v2_header(bytes)?;
    let body_start = 10usize;
    let body_end = body_start + header.size as usize;
    if body_end > bytes.len() {
        return None;
    }
    let mut pos = body_start;
    let mut frames = Vec::new();
    while pos + 10 <= body_end {
        let id_bytes = &bytes[pos..pos + 4];
        if id_bytes.iter().all(|&b| b == 0) {
            break; // 🧮️ padding
        }
        let id = String::from_utf8_lossy(id_bytes).into_owned();
        let size_bytes: [u8; 4] = bytes[pos + 4..pos + 8].try_into().ok()?;
        let size = if header.major_version >= 4 { decode_syncsafe(&size_bytes) } else { u32::from_be_bytes(size_bytes) } as usize;
        let flags = u16::from_be_bytes([bytes[pos + 8], bytes[pos + 9]]);
        let data_start = pos + 10;
        let data_end = data_start + size;
        if data_end > body_end {
            break; // 🛡️ malformed/truncated trailing frame — stop rather than panic
        }
        frames.push(Id3Frame { id, flags, data: bytes[data_start..data_end].to_vec() });
        pos = data_end;
    }
    Some((Id3v2Tag { major_version: header.major_version, minor_version: header.minor_version, flags: header.flags, frames }, body_end))
}

/// 🏷️ Re-encodes an `Id3v2Tag` to real bytes: `ID3` + version + flags + synchsafe size, then
/// every frame's id/size/flags/data verbatim (size recomputed from `data.len()`, never carried
/// stale).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_id3v2(tag: &Id3v2Tag) -> Vec<u8> {
    let mut frames_bytes = Vec::new();
    for frame in &tag.frames {
        let mut id = frame.id.clone().into_bytes();
        id.resize(4, 0);
        frames_bytes.extend_from_slice(&id[0..4]);
        let size = frame.data.len() as u32;
        if tag.major_version >= 4 {
            frames_bytes.extend_from_slice(&encode_syncsafe(size));
        } else {
            frames_bytes.extend_from_slice(&size.to_be_bytes());
        }
        frames_bytes.extend_from_slice(&frame.flags.to_be_bytes());
        frames_bytes.extend_from_slice(&frame.data);
    }
    let mut out = Vec::with_capacity(10 + frames_bytes.len());
    out.extend_from_slice(b"ID3");
    out.push(tag.major_version);
    out.push(tag.minor_version);
    out.push(tag.flags);
    out.extend_from_slice(&encode_syncsafe(frames_bytes.len() as u32));
    out.extend_from_slice(&frames_bytes);
    out
}
//#endregion 🔖️Id3v2

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
    let id3v2 = match parse_id3v2(bytes) {
        Some((tag, consumed)) => {
            pos = consumed;
            Some(tag)
        }
        None => None,
    };

    let mut frames = Vec::new();
    while let Some(offset) = find_frame_sync(&bytes[pos..]) {
        let frame_pos = pos + offset;
        let Some((header, frame_size)) = parse_frame_header(bytes, frame_pos) else { break };
        let payload = bytes[frame_pos + 4..frame_pos + frame_size].to_vec();
        frames.push(Mp3Frame { header, payload });
        pos = frame_pos + frame_size;
    }

    let id3v1 = if bytes.len() - pos == 128 && &bytes[pos..pos + 3] == b"TAG" {
        let raw = bytes[pos..pos + 128].to_vec();
        pos += 128;
        Some(Id3v1Tag { raw })
    } else {
        None
    };
    let _ = pos;

    Ok(Mp3Snapshot { schema: STDIO_MP3_DOCUMENT_SCHEMA.into(), id3v2, frames, id3v1 })
}

/// 🚶 Re-encodes a `Mp3Snapshot` to real bytes: `id3v2` (if present) + every frame's header
/// (reconstructed from typed fields) + its retained payload + `id3v1` (if present) — for a
/// snapshot decoded from a real file, this reproduces the original bytes exactly (see
/// `codec_retention_law` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_mp3(snapshot: &Mp3Snapshot) -> Vec<u8> {
    let mut out = Vec::new();
    if let Some(tag) = &snapshot.id3v2 {
        out.extend_from_slice(&encode_id3v2(tag));
    }
    for frame in &snapshot.frames {
        out.extend_from_slice(&encode_frame_header(&frame.header));
        out.extend_from_slice(&frame.payload);
    }
    if let Some(tag) = &snapshot.id3v1 {
        out.extend_from_slice(&tag.raw);
    }
    out
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
                        u32::try_from(frame.data.len()).map_err(|_| "mp3.encode.id3v2-frame-too-large")?;
                        self.id3v2_body_bytes = self.id3v2_body_bytes.checked_add(10).and_then(|bytes| bytes.checked_add(frame.data.len())).ok_or("mp3.encode.id3v2-size-overflow")?;
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
                    let bytes = [b'I', b'D', b'3', tag.major_version, tag.minor_version, tag.flags, size[0], size[1], size[2], size[3]];
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
                    let size = u32::try_from(frame.data.len()).map_err(|_| "mp3.encode.id3v2-frame-too-large")?;
                    let encoded_size = if tag.major_version >= 4 { encode_syncsafe(size) } else { size.to_be_bytes() };
                    bytes[4..8].copy_from_slice(&encoded_size);
                    bytes[8..10].copy_from_slice(&frame.flags.to_be_bytes());
                    let chunk = self.take_slice(&bytes, maximum_bytes);
                    if self.offset == bytes.len() {
                        self.phase = if frame.data.is_empty() { self.advance_id3v2_frame(snapshot) } else { Mp3EncodePhase::Id3v2FrameData };
                        self.offset = 0;
                    }
                    return Ok(Mp3EncodeAdvance::Chunk(chunk));
                }
                Mp3EncodePhase::Id3v2FrameData => {
                    let frame = snapshot.id3v2.as_ref().and_then(|tag| tag.frames.get(self.index)).ok_or("mp3.encode.id3v2-frame-missing")?;
                    let chunk = self.take_slice(&frame.data, maximum_bytes);
                    if self.offset == frame.data.len() {
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
                    let chunk = self.take_slice(&tag.raw, maximum_bytes);
                    if self.offset == tag.raw.len() {
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
    use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, Operation, RetainedJobPayload, StepContext, StepOutcome};
    use semio_framework_plugin::app::{ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactSnapshotCloseLease};
    use semio_framework_plugin::{ArtifactApp, ArtifactMediaExportJobRequest, ArtifactOwnedToolJobFactory, ArtifactReservedJob, ArtifactReservedToolJob, ArtifactSnapshotDisposer, ArtifactToolPublicationContract, ArtifactToolPublicationLane, Fault, MediaClass, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, PluginCloseStep, PortMultiplicity};
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

    pub struct Mp3PlaybackExportJob {
        operation: Operation,
        snapshot: Option<Arc<Mp3Snapshot>>,
        snapshot_close: Option<ArtifactSnapshotCloseLease<Mp3Snapshot>>,
        cursor: Option<Mp3EncodeCursor>,
        page: Vec<u8>,
        chunks: Option<ArtifactOutputChunks>,
        credit: Option<ArtifactMediaExportCredit>,
        completion: Option<ArtifactMediaExportCompletion>,
        progress: u64,
        completed: bool,
        closing: bool,
    }

    impl Mp3PlaybackExportJob {
        pub fn new<A>(request: ArtifactMediaExportJobRequest<A>) -> Self
        where
            A: ArtifactApp<Snapshot = Mp3Snapshot>,
        {
            let cursor = Mp3EncodeCursor::new(&request.snapshot);
            Self {
                operation: request.operation,
                snapshot: Some(request.snapshot),
                snapshot_close: Some(request.snapshot_close),
                cursor: Some(cursor),
                page: Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES),
                chunks: Some(request.output_chunks),
                credit: Some(request.output_credit),
                completion: Some(request.completion),
                progress: 0,
                completed: false,
                closing: false,
            }
        }

        fn fault(context: &mut StepContext<'_>, message: &str) -> StepOutcome {
            let bytes = message.as_bytes();
            let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
            let detail = context.payload_from_bytes(JobPayloadStream::Fault, bounded).unwrap_or_else(|rejected| {
                drop(rejected.into_source());
                RetainedJobPayload::empty(JobPayloadStream::Fault)
            });
            StepOutcome::Fault(JobFault { detail })
        }

        fn advance(&mut self) -> Result<bool, Fault> {
            let snapshot = self.snapshot.as_deref().ok_or_else(|| Fault::from("mp3.export.snapshot-missing"))?;
            if self.page.len() == ArtifactOutputChunks::CHUNK_BYTES {
                let page = std::mem::replace(&mut self.page, Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES));
                self.credit.as_ref().ok_or_else(|| Fault::from("mp3.export.credit-missing"))?.credit(page.len())?;
                self.chunks.as_ref().ok_or_else(|| Fault::from("mp3.export.chunks-missing"))?.push(page)?;
                self.progress = self.progress.checked_add(1).ok_or_else(|| Fault::from("mp3.export.progress-overflow"))?;
                return Ok(false);
            }
            let maximum_bytes = ArtifactOutputChunks::CHUNK_BYTES - self.page.len();
            let advance = self.cursor.as_mut().ok_or_else(|| Fault::from("mp3.export.cursor-missing"))?.advance(snapshot, maximum_bytes).map_err(Fault::from)?;
            self.progress = self.progress.checked_add(1).ok_or_else(|| Fault::from("mp3.export.progress-overflow"))?;
            match advance {
                Mp3EncodeAdvance::Progress => Ok(false),
                Mp3EncodeAdvance::Chunk(chunk) => {
                    self.page.extend_from_slice(&chunk);
                    Ok(false)
                }
                Mp3EncodeAdvance::Complete => {
                    if !self.page.is_empty() {
                        let page = std::mem::replace(&mut self.page, Vec::with_capacity(ArtifactOutputChunks::CHUNK_BYTES));
                        self.credit.as_ref().ok_or_else(|| Fault::from("mp3.export.credit-missing"))?.credit(page.len())?;
                        self.chunks.as_ref().ok_or_else(|| Fault::from("mp3.export.chunks-missing"))?.push(page)?;
                        return Ok(false);
                    }
                    let chunks = self.chunks.take().ok_or_else(|| Fault::from("mp3.export.chunks-missing"))?;
                    chunks.seal()?;
                    self.credit.as_ref().ok_or_else(|| Fault::from("mp3.export.credit-missing"))?.credit(MEDIA_SCHEMA.len())?;
                    let result = ArtifactMediaExportResult::structured(MEDIA_TYPE, MEDIA_SCHEMA, MIME_TYPE, chunks)?;
                    self.completion.as_ref().ok_or_else(|| Fault::from("mp3.export.completion-missing"))?.complete(Ok(result))?;
                    self.completed = true;
                    Ok(true)
                }
            }
        }
    }

    impl InteractiveJob for Mp3PlaybackExportJob {
        fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
            if self.closing || context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
            if context.should_yield() {
                return StepOutcome::Yield;
            }
            if context.operation() != self.operation.operation || context.generation() != self.operation.generation || self.completed {
                return Self::fault(context, "mp3.export.operation-authority-invalid");
            }
            context.set_stage("encode-mp3");
            context.consume_fuel(1);
            match self.advance() {
                Err(error) => Self::fault(context, &format!("{}: {}", error.code.0, error.message)),
                Ok(true) => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
                Ok(false) => StepOutcome::CheckpointReady(Checkpoint { state: RetainedJobPayload::empty(JobPayloadStream::CheckpointState), applied_progress: self.progress }),
            }
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
            match ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
                Ok(PluginCloseStep::Complete) => InteractiveJobCloseStep::Complete,
                Ok(PluginCloseStep::Pending { released_items, released_bytes }) => InteractiveJobCloseStep::Pending { released_items, released_bytes },
                _ => InteractiveJobCloseStep::Blocked,
            }
        }

        fn terminal_is_empty(&self) -> bool {
            ArtifactReservedJob::terminal_is_empty(self)
        }
    }

    impl ArtifactReservedJob for Mp3PlaybackExportJob {
        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            self.begin_close();
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.cursor.take().is_some() {
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.page.capacity() != 0 {
                if maximum_bytes < self.page.capacity() {
                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                }
                let released_bytes = self.page.capacity();
                self.page = Vec::new();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes });
            }
            if self.chunks.take().is_some() {
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.completion.take().is_some() {
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.credit.take().is_some() {
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(snapshot) = self.snapshot.as_ref() {
                if !self.snapshot_close.as_ref().is_some_and(|lease| lease.can_release(snapshot)) {
                    return Err(Fault::from("mp3.export.snapshot-unwitnessed"));
                }
                self.snapshot = None;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            self.snapshot_close = None;
            Ok(PluginCloseStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.cursor.is_none() && self.page.capacity() == 0 && self.chunks.is_none() && self.completion.is_none() && self.credit.is_none() && self.snapshot.is_none() && self.snapshot_close.is_none()
        }
    }

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
        retirement: Option<Mp3SnapshotRetirement>,
    }

    impl ArtifactSnapshotDisposer<Mp3Snapshot> for Mp3ExportSnapshotDisposer {
        fn close_step(&mut self, snapshot: &mut Option<Arc<Mp3Snapshot>>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if let Some(retirement) = self.retirement.as_mut() {
                let step = retirement.close_step(maximum_bytes);
                if retirement.terminal_is_empty() {
                    self.retirement = None;
                }
                return Ok(step);
            }
            let Some(owner) = snapshot.take() else {
                return Ok(PluginCloseStep::Complete);
            };
            if let Some(value) = Arc::into_inner(owner) {
                self.retirement = Some(Mp3SnapshotRetirement::new(value));
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
        }

        fn terminal_is_empty(&self, snapshot: &Option<Arc<Mp3Snapshot>>) -> bool {
            snapshot.is_none() && self.retirement.is_none()
        }
    }

    struct Mp3SnapshotRetirement {
        schema: Option<Vec<u8>>,
        id3_frames: Vec<super::Id3Frame>,
        audio_frames: Vec<super::Mp3Frame>,
        id3v1: Option<Vec<u8>>,
        pending: Option<Vec<u8>>,
        deferred: Option<Vec<u8>>,
        pending_debt: usize,
        outer_debt: usize,
    }

    impl Mp3SnapshotRetirement {
        fn new(snapshot: Mp3Snapshot) -> Self {
            let id3_frames = snapshot.id3v2.map_or_else(Vec::new, |tag| tag.frames);
            let audio_frames = snapshot.frames;
            let outer_debt = id3_frames.capacity().saturating_mul(std::mem::size_of::<super::Id3Frame>()).saturating_add(audio_frames.capacity().saturating_mul(std::mem::size_of::<super::Mp3Frame>()));
            Self {
                schema: Some(snapshot.schema.into_bytes()),
                id3_frames,
                audio_frames,
                id3v1: snapshot.id3v1.map(|tag| tag.raw),
                pending: None,
                deferred: None,
                pending_debt: 0,
                outer_debt,
            }
        }

        fn close_step(&mut self, maximum_bytes: usize) -> PluginCloseStep {
            if self.pending.is_some() {
                let released_bytes = maximum_bytes.min(self.pending_debt);
                self.pending_debt -= released_bytes;
                if self.pending_debt == 0 {
                    self.pending = None;
                    return PluginCloseStep::Pending { released_items: 1, released_bytes };
                }
                return PluginCloseStep::Pending { released_items: 0, released_bytes };
            }
            if let Some(bytes) = self.deferred.take() {
                self.set_pending(bytes);
                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if let Some(bytes) = self.schema.take() {
                self.set_pending(bytes);
                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if let Some(frame) = self.id3_frames.pop() {
                self.deferred = Some(frame.data);
                self.set_pending(frame.id.into_bytes());
                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if let Some(frame) = self.audio_frames.pop() {
                self.set_pending(frame.payload);
                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if let Some(bytes) = self.id3v1.take() {
                self.set_pending(bytes);
                return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if self.outer_debt != 0 {
                let released_bytes = maximum_bytes.min(self.outer_debt);
                self.outer_debt -= released_bytes;
                if self.outer_debt == 0 {
                    self.id3_frames = Vec::new();
                    self.audio_frames = Vec::new();
                    return PluginCloseStep::Pending { released_items: 1, released_bytes };
                }
                return PluginCloseStep::Pending { released_items: 0, released_bytes };
            }
            PluginCloseStep::Complete
        }

        fn set_pending(&mut self, bytes: Vec<u8>) {
            self.pending_debt = bytes.capacity();
            self.pending = Some(bytes);
        }

        fn terminal_is_empty(&self) -> bool {
            self.schema.is_none() && self.id3_frames.capacity() == 0 && self.audio_frames.capacity() == 0 && self.id3v1.is_none() && self.pending.is_none() && self.deferred.is_none() && self.pending_debt == 0 && self.outer_debt == 0
        }
    }
}
//#endregion 🔖️Codec

#[cfg(test)]
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::mpeg1_layer3::subsets::any::schema::Mp3Composer as Mp3RawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Mp3RawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
