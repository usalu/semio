//! 🧬️ GifSnapshot schema (87a) — complete per GIF87a §18-24: logical screen descriptor + global
//! color table + an ordered sequence of images (GIF87a legally permits more than one Image
//! Descriptor per file even without any extension block — there is simply no per-image timing or
//! disposal metadata, since GCE is an 89a-only feature). Palette indices are stored losslessly
//! (never decoded to RGBA) — `rgba()` is a derived accessor, matching the plan's "lossless-payload
//! exception" for indexed pixel buffers. Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.

use crate::STDIO_GIF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region ColorTable
/// 🎨️ One color table entry (GCT/LCT), stored exactly as read from disk — including any
/// power-of-two padding entries past the meaningful palette, since those are real on-disk bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// 🎨️ A Global or Local Color Table. `colors.len()` must be a power of two in `2..=256` on encode
/// (the on-disk "size" field is `log2(len)-1`). `sorted` mirrors the packed byte's sort flag
/// (decreasing importance ordering — rarely used in practice, but real on-disk state).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifColorTable {
    #[value(default)]
    pub sorted: bool,
    #[value(default)]
    pub colors: Vec<GifRgb>,
}
//#endregion ColorTable

//#region ImageModel
/// 🖼️ One Table-Based Image (GIF87a §20-22): its own screen sub-rectangle, optional Local Color
/// Table, interlace flag, and losslessly-retained palette indices (NOT decoded RGBA).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct GifImage {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    #[value(default)]
    pub interlace: bool,
    #[dsl(block)]
    #[value(default)]
    pub lct: Option<GifColorTable>,
    /// 🎞️ Palette indices, row-major, natural (non-interlaced) order — length must equal
    /// `width * height`. The lossless-payload exception: this is the format's actual pixel data.
    #[dsl(base64)]
    #[value(default)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif")]
pub struct GifSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[value(default)]
    #[dsl(block)]
    pub gct: Option<GifColorTable>,
    #[state(artifact)]
    #[value(default)]
    pub background_color_index: u8,
    #[state(artifact)]
    #[value(default)]
    pub pixel_aspect_ratio: u8,
    #[state(artifact)]
    #[value(default)]
    pub images: Vec<GifImage>,
}

impl Default for GifSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_GIF_DOCUMENT_SCHEMA.into(), width: 0, height: 0, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, images: Vec::new() }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs



//#endregion HandcraftedArtifactCodecs



