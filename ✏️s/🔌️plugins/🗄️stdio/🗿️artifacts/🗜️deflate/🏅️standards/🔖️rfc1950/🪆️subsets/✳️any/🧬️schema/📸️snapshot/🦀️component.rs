//! 🧬️ DeflateSnapshot schema — typed RFC1950 zlib container + real codecs.

use crate::artifacts::deflate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️CompressionLevelHint
/// 🎚️ RFC1950 §2.2 FLG.FLEVEL: a two-bit hint the encoder leaves for tooling about which
/// compression strategy it used. Never affects decoding — purely informational.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DeflateLevelHint {
    Fastest,
    Fast,
    #[default]
    Default,
    Maximum,
}

impl DeflateLevelHint {
    /// 📐️ Decodes FLG's 2-bit FLEVEL field.
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0 => DeflateLevelHint::Fastest,
            1 => DeflateLevelHint::Fast,
            2 => DeflateLevelHint::Default,
            _ => DeflateLevelHint::Maximum,
        }
    }
    /// 📐️ Encodes to FLG's 2-bit FLEVEL field.
    pub fn to_bits(self) -> u8 {
        match self {
            DeflateLevelHint::Fastest => 0,
            DeflateLevelHint::Fast => 1,
            DeflateLevelHint::Default => 2,
            DeflateLevelHint::Maximum => 3,
        }
    }
}
//#endregion 🔖️CompressionLevelHint

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.deflate` snapshot: typed RFC1950 zlib container fields (CMF/FLG) plus the
/// decompressed payload. `dict_id` and the adler32 trailer are NOT independently source-of-truth:
/// `dict_id` is only ever present when a preset dictionary was actually declared (FDICT), and the
/// adler32 trailer is always recomputed fresh from `payload` on encode (never carried stale) --
/// same treatment RFC1950 §2.3 mandates for the checksum, extended here to FCHECK too (both are
/// pure functions of the other header bits, not independently-settable data).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate")]
pub struct DeflateSnapshot {
    #[state(persistent)]
    pub schema: String,
    /// 🧪️ CMF low nibble (CM). RFC1950 defines only `8` (deflate) as legal; other values are
    /// spec-reserved and retained honestly rather than rejected at the type level.
    #[state(persistent)]
    #[serde(default)]
    pub compression_method: u8,
    /// 🪟️ CMF high nibble (CINFO). Window size = `2^(cinfo+8)`; values `0..=7` are valid for
    /// deflate (up to the 32KB RFC1951 window), `8..=15` are spec-reserved.
    #[state(persistent)]
    #[serde(default)]
    pub window_bits: u8,
    /// 🎚️ FLG.FLEVEL: informational compression-strategy hint.
    #[state(persistent)]
    #[serde(default)]
    pub compression_level_hint: DeflateLevelHint,
    /// 📖️ FLG.FDICT + DICTID: the preset dictionary's Adler-32 id, present only when a preset
    /// dictionary was declared. `None` means FDICT is clear.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<u32>,
    /// 📦️ The decompressed payload -- the format's actual content IS bytes, so this `Vec<u8>` is
    /// the recipe's legitimate exception, not generic-code-to-kill.
    #[state(persistent)]
    #[serde(default)]
    pub payload: Vec<u8>,
}

impl Default for DeflateSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
            compression_method: 8,
            window_bits: 7,
            compression_level_hint: DeflateLevelHint::default(),
            dict_id: None,
            payload: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for DeflateSnapshot {
    const EXTENSION: &'static str = "zz";
    fn envelope_id() -> &'static str { "stdio.deflate" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut zlib_bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?;
            zlib_bytes.push(byte);
            i += 2;
        }
        crate::artifacts::deflate::engine::decode_deflate_snapshot(&zlib_bytes)
            .map_err(|e| store::TextError::new(format!("zlib decode: {e}"), dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let zlib_bytes = crate::artifacts::deflate::engine::encode_deflate_snapshot(self);
        let body: String = zlib_bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for DeflateSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;

        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let zlib_bytes = crate::artifacts::deflate::engine::encode_deflate_snapshot(self);
        Ok(store::semio_format::wrap_binary(&envelope, &zlib_bytes))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {

        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        crate::artifacts::deflate::engine::decode_deflate_snapshot(&inner)
            .map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
