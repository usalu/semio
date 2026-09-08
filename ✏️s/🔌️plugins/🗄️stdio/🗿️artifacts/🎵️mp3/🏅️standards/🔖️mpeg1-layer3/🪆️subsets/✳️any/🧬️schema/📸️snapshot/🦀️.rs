//! 🧬️ Mp3Snapshot — an optional typed ID3v2 header (+ its typed frames), a sequence of typed
//! MPEG frame headers with opaque-retained payload (honest boundary — no Huffman/MDCT decode:
//! this is a container-level codec, not a full audio decoder), and an optional typed ID3v1
//! trailer. Real byte-accurate codec (see `⚙️engine`), not a container placeholder.

/// 📦️ Owned by `mp3`: one ID3v2 text/binary frame, typed-raw (`id`/`flags` decoded, `data`
/// retained verbatim — this codec does not interpret ID3 text-encoding bytes).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Id3Frame {
    pub id: String,
    pub flags: u16,
    #[value(default)]
    pub data: Vec<u8>,
}

/// 📦️ Owned by `mp3`: the ID3v2 tag header (version/flags, as two named fields — not a bare
/// tuple, per the recipe's own ban) + its frames.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Id3v2Tag {
    pub major_version: u8,
    pub minor_version: u8,
    pub flags: u8,
    #[value(default)]
    pub frames: Vec<Id3Frame>,
}

/// 📦️ Owned by `mp3`: the 128-byte ID3v1 trailer, retained verbatim as a NAMED struct (not a
/// bare `[u8;128]`, per the recipe's tuple/array-gap guidance) — this codec does not decode
/// ID3v1's fixed-width title/artist/album/year/comment/genre sub-fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Id3v1Tag {
    #[value(default)]
    pub raw: Vec<u8>,
}

/// 📦️ Owned by `mp3`: one MPEG audio frame header, every field of the real 4-byte header typed
/// individually (raw bit-field values, matching the spec's own encoding — e.g.
/// `channel_mode: 3` = mono, per `fixtures/mp3/NOTES.md`), plus the frame's payload bytes
/// (opaque-retained; the HONEST boundary this artifact draws — no Huffman/MDCT decode).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Mp3Frame {
    pub header: Mp3FrameHeader,
    #[value(default)]
    pub payload: Vec<u8>,
}

use framework_schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_MP3_DOCUMENT_SCHEMA: &str = "stdio.mp3";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
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
/// 🎧️ `ArtifactDsl`/`ArtifactPack` route through the REAL container codec
/// (`⚙️engine::encode_mp3`/`decode_mp3`) — the envelope wraps genuine on-disk MP3 bytes, the
/// same convention `BmpSnapshot`'s handcrafted codecs use (real format bytes inside the
/// `store::semio_format` envelope, not a JSON re-serialization of the Rust type).
impl store::ArtifactDsl for Mp3Snapshot {
    const EXTENSION: &'static str = "mp3";
    fn envelope_id() -> &'static str {
        STDIO_MP3_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if !hex.len().is_multiple_of(2) {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(&bytes).map_err(|e| store::TextError::new(format!("mp3 decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::standards::mpeg1_layer3::subsets::any::io::encode_mp3(self);
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Mp3Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::standards::mpeg1_layer3::subsets::any::io::encode_mp3(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
