//! 🧬️ Mp3Snapshot — an optional typed ID3v2 header (+ its typed frames), a sequence of typed
//! MPEG frame headers with opaque-retained payload (honest boundary — no Huffman/MDCT decode:
//! this is a container-level codec, not a full audio decoder), and an optional typed ID3v1
//! trailer. Real byte-accurate codec (see `⚙️engine`), not a container placeholder.

#[path = "🏷️metadata/🦀️.rs"]
pub mod metadata;
pub use metadata::{Id3Content, Id3Frame, Id3v1Tag, Id3v2Tag, id3_content_kind, validate_id3_frame, validate_id3v1_tag};

/// 📦️ Owned by `mp3`: one MPEG audio frame header, every field of the real 4-byte header typed
/// individually (raw bit-field values, matching the spec's own encoding — e.g.
/// `channel_mode: 3` = mono, per `fixtures/mp3/NOTES.md`), plus the frame's payload bytes
/// (opaque-retained; the HONEST boundary this artifact draws — no Huffman/MDCT decode).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Mp3FrameHeader {
    /// `0`=MPEG2.5, `2`=MPEG2, `3`=MPEG1 (`1` is the spec-reserved value).
    pub mpeg_version_id: u8,
    /// `1`=Layer III, `2`=Layer II, `3`=Layer I (`0` is spec-reserved).
    pub layer: u8,
    /// `true` = protection bit set = NO CRC follows the header.
    pub protection_bit: bool,
    pub bitrate_index: u8,
    pub sample_rate_index: u8,
    pub padding: bool,
    pub private_bit: bool,
    /// `0`=stereo, `1`=joint stereo, `2`=dual channel, `3`=mono.
    pub channel_mode: u8,
    pub mode_extension: u8,
    pub copyright: bool,
    pub original: bool,
    pub emphasis: u8,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Mp3Frame {
    pub header: Mp3FrameHeader,
    #[value(default)]
    #[dsl(base64)]
    pub payload: Vec<u8>,
}

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_MP3_DOCUMENT_SCHEMA: &str = "stdio.mp3";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.mp3")]
pub struct Mp3Snapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub id3v2: Option<Id3v2Tag>,
    #[state(artifact)]
    #[value(default)]
    pub frames: Vec<Mp3Frame>,
    #[state(artifact)]
    #[value(default)]
    pub id3v1: Option<Id3v1Tag>,
}

impl Default for Mp3Snapshot {
    fn default() -> Self {
        Self { schema: STDIO_MP3_DOCUMENT_SCHEMA.into(), id3v2: Default::default(), frames: Default::default(), id3v1: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests




#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
