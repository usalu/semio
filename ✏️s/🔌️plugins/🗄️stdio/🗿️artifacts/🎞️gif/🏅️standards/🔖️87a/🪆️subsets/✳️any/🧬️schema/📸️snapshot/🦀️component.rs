//! 🧬️ GifSnapshot schema (87a) — complete per GIF87a §18-24: logical screen descriptor + global
//! color table + an ordered sequence of images (GIF87a legally permits more than one Image
//! Descriptor per file even without any extension block — there is simply no per-image timing or
//! disposal metadata, since GCE is an 89a-only feature). Palette indices are stored losslessly
//! (never decoded to RGBA) — `rgba()` is a derived accessor, matching the plan's "lossless-payload
//! exception" for indexed pixel buffers. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.

use crate::artifacts::gif::STDIO_GIF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region ColorTable
/// 🎨️ One color table entry (GCT/LCT), stored exactly as read from disk — including any
/// power-of-two padding entries past the meaningful palette, since those are real on-disk bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 🎨️ A Global or Local Color Table. `colors.len()` must be a power of two in `2..=256` on encode
/// (the on-disk "size" field is `log2(len)-1`). `sorted` mirrors the packed byte's sort flag
/// (decreasing importance ordering — rarely used in practice, but real on-disk state).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifColorTable {
    #[serde(default)]
    pub sorted: bool,
    #[serde(default)]
    pub colors: Vec<GifRgb>,
}
//#endregion ColorTable

//#region ImageModel
/// 🖼️ One Table-Based Image (GIF87a §20-22): its own screen sub-rectangle, optional Local Color
/// Table, interlace flag, and losslessly-retained palette indices (NOT decoded RGBA).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GifImage {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub interlace: bool,
    #[dsl(block)]
    #[serde(default)]
    pub lct: Option<GifColorTable>,
    /// 🎞️ Palette indices, row-major, natural (non-interlaced) order — length must equal
    /// `width * height`. The lossless-payload exception: this is the format's actual pixel data.
    #[dsl(base64)]
    #[serde(default)]
    pub indices: Vec<u8>,
}

impl GifImage {
    /// 🖌️ Derived RGBA accessor — decodes `indices` through `lct` (falling back to `gct` when this
    /// image has no local table). `rgba()` is intentionally NOT a stored field: GIF87a has no
    /// transparency concept at all, so every pixel is fully opaque.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rgba(&self, gct: Option<&GifColorTable>) -> Vec<u8> {
        let table = self.lct.as_ref().or(gct);
        let mut out = Vec::with_capacity(self.indices.len() * 4);
        for &idx in &self.indices {
            let rgb = table.and_then(|t| t.colors.get(idx as usize)).copied().unwrap_or_default();
            out.extend_from_slice(&[rgb.r, rgb.g, rgb.b, 255]);
        }
        out
    }
}
//#endregion ImageModel

//#region Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif")]
pub struct GifSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(block)]
    pub gct: Option<GifColorTable>,
    #[state(artifact)]
    #[serde(default)]
    pub background_color_index: u8,
    #[state(artifact)]
    #[serde(default)]
    pub pixel_aspect_ratio: u8,
    #[state(artifact)]
    #[serde(default)]
    pub images: Vec<GifImage>,
}

impl Default for GifSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), width: 0, height: 0, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, images: Vec::new() }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for GifSnapshot {
    const EXTENSION: &'static str = "gif";
    async fn envelope_id() -> &'static str {
        "stdio.gif"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::gif::standards::v87a::engine::decode_gif(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    async fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::gif::standards::v87a::engine::encode_gif(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GifSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::gif::standards::v87a::engine::encode_gif(self).map_err(store::PackError::Schema)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id().await, store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        crate::artifacts::gif::standards::v87a::engine::decode_gif(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion HandcraftedArtifactCodecs
