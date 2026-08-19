//! 🧬️ BmpSnapshot schema — persistent fields; real codec lives in `⚙️engine` (moved there to
//! match the established stdio codec pattern — see `png`/`tiff`'s engine.rs).

use crate::artifacts::bmp::STDIO_BMP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️RowOrder
/// 📐️ BITMAPINFOHEADER's `height` field is signed: negative encodes a top-down bitmap
/// (rare, printer-friendly), positive (the overwhelming common case) encodes bottom-up. This
/// carries that as a real enum instead of leaving callers to re-derive the sign every time —
/// see `engine::decode_bmp`/`engine::encode_bmp` for how it drives row order on the wire.
/// Decoded `pixels` are always canonicalized to row 0 = image top regardless of this value.
/// 🧪️ F6: `dsl::DslScalar` — plain unit-variant enum binds as `DslField` directly (no
/// `DslVariants`/`Statements` needed, see `f6-recon-report.md` §3a/§9 STEP-2a).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum BmpRowOrder {
    #[default]
    BottomUp,
    TopDown,
}
//#endregion 🔖️RowOrder

//#region 🔖️PaletteEntry
/// 🎨️ One BITMAPINFOHEADER color-table entry (present when `bits_per_pixel <= 8`), stored in
/// the file's own on-disk field order — a weak/value entity, whole-value replaced in diffs.
/// 🧪️ F6: `dsl::DslRecord` — gives this nested value type `DslField` so it can be embedded by
/// `BmpSnapshot`'s `#[derive(dsl::DslRecord)]` and `BmpPaletteModified`/`BmpPaletteAdded`'s
/// `#[derive(dsl::DslRecord)]` in the diff module.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct BmpPaletteEntry {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub reserved: u8,
}
//#endregion 🔖️PaletteEntry

//#region 🔖️Snapshot
/// 🖼️ Complete per-spec BITMAPINFOHEADER model (11 real fields) + palette + decoded pixels.
/// `pixels` is a canonical 8-bit RGBA buffer (`width * height * 4` bytes, row 0 = image top,
/// regardless of `row_order`) — see `engine::decode_bmp`/`engine::encode_bmp` for the real
/// BITMAPFILEHEADER/BITMAPINFOHEADER codec and its documented encode scope cut.
/// 🧪️ F6: `dsl::DslRecord` — flat header + palette + rows, zero enum-in-tree, zero tri-state
/// (`Option<Option<_>>`), so the whole snapshot binds cleanly (`f6-recon-report.md` §8 row 14
/// confirmed). `#[dsl(block)]` on `palette`/`pixels` for readability (framework precedent);
/// `#[dsl(base64)]` on the bare `Vec<u8>` `pixels` field for a compact grammar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub header_size: u32,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    pub row_order: BmpRowOrder,
    #[state(artifact)]
    pub planes: u16,
    #[state(artifact)]
    pub bits_per_pixel: u16,
    #[state(artifact)]
    pub compression: u32,
    #[state(artifact)]
    pub image_size: u32,
    #[state(artifact)]
    pub x_pixels_per_meter: i32,
    #[state(artifact)]
    pub y_pixels_per_meter: i32,
    #[state(artifact)]
    pub colors_used: u32,
    #[state(artifact)]
    pub colors_important: u32,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(block)]
    pub palette: Vec<BmpPaletteEntry>,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(base64)]
    pub pixels: Vec<u8>,
}

impl Default for BmpSnapshot {
    async fn default() -> Self {
        Self {
            schema: STDIO_BMP_DOCUMENT_SCHEMA.into(),
            header_size: 40,
            width: 0,
            height: 0,
            row_order: BmpRowOrder::BottomUp,
            planes: 1,
            bits_per_pixel: 24,
            compression: 0,
            image_size: 0,
            x_pixels_per_meter: 0,
            y_pixels_per_meter: 0,
            colors_used: 0,
            colors_important: 0,
            palette: Vec::new(),
            pixels: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for BmpSnapshot {
    const EXTENSION: &'static str = "bmp";
    async fn envelope_id() -> &'static str {
        "stdio.bmp"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i + 1 < hex.len() {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("hex: {e}"), dsl::TextSpan::at(1, 1)))?);
            i += 2;
        }
        crate::artifacts::bmp::engine::decode_bmp(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let raw = crate::artifacts::bmp::engine::encode_bmp(self).unwrap_or_default();
        let body: String = raw.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BmpSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::bmp::engine::encode_bmp(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::bmp::engine::decode_bmp(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
