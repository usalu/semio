//! 🧬️ DeflateSnapshot schema — typed RFC1950 zlib container + real codecs.

use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region 🔖️CompressionLevelHint
/// 🎚️ RFC1950 §2.2 FLG.FLEVEL: a two-bit hint the encoder leaves for tooling about which
/// compression strategy it used. Never affects decoding — purely informational.
///
/// 🧪️ F6: `dsl::DslScalar` — a plain unit-variant enum binds as `DslField` directly (no
/// `DslVariants`/`Statements` needed), so `DeflateSnapshot`'s `compression_level_hint` field and
/// `DeflateMutation::SetCompressionParams`'s `level_hint` argument can both embed it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DeflateLevelHint {
    Fastest,
    Fast,
    #[default]
    Default,
    Maximum,
}

impl DeflateLevelHint {
    /// 📐️ Decodes FLG's 2-bit FLEVEL field.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0 => DeflateLevelHint::Fastest,
            1 => DeflateLevelHint::Fast,
            2 => DeflateLevelHint::Default,
            _ => DeflateLevelHint::Maximum,
        }
    }
    /// 📐️ Encodes to FLG's 2-bit FLEVEL field.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
///
/// 🧪️ F6: `dsl::DslRecord` added alongside the existing hand-rolled `store::ArtifactDsl`/
/// `store::ArtifactPack` below — NOT a replacement (same treatment as `BinarySnapshot`).
/// `DslRecord` only gives this type `DslField` so it can be embedded as a block in a record; it does not touch the artifact's own
/// honest hex-text/raw-binary envelope format.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate")]
pub struct DeflateSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🧪️ CMF low nibble (CM). RFC1950 defines only `8` (deflate) as legal; other values are
    /// spec-reserved and retained honestly rather than rejected at the type level.
    #[state(artifact)]
    #[value(default)]
    pub compression_method: u8,
    /// 🪟️ CMF high nibble (CINFO). Window size = `2^(cinfo+8)`; values `0..=7` are valid for
    /// deflate (up to the 32KB RFC1951 window), `8..=15` are spec-reserved.
    #[state(artifact)]
    #[value(default)]
    pub window_bits: u8,
    /// 🎚️ FLG.FLEVEL: informational compression-strategy hint.
    #[state(artifact)]
    #[value(default)]
    pub compression_level_hint: DeflateLevelHint,
    /// 📖️ FLG.FDICT + DICTID: the preset dictionary's Adler-32 id, present only when a preset
    /// dictionary was declared. `None` means FDICT is clear.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<u32>,
    /// 📦️ The decompressed payload -- the format's actual content IS bytes, so this `Vec<u8>` is
    /// the recipe's legitimate exception, not generic-code-to-kill.
    #[state(artifact)]
    #[value(default)]
    #[dsl(base64)]
    pub payload: Vec<u8>,
}

impl Default for DeflateSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: DeflateLevelHint::default(), dict_id: None, payload: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs





