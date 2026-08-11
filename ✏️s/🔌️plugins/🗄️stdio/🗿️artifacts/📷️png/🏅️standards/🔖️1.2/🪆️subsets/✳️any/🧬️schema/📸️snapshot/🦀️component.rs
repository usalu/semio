//! 🧬️ PngSnapshot schema — complete PNG 1.2 semantic model, real codecs. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `RasterImage`-shaped stub (`width`/`height`/`rgba` only) with typed IHDR, PLTE, tRNS, the
//! full typed ancillary set (gAMA/cHRM/sRGB/pHYs/tIME/bKGD), name-duplicate-safe index-keyed
//! text chunks (tEXt/zTXt/iTXt), decoded pixels, and chunk-ORDER + unknown-chunk verbatim
//! retention so nothing real on disk is silently dropped (`## Snapshot completeness spec`).

use crate::artifacts::png::STDIO_PNG_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region ColorModel
/// 🎨️ PNG §11.2.2 `IHDR` color type. `Palette` requires a `PLTE` chunk; `compression method`
/// and `filter method` are always `0` per spec and are validated on decode, never modeled here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PngColorType {
    Grayscale,
    Rgb,
    Palette,
    GrayscaleAlpha,
    #[default]
    Rgba,
}

impl PngColorType {
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(PngColorType::Grayscale),
            2 => Ok(PngColorType::Rgb),
            3 => Ok(PngColorType::Palette),
            4 => Ok(PngColorType::GrayscaleAlpha),
            6 => Ok(PngColorType::Rgba),
            _ => Err(format!("png: unsupported color type {v}")),
        }
    }
    pub fn to_u8(self) -> u8 {
        match self {
            PngColorType::Grayscale => 0,
            PngColorType::Rgb => 2,
            PngColorType::Palette => 3,
            PngColorType::GrayscaleAlpha => 4,
            PngColorType::Rgba => 6,
        }
    }
    /// 🔢️ Samples per pixel before any palette indirection.
    pub fn samples_per_pixel(self) -> usize {
        match self {
            PngColorType::Grayscale | PngColorType::Palette => 1,
            PngColorType::Rgb => 3,
            PngColorType::GrayscaleAlpha => 2,
            PngColorType::Rgba => 4,
        }
    }
}
//#endregion ColorModel

//#region Palette
/// 🎨️ One `PLTE` entry — a weak value (whole-value replaced in diffs, never sub-diffed).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PngRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
//#endregion Palette

//#region Transparency
/// 👁️ Typed `tRNS` payload — shape depends on `color_type` (§11.3.3). Grayscale/RGB store
/// full 16-bit samples per spec regardless of `bit_depth`; a whole-value weak entity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "colorType", rename_all = "camelCase")]
pub enum PngTransparency {
    Indexed { alpha: Vec<u8> },
    Grayscale { gray: u16 },
    Rgb { r: u16, g: u16, b: u16 },
}
//#endregion Transparency

//#region Ancillary
/// 📐️ `cHRM` — CIE xy chromaticity coordinates, each an int of `value * 100000` (§11.3.5.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PngChromaticities {
    pub white_x: u32,
    pub white_y: u32,
    pub red_x: u32,
    pub red_y: u32,
    pub green_x: u32,
    pub green_y: u32,
    pub blue_x: u32,
    pub blue_y: u32,
}

/// 🖌️ `sRGB` rendering intent (§11.3.5.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PngSrgbIntent {
    #[default]
    Perceptual,
    RelativeColorimetric,
    Saturation,
    AbsoluteColorimetric,
}

impl PngSrgbIntent {
    pub fn from_u8(v: u8) -> Result<Self, String> {
        match v {
            0 => Ok(PngSrgbIntent::Perceptual),
            1 => Ok(PngSrgbIntent::RelativeColorimetric),
            2 => Ok(PngSrgbIntent::Saturation),
            3 => Ok(PngSrgbIntent::AbsoluteColorimetric),
            _ => Err(format!("png sRGB: unsupported rendering intent {v}")),
        }
    }
    pub fn to_u8(self) -> u8 {
        match self {
            PngSrgbIntent::Perceptual => 0,
            PngSrgbIntent::RelativeColorimetric => 1,
            PngSrgbIntent::Saturation => 2,
            PngSrgbIntent::AbsoluteColorimetric => 3,
        }
    }
}

/// 📏️ `pHYs` — pixel-per-unit density (§11.3.5.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PngPhysicalDims {
    pub ppu_x: u32,
    pub ppu_y: u32,
    pub unit_is_meter: bool,
}

/// 🕰️ `tIME` — last modification time, UTC (§11.3.6.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PngTimestamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// 🖼️ `bKGD` — default background color; shape depends on `color_type` (§11.3.5.1).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "colorType", rename_all = "camelCase")]
pub enum PngBackground {
    Grayscale { gray: u16 },
    Rgb { r: u16, g: u16, b: u16 },
    Indexed { index: u8 },
}
//#endregion Ancillary

//#region Text
/// 🔤️ Which of the three PNG text chunk types (§11.3.4) a [`PngTextChunk`] came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PngTextKind {
    #[default]
    Text,
    ZText,
    IText,
}

