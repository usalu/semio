//! 🧬️ SemioImageSnapshot — complete per the master plan's image subset spec: width/height/
//! colorspace/bit-depth + frames{delay_ms, rgba8 pixels} + embedded ICC profile + metadata
//! entries. Informed by png's typed IHDR/ancillary model and gif 89a's frame sequence; replaces
//! the pre-migration `RasterImage`. Ticket
//! 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT (W2b/image). This is
//! a NEUTRAL semio type (not itself an on-disk file format), so its own `ArtifactDsl`/
//! `ArtifactPack` stay a JSON-then-hex/binary envelope passthrough — real per-format bytes
//! (png/gif/bmp/jpg/tiff) are produced by the semio↔format `🚪️io` leaves (W4), not here.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — the semio design
/// (unlike gif 87a/89a's deliberately-split convention) uses the SAME literal for both, per the
/// master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`" note, one per
/// subset. Must stay repo-wide unique — `register_document_codec` duplicate-id detection is a
/// static policy check.
pub const STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA: &str = "s.stdio.semio.image";
//#endregion 🔖️Ids

//#region 🔖️Colorspace
/// 🎨️ Source pixel colorspace — every frame's `rgba8` buffer is always normalized to RGBA8 on
/// decode (per the master plan's snapshot spec), so this field records the SOURCE colorspace for
/// honest round-trip/re-encode decisions, not a second in-memory pixel layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SemioColorspace {
    #[default]
    Rgb,
    Rgba,
    Grayscale,
    GrayscaleAlpha,
    Indexed,
}
//#endregion 🔖️Colorspace

//#region 🔖️Frame
/// 🖼️ One decoded frame: always-RGBA8 pixels (row-major, `width*height*4` bytes) plus its
/// animation delay. A single-frame image (png/jpg/bmp/tiff) has exactly one `SemioImageFrame`
/// with `delay_ms: 0`. Strong entity — per-field diffable (see `🔺️diff`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioImageFrame {
    pub delay_ms: u32,
    #[serde(default)]
    pub rgba8: Vec<u8>,
}
//#endregion 🔖️Frame

//#region 🔖️Metadata
/// 🏷️ One textual metadata entry (png tEXt/iTXt, exif-as-text, gif comment-extension-derived, …)
/// — name-keyed by `key`. Weak/value entity: its "diff" is the whole new value, never sub-diffed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemioImageMetadataEntry {
    pub key: String,
    #[serde(default)]
    pub value: String,
}
//#endregion 🔖️Metadata

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image")]
pub struct SemioImageSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub colorspace: SemioColorspace,
    #[state(persistent)]
    #[serde(default)]
    pub bit_depth: u8,
    #[state(persistent)]
    #[serde(default)]
    pub frames: Vec<SemioImageFrame>,
    /// 🎨️ Embedded ICC color profile bytes, verbatim — `None` when the source carried none.
    #[state(persistent)]
    #[serde(default)]
    pub icc: Option<Vec<u8>>,
    #[state(persistent)]
    #[serde(default)]
    pub metadata: Vec<SemioImageMetadataEntry>,
}

impl Default for SemioImageSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: 0,
            height: 0,
            colorspace: SemioColorspace::default(),
            bit_depth: 0,
            frames: Vec::new(),
            icc: None,
            metadata: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🧾 JSON-then-hex envelope round trip (honest — a genuinely working codec, not a per-format
/// binary decoder, since this subset's snapshot is a NEUTRAL semio type). Wrapped in the same
/// `store::semio_format` envelope every stdio artifact uses.
impl store::ArtifactDsl for SemioImageSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioImageSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn populated() -> SemioImageSnapshot {
        SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: 2,
            height: 2,
            colorspace: SemioColorspace::Rgba,
            bit_depth: 8,
            frames: vec![SemioImageFrame { delay_ms: 100, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255] }],
            icc: Some(vec![1, 2, 3, 4]),
            metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "test".into() }],
        }
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = SemioImageSnapshot::default();
        let bytes = <SemioImageSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = SemioImageSnapshot::default();
        let text = <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
    /// on a fully-populated snapshot (frames/icc/metadata all non-empty), not just the default.
    #[test]
    fn codec_retention_law() {
        let snap = populated();
        let bytes = <SemioImageSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
        let text = <SemioImageSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🔖️Tests
