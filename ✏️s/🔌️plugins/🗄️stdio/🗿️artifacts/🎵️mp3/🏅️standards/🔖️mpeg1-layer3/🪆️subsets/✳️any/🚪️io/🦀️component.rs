//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{
        ArtifactComposition, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    };
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::Mp3Analyzer;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct Mp3ComposerComposition;

    impl ArtifactComposition for Mp3ComposerComposition {
        type Snapshot = Mp3Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] { &[DIALECT] }

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
                return Err(ComposeError { message: "Mp3ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = Mp3Analyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "Mp3ComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec. Called from
    /// this artifact's standard-level `engine::register()`.
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mp3_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<Mp3Snapshot, crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation>(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::STDIO_MP3_DOCUMENT_SCHEMA));
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.mp3.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::inferences::mp3_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Sniff
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{
    Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader, Mp3Snapshot, STDIO_MP3_DOCUMENT_SCHEMA,
};

/// 🔍 Real magic sniff: an ID3v2 header at the front, OR a valid MPEG frame sync anywhere in the
/// buffer.
pub fn sniff_real_bytes(bytes: &[u8]) -> bool {
    detect_id3v2_header(bytes).is_some() || find_frame_sync(bytes).is_some()
}
//#endregion 🔖️Sniff

//#region 🔖️Syncsafe
/// 📐️ Decodes a 4-byte ID3v2 synchsafe integer (7 significant bits per byte).
fn decode_syncsafe(bytes: &[u8; 4]) -> u32 {
    bytes.iter().fold(0u32, |acc, &b| (acc << 7) | (b as u32 & 0x7F))
}
/// 📐️ Encodes a `u32` (must be `< 2^28`) as a 4-byte ID3v2 synchsafe integer.
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
    let sample_rate = crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::sample_rate_hz(mpeg_version_id, sample_rate_index)?;
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
    Some((
        Mp3FrameHeader {
            mpeg_version_id,
            layer,
            protection_bit,
            bitrate_index,
            sample_rate_index,
            padding,
            private_bit,
            channel_mode,
            mode_extension,
            copyright,
            original,
            emphasis,
        },
        frame_size,
    ))
}

/// 📐️ Re-encodes a frame header's typed fields back to the real 4 header bytes.
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
    loop {
        match find_frame_sync(&bytes[pos..]) {
            Some(offset) => {
                let frame_pos = pos + offset;
                match parse_frame_header(bytes, frame_pos) {
                    Some((header, frame_size)) => {
                        let payload = bytes[frame_pos + 4..frame_pos + frame_size].to_vec();
                        frames.push(Mp3Frame { header, payload });
                        pos = frame_pos + frame_size;
                    }
                    None => break, // sync word without a decodable header — stop (honest boundary)
                }
            }
            None => break,
        }
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
//#endregion 🔖️Codec

#[cfg(test)]
mod codec_tests {
    use super::*;

    /// 🌱 Real ID3v2.3.0 + 4× MPEG1 Layer III fixture — byte-identical to the artifact's own
    /// `📚️examples/🎬️demo/🖼️assets/🎵️example.mp3` (per ticket `fixtures/mp3/NOTES.md`), duplicated
    /// here as a literal so the test doesn't reach across an emoji-path `include_bytes!`
    /// boundary.
    fn real_fixture() -> Vec<u8> {
        include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎵️example.mp3").to_vec()
    }

    #[test]
    fn detects_a_synthetic_id3v2_header() {
        let mut bytes = b"ID3".to_vec();
        bytes.extend_from_slice(&[0x03, 0x00, 0x00]);
        bytes.extend_from_slice(&[0x00, 0x00, 0x02, 0x01]);
        let hdr = detect_id3v2_header(&bytes).expect("id3v2");
        assert_eq!(hdr.major_version, 3);
        assert_eq!(hdr.size, 257);
    }

    #[test]
    fn finds_a_synthetic_mpeg1_layer3_frame_sync() {
        let bytes = [0x00, 0x00, 0xFF, 0xFB, 0x90, 0x00];
        assert_eq!(find_frame_sync(&bytes), Some(2));
    }

    #[test]
    fn no_id3v2_header_returns_none() {
        assert!(detect_id3v2_header(b"not an id3 tag").is_none());
    }

    #[test]
    fn frame_header_bit_layout_round_trips() {
        // FF FB 90 C4: MPEG1(11) Layer III(01) no-CRC(1), bitrate idx 9, sr idx 0, mono, original.
        // 128kbps/44100Hz gives a real 417-byte frame (`144*128000/44100`, no padding) —
        // `parse_frame_header` honestly bounds-checks the WHOLE frame against the buffer (not
        // just its 4 header bytes), so the buffer must actually be 417 bytes long.
        let header_bytes = [0xFF, 0xFB, 0x90, 0xC4];
        let mut bytes = header_bytes.to_vec();
        bytes.resize(417, 0);
        let (header, size) = parse_frame_header(&bytes, 0).expect("header");
        assert_eq!(header.mpeg_version_id, 0b11);
        assert_eq!(header.layer, 0b01);
        assert!(header.protection_bit);
        assert_eq!(header.bitrate_index, 9);
        assert_eq!(header.sample_rate_index, 0);
        assert!(!header.padding);
        assert_eq!(header.channel_mode, 0b11);
        assert!(header.original);
        assert_eq!(size, 417);
        assert_eq!(encode_frame_header(&header), header_bytes);
    }

    //#region codec_retention_law
    /// 🧪️ `codec_retention_law`: mp3's honest boundary is the container level — frame COUNT and
    /// every typed header field must round-trip exactly, and the full byte stream (incl. opaque
    /// payload bytes) must re-encode byte-identical, even though the payload itself is never
    /// Huffman-decoded.
    #[test]
    fn codec_retention_law() {
        let fixture = real_fixture();
        let decoded = decode_mp3(&fixture).expect("decode real fixture");

        let tag = decoded.id3v2.as_ref().expect("id3v2 tag present");
        assert_eq!(tag.major_version, 3);
        assert_eq!(tag.minor_version, 0);
        assert_eq!(tag.frames.len(), 2, "TIT2 + TPE1");
        assert_eq!(tag.frames[0].id, "TIT2");
        assert_eq!(tag.frames[1].id, "TPE1");
        assert_eq!(String::from_utf8_lossy(&tag.frames[0].data[1..]), "semio fixture");
        assert_eq!(String::from_utf8_lossy(&tag.frames[1].data[1..]), "W0 handcraft");

        assert_eq!(decoded.frames.len(), 4, "4 MPEG frames per fixtures/mp3/NOTES.md");
        for frame in &decoded.frames {
            assert_eq!(frame.header.mpeg_version_id, 0b11, "MPEG1");
            assert_eq!(frame.header.layer, 0b01, "Layer III");
            assert!(frame.header.protection_bit, "no CRC");
            assert_eq!(frame.header.bitrate_index, 9, "128kbps");
            assert_eq!(frame.header.sample_rate_index, 0, "44100Hz");
            assert!(!frame.header.padding);
            assert_eq!(frame.header.channel_mode, 0b11, "mono");
            assert!(frame.header.original);
            assert_eq!(frame.payload.len(), 413, "417 - 4 header bytes");
        }
        assert!(decoded.id3v1.is_none(), "fixture has no trailing ID3v1 tag");

        let re_encoded = encode_mp3(&decoded);
        assert_eq!(re_encoded, fixture, "encode(decode(real fixture)) must be byte-identical");

        let re_decoded = decode_mp3(&re_encoded).expect("decode re-encoded");
        assert_eq!(re_decoded, decoded);
    }
    //#endregion codec_retention_law

    //#region 🔖️Id3v1Retention
    #[test]
    fn id3v1_trailer_round_trips() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[0xFF, 0xFB, 0x90, 0xC4]);
        bytes.extend(std::iter::repeat(0u8).take(413));
        let mut tag = b"TAG".to_vec();
        tag.resize(128, 0);
        bytes.extend_from_slice(&tag);

        let decoded = decode_mp3(&bytes).expect("decode");
        assert_eq!(decoded.frames.len(), 1);
        let id1 = decoded.id3v1.as_ref().expect("id3v1 tag present");
        assert_eq!(id1.raw.len(), 128);
        assert_eq!(&id1.raw[0..3], b"TAG");

        let re_encoded = encode_mp3(&decoded);
        assert_eq!(re_encoded, bytes);
    }
    //#endregion 🔖️Id3v1Retention
}

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::Mp3Composer as Mp3RawAnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Mp3RawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