/// 💬️ One `tEXt`/`zTXt`/`iTXt` chunk. **Key-kind choice**: index-keyed, NOT name(keyword)-keyed
/// — PNG explicitly permits duplicate keywords (§11.3.4.2 "there is no requirement that
/// keywords be unique"), so keyword identity is unsound as a diff key; position is the only
/// safe stable-enough identity within one decode. `language_tag`/`translated_keyword` are
/// iTXt-only and stay empty strings for `Text`/`ZText` kinds (documented normalization).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PngTextChunk {
    pub keyword: String,
    pub value: String,
    #[serde(default)]
    pub compressed: bool,
    #[serde(default)]
    pub kind: PngTextKind,
    #[serde(default)]
    pub language_tag: String,
    #[serde(default)]
    pub translated_keyword: String,
}
//#endregion Text

//#region UnknownChunks
/// 🗃️ A chunk the codec doesn't specifically model, retained VERBATIM (typed raw-retention —
/// "nothing real on disk silently dropped").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PngChunk {
    pub kind: [u8; 4],
    pub data: Vec<u8>,
}
//#endregion UnknownChunks

//#region ChunkOrder
/// 🧭️ One slot in the file's real chunk sequence. `Idat` coalesces every physical IDAT chunk
/// of the source file into one logical position (documented normalization — the codec
/// canonicalizes to decoded `pixels`, so exact IDAT split points aren't retained). `Text`/
/// `Unknown` carry the index into `text_chunks`/`unknown_chunks` that occupies this position;
/// mutations that insert/remove those collections renumber the markers referencing shifted
/// positions (see `schema::mutations`). This is what makes chunk order genuinely diffable as
/// its own index-keyed collection instead of merely implied by insertion order elsewhere.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "chunk", rename_all = "camelCase")]
pub enum PngChunkMarker {
    Ihdr,
    Plte,
    Trns,
    Gama,
    Chrm,
    Srgb,
    Phys,
    Time,
    Bkgd,
    Idat,
    Iend,
    Text { index: usize },
    Unknown { index: usize },
}
//#endregion ChunkOrder

//#region Snapshot
/// 🧬️ Complete `stdio.png` 1.2 semantic snapshot. `schema` is an identity field, never diffed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png")]
pub struct PngSnapshot {
    #[state(persistent)]
    pub schema: String,
    // IHDR (§11.2.2) — compression method / filter method are always 0, validated on decode,
    // never modeled as mutable fields.
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    pub bit_depth: u8,
    #[state(persistent)]
    pub color_type: PngColorType,
    #[state(persistent)]
    pub interlace: bool,
    // PLTE (§11.2.3) — index-keyed collection, optional.
    #[state(persistent)]
    #[serde(default)]
    pub plte: Option<Vec<PngRgb>>,
    // tRNS (§11.3.3).
    #[state(persistent)]
    #[serde(default)]
    pub trns: Option<PngTransparency>,
    // Typed ancillary set (§11.3.5-11.3.6).
    #[state(persistent)]
    #[serde(default)]
    pub gama: Option<u32>,
    #[state(persistent)]
    #[serde(default)]
    pub chrm: Option<PngChromaticities>,
    #[state(persistent)]
    #[serde(default)]
    pub srgb: Option<PngSrgbIntent>,
    #[state(persistent)]
    #[serde(default)]
    pub phys: Option<PngPhysicalDims>,
    #[state(persistent)]
    #[serde(default)]
    pub time: Option<PngTimestamp>,
    #[state(persistent)]
    #[serde(default)]
    pub bkgd: Option<PngBackground>,
    // tEXt/zTXt/iTXt (§11.3.4) — index-keyed, see `PngTextChunk` doc for why.
    #[state(persistent)]
    #[serde(default)]
    pub text_chunks: Vec<PngTextChunk>,
    // Decoded raster payload — legitimate `Vec<u8>` exception (the format's payload IS
    // pixels): always canonical 8-bit-per-channel RGBA, width*height*4 bytes, row-major,
    // top-to-bottom, non-interlaced regardless of the source file's own encoding (see the
    // engine's `EncodeScopeNote` — decode fully supports every color type/bit depth/Adam7,
    // encode always canonicalizes).
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
    // Chunk order + raw retention — see `PngChunkMarker`/`PngChunk` docs.
    #[state(persistent)]
    #[serde(default)]
    pub chunk_order: Vec<PngChunkMarker>,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_chunks: Vec<PngChunk>,
}

impl Default for PngSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_PNG_DOCUMENT_SCHEMA.into(),
            width: 0,
            height: 0,
            bit_depth: 8,
            color_type: PngColorType::Rgba,
            interlace: false,
            plte: None,
            trns: None,
            gama: None,
            chrm: None,
            srgb: None,
            phys: None,
            time: None,
            bkgd: None,
            text_chunks: Vec::new(),
            pixels: Vec::new(),
            chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Idat, PngChunkMarker::Iend],
            unknown_chunks: Vec::new(),
        }
    }
}
//#endregion Snapshot

//#region HandcraftedArtifactCodecs
impl store::ArtifactDsl for PngSnapshot {
    const EXTENSION: &'static str = "png";
    fn envelope_id() -> &'static str { "stdio.png" }

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
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?;
            bytes.push(byte);
            i += 2;
        }
        crate::artifacts::png::engine::decode_png(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::png::engine::encode_png(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PngSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::png::engine::encode_png(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
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
        crate::artifacts::png::engine::decode_png(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion HandcraftedArtifactCodecs
