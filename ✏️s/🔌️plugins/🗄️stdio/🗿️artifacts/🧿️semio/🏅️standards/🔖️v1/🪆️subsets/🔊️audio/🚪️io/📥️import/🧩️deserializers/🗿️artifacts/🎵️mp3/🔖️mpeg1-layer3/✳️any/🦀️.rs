//! 📥️ Deserialize `s.stdio.mp3` (mpeg1-layer3/✳️any) into `s.stdio.semio` (v1/audio) —
//! metadata-plus-opaque-payload only, per the ticket brief's explicit, unavoidable asymmetry:
//! mp3's own honest boundary never decodes Huffman/MDCT frame payloads to samples (see
//! `Mp3FrameHeader`'s own doc comment), so this deserializer CANNOT produce real f32 samples the
//! way `audio↔wav` does. What IS real and honest here: `sample_rate` is derived from the first
//! frame's `mpeg_version_id`/`sample_rate_index` via the genuine MPEG Table 3.B.2 lookup (never
//! guessed), `channels` gets the correct COUNT from `channel_mode` (mono=1, else=2) with each
//! channel's `samples` left empty (never fabricated PCM), and `tags` carries real ID3v2 text
//! frames (best-effort ISO-8859-1/UTF-16 decode of the frame's raw bytes, matching real-world ID3
//! text-frame content) plus a synthetic `"id3v1.raw"` tag when an ID3v1 trailer is present (its
//! 128 bytes are NOT sub-field-decoded here -- `Id3v1Tag` itself only retains them verbatim, see
//! its own doc comment -- so this bridge doesn't fabricate a decode ID3v1's own type declines to
//! do either).

use semio_s_artifact_stdio_mp3::Mp3Snapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.mp3", standard: StandardId("mpeg1-layer3"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

pub struct SemioAudioFromMp3;

impl ArtifactDeserializer for SemioAudioFromMp3 {
    type From = Mp3Snapshot;
    type Into = SemioAudioSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let (sample_rate, channel_count) = match from.frames.first() {
            Some(frame) => (mpeg_sample_rate(frame.header.mpeg_version_id, frame.header.sample_rate_index), if frame.header.channel_mode == 3 { 1 } else { 2 }),
            None => (0, 0),
        };
        let channels = (0..channel_count).map(|_| SemioAudioChannel { samples: Vec::new() }).collect();

        let mut tags = Vec::new();
        if let Some(id3v2) = &from.id3v2 {
            for frame in &id3v2.frames {
                tags.push(SemioAudioTag { key: frame.id.clone(), value: decode_id3_text(&frame.data) });
            }
        }
        if let Some(id3v1) = &from.id3v1 {
            tags.push(SemioAudioTag { key: "id3v1.raw".into(), value: id3v1.raw.iter().map(|b| format!("{b:02x}")).collect() });
        }

        Ok(SemioAudioSnapshot { schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(), sample_rate, format: SemioAudioFormat::default(), channels, tags })
    }
}

/// 📐️ MPEG-1/2/2.5 Layer III sample rate table (ISO/IEC 11172-3 Table 3.B.2), keyed by
/// `(mpeg_version_id, sample_rate_index)`. `sample_rate_index == 3` is spec-reserved; falls back
/// honestly to `0` (never a fabricated guess) for both the reserved index and unrecognized version.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mpeg_sample_rate(version_id: u8, index: u8) -> u32 {
    match (version_id, index) {
        (3, 0) => 44_100,
        (3, 1) => 48_000,
        (3, 2) => 32_000, // MPEG1
        (2, 0) => 22_050,
        (2, 1) => 24_000,
        (2, 2) => 16_000, // MPEG2
        (0, 0) => 11_025,
        (0, 1) => 12_000,
        (0, 2) => 8_000, // MPEG2.5
        _ => 0,
    }
}

/// 🔤️ Best-effort ID3v2 text-frame decode: a leading encoding byte (`0`=ISO-8859-1, `1`=UTF-16
/// w/ BOM, `2`/`3`=UTF-16BE/UTF-8 in ID3v2.4) followed by the text; falls back to a lossy
/// byte-for-byte Latin-1 mapping on anything this doesn't recognize -- never panics, never drops
/// the frame outright.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_id3_text(data: &[u8]) -> String {
    match data.first() {
        Some(0) => data[1..].iter().map(|&b| b as char).collect(),
        Some(3) => String::from_utf8_lossy(&data[1..]).into_owned(),
        Some(1) | Some(2) if data.len() > 2 => {
            let body = &data[1..];
            let units: Vec<u16> = body.as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
            String::from_utf16_lossy(&units)
        }
        _ => data.iter().map(|&b| b as char).collect(),
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
